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
    version = "1.8.1";
    src = builtins.fetchurl "https://github.com/fosrl/newt/releases/download/1.8.1/newt_linux_amd64";
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
        allowedTCPPorts = [ 22 8080 5000 51000 ];
        allowedUDPPorts = [ 51001 51820 ];
      };
    };
  };

  environment.systemPackages = with pkgs; [
    docker-compose
    vim
    restic
    autorestic
    yq-go
    run-autorestic
    uncloud
    custom-newt
  ];

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
        systemd-services:
          <<: *backblaze-standard
          from: /persist/var/lib/systemd/system
          cron: '0 * * * *'
    '';
  };

  users.users = {
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

  systemd.services."uncloud" = {
    enable = true;
    after = [ "network-online.target" ];
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
    "0 5 * * * root journalctl --vacuum-size=128M"
    "*/5 * * * * root ${run-autorestic}/bin/run-autorestic.sh"
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
      "/var/log"
    ];
  };

  # SSH
  services.openssh = {
    enable = true;
    ports = [ 8080 ];
    openFirewall = true;
    allowSFTP = true;
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
