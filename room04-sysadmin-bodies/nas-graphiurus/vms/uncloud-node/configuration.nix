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
  system.stateVersion = "25.11";
  system.autoUpgrade.channel = "https://nixos.org/channels/nixos-25.11-small";

  networking.firewall = {
    enable = false;
    allowedTCPPorts = [ 22 ];
    allowedUDPPorts = [];
  };

  environment.systemPackages = with pkgs; [
    docker-compose
    vim
    iperf
    htop
    restic
    autorestic
    yq-go
    run-autorestic
    uncloud
    iptables
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
        syncthing:
          <<: *backblaze-standard
          from:
            - /persist/var/lib/docker/volumes/syncthing-config
            - /persist/var/lib/docker/volumes/syncthing-data
          cron: '0 * * * *'
        palmr:
          <<: *backblaze-standard
          from:
            - /persist/var/lib/docker/volumes/palmr_data
          cron: '0 * * * *'
        garage-hot:
          <<: *backblaze-standard
          from:
            - /persist/var/lib/docker/volumes/garage-hot-meta
            - /persist/var/lib/docker/volumes/garage-hot-data
          cron: '0 * * * *'
        garage-cold:
          <<: *backblaze-standard
          from:
            - /persist/var/lib/docker/volumes/garage-cold-meta
          cron: '0 * * * *'
        grafana:
          <<: *backblaze-standard
          from:
            - /persist/var/lib/docker/volumes/grafana-data
          cron: '0 * * * *'
        loki:
          <<: *backblaze-standard
          from:
            - /persist/var/lib/docker/volumes/loki-data
          cron: '0 * * * *'
        suwayomi:
          <<: *backblaze-standard
          from:
            - /persist/var/lib/docker/volumes/suwayomi-data
          cron: '0 * * * *'
        yacy:
          <<: *backblaze-standard
          from:
            - /persist/var/lib/docker/volumes/yacy-data
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
    path = [ custom-newt ];
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
      "/etc/adjtime"
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

  fileSystems = {
    "/" = {
      device = "none";
      fsType = "tmpfs";
      options = [ "defaults" "size=512M" "mode=755" ];
      neededForBoot = true;
    };
    "/persist" = {
      device = "/dev/vda1";
      autoResize = true;
      fsType = "ext4";
      neededForBoot = true;
    };
  };

  # SSH
  services.openssh = {
    enable = true;
    ports = [ 22 ];
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

  # Boot
  boot.extraSystemdUnitPaths = [ "/var/lib/systemd/system/" ];
  boot.supportedFilesystems = [ "nfs" ];
  services.rpcbind.enable = true;
  zramSwap.enable = true;

  # Secure settings
  users.mutableUsers = false;
  services.lvm.enable = false;
  security.sudo.enable = false;
  # If I disable it I cannot boot anymore
  nix.enable = true;

  # Networking
  networking = {
    hostName = "nas-uncloud";
    domain = "";
    nameservers = [ "10.0.0.1" ];
    defaultGateway = "10.0.0.1";
    defaultGateway6 = {
      address = "";
      interface = "eth0";
    };
    dhcpcd.enable = true;
  };

  # Remote updates and flakes
  nix.settings.trusted-users = [ "root" "@wheel" ]; # Allow remote updates
  nix.settings.experimental-features =
    [ "nix-command" "flakes" ]; # Enable flakes

  # Promox
  services.qemuGuest.enable =
    lib.mkDefault true; # Enable QEMU Guest for Proxmox

  # Hardware configuration
  imports = [ (modulesPath + "/profiles/qemu-guest.nix") "${impermanence}/nixos.nix" ];
  boot.loader.grub.device = "/dev/vda";
  boot.loader.grub.storePath = "/persist/nix/store";
  boot.tmp.cleanOnBoot = true;
  boot.growPartition = lib.mkDefault true;
  boot.initrd.availableKernelModules =
    [ "ata_piix" "uhci_hcd" "xen_blkfront" "vmw_pvscsi" ];
  boot.initrd.kernelModules = [ "nvme" ];
  boot.kernelPackages = pkgs.linuxPackages_latest;
}
