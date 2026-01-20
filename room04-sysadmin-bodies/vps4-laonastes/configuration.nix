{ modulesPath, pkgs, lib, ... }:
let
  impermanence = builtins.fetchTarball
    "https://github.com/nix-community/impermanence/archive/master.tar.gz";

  run-autorestic = pkgs.writeShellScriptBin "run-autorestic.sh" ''
    autorestic_conf="$(${pkgs.yq-go}/bin/yq eval-all '. as $item ireduce ({}; . * $item)' /etc/autorestic.yml /etc/autorestic-backends.yml)"
    echo "$autorestic_conf" > /tmp/.autorestic.yml
    ${pkgs.autorestic}/bin/autorestic -c /tmp/.autorestic.yml --ci cron > /var/log/autorestic.log 2>&1
    rm -f /tmp/.autorestic.yml
  '';
in {
  imports =
    [ (modulesPath + "/profiles/qemu-guest.nix") "${impermanence}/nixos.nix" ];

  system.stateVersion = "25.11";
  system.autoUpgrade.channel = "https://nixos.org/channels/nixos-25.11-small";

  networking = {
    firewall = {
      enable = false;
      interfaces = {
        "ens6" = {
          allowedTCPPorts = [ 22 80 443 8080 ];
          allowedUDPPorts = [ 53 443 51820 ];
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

  services.logrotate = {
    enable = true;
    settings = {
      "/var/log/logs/traefik/access.log" = {
        frequency = "hourly";
        size = "1M"; # Rotate when size reach 1MB
        rotate = 1; # Keep only last version for vector
        missingok = true; # Ignore if file is missing
        postrotate = ''
          ${pkgs.systemd}/bin/systemctl start traefik-logrotate
        '';
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
          endpoint = "http://loki:3100";
          encoding = { codec = "json"; };

          labels.source = "laonastes_journald";
        };
        loki_outer_traefik = {
          type = "loki";
          inputs = [ "outer_traefik" ];
          endpoint = "http://loki:3100";
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

  systemd.services."podman-compose@" = {
    enable = false;
    path = [ pkgs.podman ];
    serviceConfig = {
      Type = "simple";
      EnvironmentFile = "%h/.config/containers/compose/projects/%i.env";
      ExecStartPre = [
        "-${pkgs.podman-compose}/bin/podman-compose up --no-start"
        "${pkgs.podman}/bin/podman pod start pod_%i"
      ];
      ExecStart = "${pkgs.podman-compose} wait";
      ExecStop = "${pkgs.podman}/bin/podman pod stop pod_%i";
    };
    wantedBy = [ "default.target" ];
  };

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
    vim
    iperf
    htop
    restic
    autorestic
    yq-go
    run-autorestic
  ];

  services.cron.systemCronJobs = [
    "0 5 * * * root journalctl --vacuum-size=128M"
    "*/5 * * * * root ${run-autorestic}/bin/run-autorestic.sh"
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
