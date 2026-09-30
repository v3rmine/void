{ pkgs, config, lib, ... }: 
let 
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
    #iptables -I INPUT 1 -m set --match-set scanners-ipv4 src -j DROP
    #iptables -I FORWARD 1 -m set --match-set scanners-ipv4 src -j DROP
    #ip6tables -I INPUT 1 -m set --match-set scanners-ipv6 src -j DROP
    #ip6tables -I FORWARD 1 -m set --match-set scanners-ipv6 src -j DROP
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
      firewall-block-scrapers-rules
      ipset
      iptables
    ];

    services.cron.systemCronJobs = [
      "*/15 * * * * root ${pkgs.ipset}/bin/ipset save > /etc/iptables/ipsets"
    ];

    systemd.services."ipset-persistent" = {
      enable = true;
      before = [ "network.target" "firewall.service" ];
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
      requiredBy = [ "firewall.service" ];
    };

    systemd.services."ipset-filters-setup" = {
      enable = false;
      wants = [ "firewall.service" "uncloud.service" "docker.service" "tailscaled.service" ];
      after = [ "docker.service" "tailscaled.service" ];
      path = [ pkgs.iptables ];
      serviceConfig = {
        Type = "oneshot";
        ExecStart = "${firewall-block-scrapers-rules}/bin/add-firewall-block-scrapers-rules.sh";
      };
      wantedBy = [ "multi-user.target" ];
    };
  };
}
