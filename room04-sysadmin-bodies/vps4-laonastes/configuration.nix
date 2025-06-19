{ modulesPath, pkgs, lib, ... }:
let
  impermanence = builtins.fetchTarball
    "https://github.com/nix-community/impermanence/archive/master.tar.gz";
in {
  imports =
    [ (modulesPath + "/profiles/qemu-guest.nix") "${impermanence}/nixos.nix" ];

  system.stateVersion = "24.11";
  system.autoUpgrade.channel = "https://nixos.org/channels/nixos-24.11-small";

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

  services.logrotate = {
    enable = true;
    settings = {
      "/root/logs/anubis-traefik/access.log" = {
        frequency = "hourly";
        size = "1M"; # Rotate when size reach 1MB
        rotate = 1; # Keep only last version for vector
        missingok = true; # Ignore if file is missing
        postrotate = ''
          ${pkgs.podman}/bin/podman kill -s USR1 anubis-traefik
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
          include = [ "/root/logs/anubis-traefik/access.log" ];
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
  systemd.services.vector.serviceConfig = {
    AmbientCapabilities =
      lib.mkForce "CAP_NET_BIND_SERVICE CAP_DAC_READ_SEARCH";
    CapabilityBoundingSet = "CAP_DAC_READ_SEARCH";
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
    prometheus-node-exporter
    htop
    iotop-c
    iftop
  ];

  systemd.services.prometheus-node-exporter = {
    enable = true;
    unitConfig = { Type = "simple"; };
    serviceConfig = {
      ExecStart = "${pkgs.prometheus-node-exporter}/bin/node_exporter";
    };
    wantedBy = [ "multi-user.target" ];
  };

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
      "/root/logs"
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
      options = [ "defaults" "size=512M" "mode=755" ];
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
