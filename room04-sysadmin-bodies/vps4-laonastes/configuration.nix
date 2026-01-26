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

  fill-blocklists = pkgs.writeShellScriptBin "fill-blocklists.sh" ''
    # Ensure lists exists
    ${pkgs.ipset}/bin/ipset list scanners-ipv4 >/dev/null 2>&1 \
      || ${pkgs.ipset}/bin/ipset create scanners-ipv4 hash:ip hashsize 4096 counters family inet
    ${pkgs.ipset}/bin/ipset list scanners-ipv6 >/dev/null 2>&1 \
      || ${pkgs.ipset}/bin/ipset create scanners-ipv6 hash:ip hashsize 4096 counters family inet6

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

    # IPs that have been rejected more than 10k times by iocaine
    iocaine_rejected_ips=$(journalctl CONTAINER_NAME=pangolin_iocaine_1 -o json -r \
      | yq -p=json '.MESSAGE | from_json| select(."verdict.type" == "accept") | .request.header.x-forwarded-for' \
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
in {
  imports =
    [ (modulesPath + "/profiles/qemu-guest.nix") "${impermanence}/nixos.nix" ];

  system.stateVersion = "25.11";
  system.autoUpgrade.channel = "https://nixos.org/channels/nixos-25.11";

  networking = {
    firewall = {
      enable = true;
      interfaces = {
        "ens6" = {
          allowedTCPPorts = [ 22 80 443 8080 22000 ];
          allowedUDPPorts = [ 53 443 51820 22000 21027 ];
        };
      };
    };
  };

  environment.etc."autorestic.yml" = {
    text = ''
      version: 2

      extras:
        policies: &backup-policy
          keep-daily: 7
          keep-weekly: 52
          keep-yearly: 10
        backblaze-standard: &backblaze-standard
          to:
            - backblaze
          options:
            backup:
              compression: max
              skip-if-unchanged: true
            forget:
              <<: *backup-policy


      locations:
        pangolin:
          <<: *backblaze-standard
          from: /root/pangolin
          cron: '0 * * * *'
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
        maxsize = "5M"; # Rotate when size reach 1MB
        rotate = 3; # Keep the last 5 version for ipset
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
        outer_traefik = {
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

          labels.source = "laonastes_journald";
        };
        loki_outer_traefik = {
          type = "loki";
          inputs = [ "outer_traefik" ];
          endpoint = "http://nas-uncloud:3100";
          encoding = { codec = "json"; };

          labels.source = "laonastes_outer_traefik";
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
      ExecStart = "${pkgs.podman}/bin/podman kill -s USR1 traefik";
    };
  };

  systemd.services."podman-restart".enable = true;

  virtualisation = {
    podman = {
      enable = true;
      # Create a `docker` alias for podman, to use it as a drop-in replacement
      dockerCompat = true;
      dockerSocket = { enable = true; };
      # Required for containers under podman-compose to be able to talk to each other.
      defaultNetwork.settings.dns_enabled = true;
    };
  };

  services.tailscale = {
    enable = true;
    extraUpFlags = [ "--ssh" ];
  };

  environment.systemPackages = with pkgs; [
    podman-compose
    git
    vim
    htop
    restic
    autorestic
    run-autorestic
    yq-go
    fill-blocklists
    flush-blocklists
    iptables
  ];

  services.cron.systemCronJobs = [
    "0 * * * * root journalctl --vacuum-size=128M"
    "*/5 * * * * root ${run-autorestic}/bin/run-autorestic.sh"
    "*/15 * * * * root ${fill-blocklists}/bin/fill-blocklists.sh"
    "0 0 * * MON root ${flush-blocklists}/bin/flush-blocklists.sh"
  ];

  # Networking and SSH
  networking.hostName = "laonastes";
  networking.domain = "";

  services.openssh = {
    enable = true;
    ports = [ 8080 ];
    openFirewall = true;
    allowSFTP = false;
    settings = {
      PermitRootLogin = "yes";
      PasswordAuthentication = false;
      KbdInteractiveAuthentication = false;
    };
    extraConfig = ''
      AllowTcpForwarding yes
      X11Forwarding no
      AllowAgentForwarding no
      AllowStreamLocalForwarding no
      AuthenticationMethods publickey
    '';
  };
  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIHt8EvWzOBZxA7JEQCnknW+hjEi8Id6dfMtb1ONw1xbw astrid@astrid-lux"
  ];

  # Impermanence
  environment.persistence."/persist" = {
    files = [
      "/etc/machine-id"
      "/etc/adjtime"
      "/etc/ssh/ssh_host_ed25519_key"
      "/etc/ssh/ssh_host_ed25519_key.pub"
      "/etc/ssh/ssh_host_rsa_key"
      "/etc/ssh/ssh_host_rsa_key.pub"
      "/etc/autorestic-backends.yml"
      "/etc/iptables/ipsets"
    ];
    directories = [
      "/nix"
      "/var/lib/nixos"
      "/var/lib/tailscale"
      "/tmp"
      "/var/tmp"
      "/etc/containers"
      "/var/lib/containers/storage"
      "/var/lib/swap"
      "/root/pangolin"
      "/var/log"
    ];
  };

  # Storage optimization
  # https://wiki.nixos.org/wiki/Storage_optimization
  services.udev.enable = false;
  services.lvm.enable = false;
  security.sudo.enable = false;
  documentation.enable = false;
  documentation.doc.enable = false;
  documentation.info.enable = false;
  documentation.man.enable = false;
  nix.enable = false;

  # System Config
  boot.kernel.sysctl = { "fs.file-max" = 65536; };
  boot.tmp.cleanOnBoot = true;
  zramSwap.enable = false;
  swapDevices = [{
    device = "/var/lib/swap/swapfile";
    size = 1024;
  }];
  boot.kernel.sysctl = { "vm.swappiness" = 130; };

  powerManagement.cpuFreqGovernor = "performance";
  users.mutableUsers = false;

  # Hardware config
  boot.loader.grub = {
    efiSupport = true;
    efiInstallAsRemovable = true;
    device = "nodev";
  };
  boot.initrd.availableKernelModules =
    [ "ata_piix" "uhci_hcd" "xen_blkfront" "vmw_pvscsi" ];
  boot.initrd.kernelModules = [ "nvme" ];
  fileSystems = {
    "/" = {
      device = "none";
      fsType = "tmpfs";
      options = [ "defaults" "size=256M" "mode=755" ];
      neededForBoot = true;
    };
    "/persist" = {
      device = "/dev/vda1";
      fsType = "ext4";
      neededForBoot = true;
    };
    "/boot" = {
      device = "/dev/disk/by-uuid/53CC-ADDA";
      fsType = "vfat";
      neededForBoot = true;
    };
  };
}
