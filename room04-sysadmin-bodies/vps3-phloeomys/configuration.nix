{ modulesPath, pkgs, lib, ... }:
let
  impermanence = builtins.fetchTarball
    "https://github.com/nix-community/impermanence/archive/master.tar.gz";

  run-autorestic = pkgs.writeShellScriptBin "run-autorestic.sh" ''
    # Merge location and backend autorestic confs
    autorestic_conf="$(${pkgs.yq-go}/bin/yq eval-all '. as $item ireduce ({}; . * $item)' /etc/autorestic.yml /etc/autorestic-backends.yml)"
    echo "$autorestic_conf" > /tmp/.autorestic.yml

    # Run the cron task of autorestic
    ${pkgs.autorestic}/bin/autorestic -c /tmp/.autorestic.yml --ci cron > /var/log/autorestic.log 2>&1
    rm -f /tmp/.autorestic.yml
  '';

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

  record-rss-readers = pkgs.writeShellScriptBin "record-rss-readers.sh" ''
      for file in /var/log/logs/traefik/* /var/log/logs/astriiid-fr-rss-readers.log; do
          grep '"RequestHost":"astriiid.fr"' $file \
          | grep -E '"RequestPath":"[^"]+(rss|atom)\.xml"';
      done \
      | yq -p=json '[.request_User-Agent, .ClientHost, .time]' -o=csv --csv-separator='|' \
      | sort \
      | uniq \
      > /var/log/logs/astriiid-fr-rss-readers.log
  '';

  uncloud = pkgs.stdenv.mkDerivation rec {
    pname = "uncloud";
    version = "0.16.0";
    src = builtins.fetchTarball "https://github.com/psviderski/uncloud/releases/download/v${version}/uncloudd_linux_amd64.tar.gz";

    installPhase = ''
      runHook preInstall
      mkdir -p $out/bin
      cp uncloudd $out/bin
      runHook postInstall
    '';
  };

  corrosion = pkgs.stdenv.mkDerivation rec {
    pname = "corrosion";
    version = "0.2.2";
    src = builtins.fetchTarball "https://github.com/psviderski/corrosion/releases/download/v${version}/corrosion-x86_64-unknown-linux-gnu.tar.gz";

    nativeBuildInputs = with pkgs; [ autoPatchelfHook ];
    buildInputs = with pkgs; [ glibc gcc-unwrapped ];

    installPhase = ''
      runHook preInstall
      mkdir -p $out/bin
      cp corrosion $out/bin/uncloud-corrosion
      runHook postInstall
    '';
  };

  custom-newt = pkgs.stdenv.mkDerivation rec {
    pname = "newt";
    version = "1.9.0";
    src = builtins.fetchurl "https://github.com/fosrl/newt/releases/download/1.9.0/newt_linux_amd64";
    dontUnpack = true;

    installPhase = ''
      runHook preInstall
      mkdir -p $out/bin
      cp $src $out/bin/newt
      chmod +x $out/bin/newt
      runHook postInstall
    '';
  };
in {
  imports =
    [ (modulesPath + "/profiles/qemu-guest.nix") "${impermanence}/nixos.nix" ];

  system.stateVersion = "25.11";
  system.autoUpgrade.channel = "https://nixos.org/channels/nixos-25.11";

  networking.firewall = {
    enable = true;
    interfaces = {
      "eth0" = {
        allowedTCPPorts = [ 22 80 443 5000 6697 8080 22000 51000 ];
        allowedUDPPorts = [ 53 443 21027 21820 22000 51001 51820 51830 ];
      };
    };
  };

  environment.systemPackages = with pkgs; [
    docker-compose
    git
    vim
    htop
    restic
    autorestic
    run-autorestic
    yq-go
    uncloud
    custom-newt
    fill-blocklists-fast
    fill-blocklists-slow
    flush-blocklists
    iptables
    ipset
    cifs-utils
    record-rss-readers
  ];

  environment.etc."autorestic.yml" = {
    text = ''
      version: 2

      extras:
        policies: &backup-policy
          keep-daily: 7
          keep-weekly: 52
          keep-yearly: 10
        standard: &standard
          to:
            - backblaze
            - hetzner
          options:
            backup:
              compression: max
              skip-if-unchanged: true
            forget:
              <<: *backup-policy
        backblaze: &backblaze
          to:
            - backblaze
          options:
            backup:
              compression: max
              skip-if-unchanged: true
            forget:
              <<: *backup-policy

      locations:
        systemd-services:
          <<: *standard
          from: /persist/var/lib/systemd/system
          cron: '0 * * * *'
        pangolin:
          <<: *standard
          from: /root/pangolin
          cron: '0 * * * *'
        filestash:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/filestash-config
          cron: '0 * * * *'
        goatcounter:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/goatcounter-data
          cron: '0 * * * *'
        miniflux:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/miniflux-data
          cron: '0 * * * *'
        shaarli:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/shaarli-data
          cron: '0 * * * *'
        linkding:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/linkding-data
          cron: '0 * * * *'
        otterwiki:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/otterwiki-data
          cron: '0 * * * *'
        continuwuity:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/continuwuity-data
          cron: '0 * * * *'
        sharkey:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/sharkey-files
            - /persist/var/lib/docker/volumes/sharkey-db
            - /persist/var/lib/docker/volumes/sharkey-meilisearch
          cron: '0 * * * *'
        drive:
          <<: *backblaze
          from:
            - /persist/mnt/drive
          cron: '0 2 * * *'
          options:
            backup:
              exclude-file: /persist/mnt/drive/.backup-ignore
    '';
  };

  networking.firewall.extraCommands = ''
    iptables -A INPUT -m set --match-set scanners-ipv4 src -j DROP
    iptables -A FORWARD -m set --match-set scanners-ipv4 src -j DROP
    ip6tables -A INPUT -m set --match-set scanners-ipv6 src -j DROP
    ip6tables -A FORWARD -m set --match-set scanners-ipv6 src -j DROP
  '';

  services.logrotate = {
    enable = true;
    settings = {
      "/var/log/logs/traefik/access.log" = {
        frequency = "daily";
        maxsize = "50M"; # Rotate when size reach 50MB
        rotate = 5; # Keep the last 5 version for ipset
        missingok = true; # Ignore if file is missing
        postrotate = ''
          ${pkgs.systemd}/bin/systemctl start traefik-logrotate
        '';
        nocompress = true; # Do not compress for postprocess by ipset
        notifempty = true; # Do not rotate if empty
      };
    };
  };

  services.vector = {
    enable = true;
    journaldAccess = true;
    settings = {
      api.enabled = true;

      sources = {
        journald.type = "journald";
        traefik = {
          type = "file";
          include = [ "/var/log/logs/traefik/access.log" ];
          fingerprint.strategy = "device_and_inode";
          rotate_wait_secs = 30;
        };
      };

      sinks = {
        loki_journald = {
          type = "loki";
          inputs = [ "journald" ];
          endpoint = "http://nas-uncloud:3100";
          encoding = { codec = "json"; };

          labels.source = "phloeomys_journald";
        };
        loki_traefik = {
          type = "loki";
          inputs = [ "traefik" ];
          endpoint = "http://nas-uncloud:3100";
          encoding = { codec = "json"; };

          labels.source = "phloeomys_traefik";
        };
      };
    };
  };
  systemd.services."vector".serviceConfig = {
    AmbientCapabilities =
      lib.mkForce "CAP_NET_BIND_SERVICE CAP_DAC_READ_SEARCH";
    CapabilityBoundingSet = "CAP_DAC_READ_SEARCH";
  };
  systemd.services."logrotate".serviceConfig = {
    PrivateNetwork = lib.mkForce false;
    RestrictAddressFamilies = lib.mkForce "AF_UNIX";
    BindPaths = "/run/systemd/private /run/dbus/system_bus_socket";
  };
  systemd.services."traefik-logrotate" = {
    serviceConfig = {
      Type = "oneshot";
      ExecStart = "${pkgs.docker}/bin/docker kill -s USR1 traefik";
    };
  };

  users.users = {
    root = {
      hashedPasswordFile = "/persist/etc/shadowRoot";
    };
    uncloud = {
      isSystemUser = true;
      createHome = false;
      home = "/var/empty";
      shell = pkgs.shadow;
      group = "uncloud";
    };
  };
  users.groups = {
    uncloud = {};
  };

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

  systemd.services."uncloud" = {
    enable = true;
    after = [ "network-online.target" "ipset-persistent.service" "firewall.service" ];
    wants = [ "network-online.target" ];
    path = [ pkgs.iptables ];
    serviceConfig = {
      Type = "notify";
      ExecStart = "${uncloud}/bin/uncloudd";
      TimeoutStartSec = 15;
      Restart = "always";
      RestartSec = 2;
      # Hardening options.
      NoNewPrivileges = true;
      ProtectSystem = "full";
      ProtectControlGroups = true;
      ProtectHome = "read-only";
      ProtectKernelTunables = true;
      PrivateTmp = true;
      RestrictAddressFamilies = "AF_INET AF_INET6 AF_UNIX AF_NETLINK";
      RestrictNamespaces = true;
    };
    wantedBy = [ "multi-user.target" ];
  };

  systemd.services."uncloud-corrosion" = {
    enable = true;
    partOf = [ "uncloud.service" ];
    serviceConfig = {
      Type = "simple";
      ExecStart = "${corrosion}/bin/uncloud-corrosion agent -c /var/lib/uncloud/corrosion/config.toml";
      ExecReload = "${corrosion}/bin/uncloud-corrosion reload -c /var/lib/uncloud/corrosion/config.toml";
      Restart = "always";
      RestartSec = 2;
      User = "uncloud";
      Group = "uncloud";
      # Hardening options.
      NoNewPrivileges = true;
      ProtectSystem = "full";
      ProtectControlGroups = true;
      ProtectHome = true;
      ProtectKernelTunables = true;
      PrivateTmp = true;
      RestrictAddressFamilies = "AF_INET AF_INET6 AF_UNIX";
    };
    wantedBy = [ "multi-user.target" ];
  };

  systemd.services."newt" = {
    enable = true;
    after = [ "network-online.target" ];
    wants = [ "network-online.target" ];
    serviceConfig = {
      Type = "simple";
      Restart = "always";
      RestartSec = 2;
      # Hardening options.
      NoNewPrivileges = true;
      PrivateTmp = true;
      ProtectControlGroups = true;
      ProtectHome = true;
      ProtectKernelTunables = true;
      ProtectSystem = "full";
      RestrictAddressFamilies = "AF_INET AF_INET6 AF_UNIX AF_NETLINK";
      RestrictNamespaces = true;
    };
    wantedBy = [ "multi-user.target" ];
  };


  # System
  services.cron.systemCronJobs = [
    "0 * * * * root journalctl --vacuum-size=512M"
    "*/5 * * * * root ${run-autorestic}/bin/run-autorestic.sh"
    "*/5 * * * * root ${fill-blocklists-fast}/bin/fill-blocklists-fast.sh"
    "3-59/15 * * * * root ${fill-blocklists-slow}/bin/fill-blocklists-slow.sh"
    "0 0 * * MON root ${flush-blocklists}/bin/flush-blocklists.sh"
    "*/15 * * * * root ${record-rss-readers}/bin/record-rss-readers.sh"
  ];

  virtualisation = {
    containerd.enable = true;
    docker = {
      enable = true;
      daemon.settings = {
        features = {
          containerd-snapshotter = true;
        };
        storage-driver = "overlayfs";
        live-restore = true;
      };
    };
  };

  services.tailscale = {
    enable = true;
    extraUpFlags = [ "--ssh" ];
  };

  time.timeZone = "Europe/Paris";

  # Impermanence
  environment.persistence."/persist" = {
    files = [
      "/etc/machine-id"
      "/etc/ssh/ssh_host_ed25519_key"
      "/etc/ssh/ssh_host_ed25519_key.pub"
      "/etc/ssh/ssh_host_rsa_key"
      "/etc/ssh/ssh_host_rsa_key.pub"
      "/etc/autorestic-backends.yml"
      "/etc/iptables/ipsets"
    ];
    directories = [
      "/boot"
      "/nix"
      "/tmp"
      "/var/tmp"
      "/var/lib/nixos"
      "/var/lib/tailscale"
      "/var/lib/docker"
      "/var/lib/containerd"
      "/var/lib/uncloud"
      "/var/lib/systemd/system"
      "/root/pangolin"
      "/root/.ssh"
      "/var/log"
    ];
  };

  # SSH
  services.openssh = {
    enable = true;
    ports = [ 8080 ];
    openFirewall = true;
    allowSFTP = false;
    settings = {
      PermitRootLogin = "without-password";
      PasswordAuthentication = false;
      KbdInteractiveAuthentication = false;
    };
    extraConfig = ''
      AllowTcpForwarding yes
      X11Forwarding no
      AllowAgentForwarding no
      AllowStreamLocalForwarding yes
      AuthenticationMethods publickey
      ClientAliveCountMax 2
      LogLevel verbose
      MaxAuthTries 3
      MaxSessions 4
      TCPKeepAlive no
    '';
  };
  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIHt8EvWzOBZxA7JEQCnknW+hjEi8Id6dfMtb1ONw1xbw astrid@astrid-lux"
  ];

  # Systemd
  boot.extraSystemdUnitPaths = [ "/var/lib/systemd/system/" ];

  # Secure settings
  users.mutableUsers = false;
  services.lvm.enable = false;
  security.sudo.enable = false;
  nix.enable = true;

  # Hostname
  networking.hostName = "phloeomys";
  networking.domain = "";

  # Remote updates and flakes
  nix.settings.trusted-users = [ "root" "@wheel" ]; # Allow remote updates
  nix.settings.experimental-features =
    [ "nix-command" "flakes" ]; # Enable flakes

  # VM guest additions
  services.qemuGuest.enable = true;

  # Hardware configuration
  boot.kernelPackages = pkgs.linuxPackages_latest;
  boot.tmp.cleanOnBoot = true;
  zramSwap.enable = true;
  boot.loader.grub.device = "/dev/sda";
  boot.loader.grub.storePath = "/persist/nix/store";
  boot.initrd.availableKernelModules = [ "ata_piix" "uhci_hcd" "xen_blkfront" "vmw_pvscsi" ];
  boot.initrd.kernelModules = [ "nvme" ];
  fileSystems = {
    "/" = {
      device = "none";
      fsType = "tmpfs";
      options = [ "defaults" "size=512M" "mode=755" ];
      neededForBoot = true;
    };
    "/persist" = {
      device = "/dev/sda1";
      fsType = "ext4";
      neededForBoot = true;
    };
    "/persist/mnt/drive" = {
      device = "//u264248-sub2.your-storagebox.de/u264248-sub2";
      fsType = "cifs";
      options = let
        # this line prevents hanging on network split
        automount_opts = "x-systemd.automount,noauto,x-systemd.idle-timeout=60,x-systemd.device-timeout=5s,x-systemd.mount-timeout=5s";
      in ["ro,${automount_opts},credentials=/persist/etc/cifs-drive-credentials"];
      neededForBoot = false;
    };
  };

  # Networking
  networking = {
    nameservers = [ "8.8.8.8" ];
    defaultGateway = "172.31.1.1";
    defaultGateway6 = {
      address = "fe80::1";
      interface = "eth0";
    };
    dhcpcd.enable = false;
    usePredictableInterfaceNames = lib.mkForce false;
    interfaces = {
      eth0 = {
        ipv4.addresses = [
          { address="46.225.11.44"; prefixLength=32; }
        ];
        ipv6.addresses = [
          { address="2a01:4f8:1c19:cc7a::1"; prefixLength=64; }
          { address="fe80::9000:7ff:fe08:d36a"; prefixLength=64; }
        ];
        ipv4.routes = [ { address = "172.31.1.1"; prefixLength = 32; } ];
        ipv6.routes = [ { address = "fe80::1"; prefixLength = 128; } ];
      };
    };
  };

  services.udev.extraRules = ''
    ATTR{address}=="92:00:07:08:d3:6a", NAME="eth0"
  '';
}
