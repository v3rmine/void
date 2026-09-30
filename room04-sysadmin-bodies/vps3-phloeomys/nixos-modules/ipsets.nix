{ pkgs, config, lib, ... }: 
let 
  fill-blocklists-fast = pkgs.writeShellScriptBin "fill-blocklists-fast.sh" ''
    # Ensure lists exists
    ${pkgs.ipset}/bin/ipset list scanners-ipv4 >/dev/null 2>&1 \
    || ${pkgs.ipset}/bin/ipset create scanners-ipv4 hash:ip hashsize 4096 counters family inet
    ${pkgs.ipset}/bin/ipset list scanners-ipv6 >/dev/null 2>&1 \
    || ${pkgs.ipset}/bin/ipset create scanners-ipv6 hash:ip hashsize 4096 counters family inet6

    # I don't host any wordpress so ban ip that scans for it
    wp_scanners_ips=$(for file in /var/log/logs/traefik/*; do
      grep -E 'RequestPath":"/(wp-admin|wp-content|wp-includes)[^"]+"' $file \
      | awk 'match($0, /"ClientHost"[[:space:]]*:[[:space:]]*"([^"]+)"/, a) { print a[1] }';
    done \
      | sort \
      | uniq)

    echo "$wp_scanners_ips" | grep -F '.' | xargs --no-run-if-empty -n1 ${pkgs.ipset}/bin/ipset add scanners-ipv4 -exist
    echo "$wp_scanners_ips" | grep -F ':' | xargs --no-run-if-empty -n1 ${pkgs.ipset}/bin/ipset add scanners-ipv6 -exist

    # IPs that make >100 http request get banned:
    http_requester_ips=$(for file in /var/log/logs/traefik/*; do
      grep 'RequestScheme":"http"' $file \
      | awk 'match($0, /"ClientHost"[[:space:]]*:[[:space:]]*"([^"]+)"/, a) { print a[1] }';
    done \
      | sort \
      | uniq -c \
      | awk '{if ($1 >= 100) print $2}')

    echo "$http_requester_ips" | grep -F '.' | xargs --no-run-if-empty -n1 ${pkgs.ipset}/bin/ipset add scanners-ipv4 -exist
    echo "$http_requester_ips" | grep -F ':' | xargs --no-run-if-empty -n1 ${pkgs.ipset}/bin/ipset add scanners-ipv6 -exist

    # Auto ban sshd scanners
    sshd_ips=$(journalctl -u sshd -o json -r \
      | grep -E "(invalid format|invalid user|Closed.*preauth)" \
      | yq -p=json '.MESSAGE' \
      | grep -oE "([0-9]+\.[0-9]+\.[0-9]+\.[0-9]+|[0-9]+(:[0-9]+)+)" \
      | sort \
      | uniq)

    echo "$sshd_ips" | grep -F '.' | xargs --no-run-if-empty -n1 ${pkgs.ipset}/bin/ipset add scanners-ipv4 -exist
    echo "$sshd_ips" | grep -F ':' | xargs --no-run-if-empty -n1 ${pkgs.ipset}/bin/ipset add scanners-ipv6 -exist

    # Make banned IP list persistent
    ${pkgs.ipset}/bin/ipset save > /etc/iptables/ipsets
  '';

  fill-blocklists-slow = pkgs.writeShellScriptBin "fill-blocklists-slow.sh" ''
    # Ensure lists exists
    ${pkgs.ipset}/bin/ipset list scanners-ipv4 >/dev/null 2>&1 \
      || ${pkgs.ipset}/bin/ipset create scanners-ipv4 hash:ip hashsize 4096 counters family inet
    ${pkgs.ipset}/bin/ipset list scanners-ipv6 >/dev/null 2>&1 \
      || ${pkgs.ipset}/bin/ipset create scanners-ipv6 hash:ip hashsize 4096 counters family inet6

    # IPs that have been rejected more than 10k times by iocaine
    iocaine_rejected_ips=$(journalctl CONTAINER_NAME=pangolin-iocaine-1 -o json -r \
      | grep '\\"verdict.type\\":\\"accept\\"' \
      | yq -p=json '.MESSAGE | from_json | select(."verdict.type" == "accept") | .request.header.x-forwarded-for' \
      | grep -v "\---" \
      | sort \
      | uniq -c \
      | awk '{if ($1 >= 10000) print $2}')

    echo "$iocaine_rejected_ips" | grep -F '.' | xargs --no-run-if-empty -n1 ${pkgs.ipset}/bin/ipset add scanners-ipv4 -exist
    echo "$iocaine_rejected_ips" | grep -F ':' | xargs --no-run-if-empty -n1 ${pkgs.ipset}/bin/ipset add scanners-ipv6 -exist

    # Facebook IPs (fuck them changing IPv6s constantly)
    # Claude do the same fucking thing but on ipv4
    iocaine_fuckers_ips=$(journalctl CONTAINER_NAME=pangolin-iocaine-1 -o json -r \
      | grep '\\"verdict.type\\":\\"accept\\"' \
      | grep -E '\\"classification\\":\\"(Facebook|Claude)\\"' \
      | yq -p=json '.MESSAGE | from_json | select(."verdict.type" == "accept" and (."classification" == "Facebook" or ."classification" == "Claude")) | .request.header.x-forwarded-for' \
      | grep -v "\---" \
      | sort \
      | uniq)

    echo "$iocaine_fuckers_ips" | grep -F '.' | xargs --no-run-if-empty -n1 ${pkgs.ipset}/bin/ipset add scanners-ipv4 -exist
    echo "$iocaine_fuckers_ips" | grep -F ':' | xargs --no-run-if-empty -n1 ${pkgs.ipset}/bin/ipset add scanners-ipv6 -exist

    # Make banned IP list persistent
    ${pkgs.ipset}/bin/ipset save > /etc/iptables/ipsets
  '';

  flush-blocklists = pkgs.writeShellScriptBin "flush-blocklists.sh" ''
      # Ensure lists exists
      ${pkgs.ipset}/bin/ipset list scanners-ipv4 >/dev/null 2>&1 \
        || ${pkgs.ipset}/bin/ipset create scanners-ipv4 hash:ip hashsize 4096 counters family inet
      ${pkgs.ipset}/bin/ipset list scanners-ipv6 >/dev/null 2>&1 \
        || ${pkgs.ipset}/bin/ipset create scanners-ipv6 hash:ip hashsize 4096 counters family inet6

      # Flush lists from previous values
      ${pkgs.ipset}/bin/ipset flush scanners-ipv4
      ${pkgs.ipset}/bin/ipset flush scanners-ipv6

      # Make banned IP list persistent
      ${pkgs.ipset}/bin/ipset save > /etc/iptables/ipsets
  '';

  firewall-block-scrapers-rules = pkgs.writeShellScriptBin "add-firewall-block-scrapers-rules.sh" ''
    tmpdir=$(mktemp -d)
    trap exiting exit
    function exiting() { rm -r "$tmpdir"; exit; }
    # Clean iptables of previous rules
    iptables-save | \
      sed -E 's/^.*scanners-ipv4.*//m' | \
      sed -E 's/^.*Alibaba-crawler.*//m' | \
      grep -vE '^$' > "$tmpdir/iptables"
    iptables-restore "$tmpdir/iptables"
    ip6tables-save | \
      sed -E 's/^.*scanners-ipv6.*//m' | \
      grep -vE '^$' > "$tmpdir/ip6tables"
    ip6tables-restore "$tmpdir/ip6tables"
    # Insert new rules
    iptables -I INPUT 1 -m set --match-set scanners-ipv4 src -j DROP
    iptables -I FORWARD 1 -m set --match-set scanners-ipv4 src -j DROP
    ip6tables -I INPUT 1 -m set --match-set scanners-ipv6 src -j DROP
    ip6tables -I FORWARD 1 -m set --match-set scanners-ipv6 src -j DROP
    # Ban Alibaba crawlers subnet
    iptables -I INPUT 1 -m comment --comment Alibaba-crawler -s 8.219.128.0/17 -j DROP
    iptables -I FORWARD 1 -m comment --comment Alibaba-crawler -s 8.219.128.0/17 -j DROP
    iptables -I INPUT 1 -m comment --comment Alibaba-crawler -s 47.79.192.0/19 -j DROP
    iptables -I FORWARD 1 -m comment --comment Alibaba-crawler -s 47.79.192.0/19 -j DROP
  '';
in {
  imports = [];
  options = {
    services.ipset-autoban.enable = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = "ipset autoban rules";
    };
  };

  config = lib.mkIf config.services.ipset-autoban.enable {
    environment.persistence."/persist".files = [
      "/etc/iptables/ipsets"
    ];

    environment.systemPackages = with pkgs; [
      fill-blocklists-fast
      fill-blocklists-slow
      flush-blocklists
      firewall-block-scrapers-rules
      ipset
      iptables
    ];

    services.cron.systemCronJobs = [
      "*/5 * * * * root ${fill-blocklists-fast}/bin/fill-blocklists-fast.sh"
      "3-59/15 * * * * root ${fill-blocklists-slow}/bin/fill-blocklists-slow.sh"
      "0 0 * * MON root ${flush-blocklists}/bin/flush-blocklists.sh"
    ];

    systemd.services."ipset-persistent" = {
      enable = true;
      before = [ "network.target" "firewall.service" "uncloud.service" ];
      unitConfig = {
        ConditionFileNotEmpty = "/etc/iptables/ipsets";
      };
      serviceConfig = {
        Type = "oneshot";
        ExecStart = "${pkgs.ipset}/bin/ipset restore -exist -file /etc/iptables/ipsets";
        # ExecStop = "${pkgs.ipset}/bin/ipset flush";
        # ExecStopPost = "${pkgs.ipset}/bin/ipset destroy";
      };
      wantedBy = [ "multi-user.target" ];
      requiredBy = [ "firewall.service" "uncloud.service" ];
    };

    systemd.services."ipset-filters-setup" = {
      enable = true;
      wants = [ "firewall.service" "uncloud.service" "docker.service" "tailscaled.service" ];
      after = [ "uncloud.service" "docker.service" "tailscaled.service" ];
      path = [ pkgs.iptables ];
      serviceConfig = {
        Type = "oneshot";
        ExecStart = "${firewall-block-scrapers-rules}/bin/add-firewall-block-scrapers-rules.sh";
      };
      wantedBy = [ "multi-user.target" ];
    };
  };
}
