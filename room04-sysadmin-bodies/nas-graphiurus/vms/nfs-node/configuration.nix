{ modulesPath, pkgs, lib, ... }:
let
  get_import_dir =
    dir:
    lib.flatten (
      lib.pipe dir [
        builtins.readDir
        (lib.filterAttrs (name: type: type == "directory" || lib.hasSuffix ".nix" name))
        (lib.mapAttrsToList (
          name: type: if type == "directory" then get_import_dir (dir + ("/" + name)) else dir + ("/" + name)
        ))
      ]
    );
in {
  system.stateVersion = "25.11";
  imports = [] ++ (get_import_dir ./nixos-modules/.);

  networking.firewall = {
    enable = false;
    allowedTCPPorts = [];
    allowedUDPPorts = [];
  };

  environment.systemPackages = with pkgs; [
    mergerfs
    cryptsetup
    smartmontools
    openseachest
    yq-go
  ];

  boot.kernelModules = [ "dm_crypt" ];
  environment.etc.crypttab = {
    mode = "0600";
    text = ''
      # <volume-name> <encrypted-device> [key-file] [options]
      parity1 UUID=e5e2fa22-6b0d-4516-b5da-ffe826a02736 /var/lib/hdd_key luks,discard,tries=1,noauto,nofail
      disk1 UUID=dbb1e30c-44f3-4541-8941-2452a544b93b /var/lib/hdd_key luks,discard,tries=1,noauto,nofail
    '';
  };

  fileSystems."/media/disk1" = {
    fsType = "xfs";
    device = "/dev/mapper/disk1";
    options = [
      "noauto"
      "nofail"
      "x-systemd.automount"
      "x-systemd.idle-timeout=15min"
      "x-systemd.device-timeout=5"
    ];
    neededForBoot = false;
  };

  fileSystems."/media/parity1" = {
    fsType = "xfs";
    device = "/dev/mapper/parity1";
    options = [
      "noauto"
      "nofail"
      "x-systemd.automount"
      "x-systemd.idle-timeout=15min"
      "x-systemd.device-timeout=5"
    ];
    neededForBoot = false;
  };

  fileSystems."/media/merged" = {
    fsType = "fuse.mergerfs";
    device = "/media/disk1";
    options = [
      "nofail"
      "cache.files=auto-full"
      "dropcacheonclose=true"
      "x-systemd.requires=/media/disk1"
      "x-systemd.device-timeout=5"
      "x-systemd.automount"
      "fsname=e5f13730-4f09-4eb3-80f7-b2cfb3285e1d"
      "category.create=pfrd"
      "func.getattr=newest"
    ];
    neededForBoot = false;
  };

  services.snapraid = {
    enable = true;
    parityFiles = [ "/media/parity1/snapraid.parity" ];
    contentFiles = [
      "/var/snapraid/content"
      "/media/disk1/snapraid.content"
    ];
    dataDisks = {
      d1 = "/media/disk1/";
    };
    sync = {
      # SYNC snapraid daily at 5:00
      interval = "05:00";
    };
    scrub = {
      # SCRUB snapraid weekly on monday at 4:00
      interval = "Mon *-*-* 04:00";
    };
  };

  fileSystems."/export/proxmox-backup" = {
    fsType = "fuse.mergerfs";
    device = "/media/merged/proxmox-backup";
    options = [ "bind" ];
  };
  fileSystems."/export/uncloud-garage" = {
    fsType = "fuse.mergerfs";
    device = "/media/merged/uncloud/garage";
    options = [ "bind" ];
  };
  fileSystems."/export/uncloud-immich" = {
    fsType = "fuse.mergerfs";
    device = "/media/merged/uncloud/immich";
    options = [ "bind" ];
  };

  services.nfs.server = {
    enable = true;
    exports = ''
      /export/proxmox-backup 10.0.0.0/16(fsid=1,rw,subtree_check,insecure,all_squash)
      # /export/kube 10.0.0.0/16(fsid=2,rw,subtree_check,insecure,root_squash)
      /export/uncloud-garage 10.0.0.0/16(fsid=4,rw,subtree_check,insecure,root_squash)
      /export/uncloud-immich 10.0.0.0/16(fsid=5,rw,subtree_check,insecure,root_squash)
      /export 10.0.0.0/16(ro,fsid=0,root_squash,no_subtree_check,hide)
    '';
  };

  environment.etc."autorestic.yml" = {
    text = ''
      version: 2

      extras:
        policies: &backup-policy
          keep-daily: 7
          keep-weekly: 8
          keep-yearly: 2
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

      locations:
        immich:
          <<: *standard
          from: /media/merged/uncloud/immich
          cron: '0 2 * * *'
        snapraid-system-content:
          <<: *standard
          from: /persist/var/snapraid/content
          cron: '0 * * * *'
    '';
  };

  users.users = {
    root = {
      hashedPasswordFile = "/persist/etc/shadowRoot";
    };
  };

  services.cron.systemCronJobs = [
    "*/5 * * * * root ${run-autorestic}/bin/run-autorestic.sh"
    "3 1 * * * root ${prune-autorestic}/bin/prune-autorestic.sh"
    # Run short SMART test once a day
    "0 5 * * * root ${pkgs.smartmontools}/bin/smartctl -t short /dev/sdb"
    "0 5 * * * root ${pkgs.smartmontools}/bin/smartctl -t short /dev/sda"
    # Run long SMART test once a month
    "0 4 1 * * root ${pkgs.smartmontools}/bin/smartctl -t long /dev/sdb"
    "0 4 1 * * root ${pkgs.smartmontools}/bin/smartctl -t long /dev/sda"
  ];

  time.timeZone = "Europe/Paris";

  # Impermanence
  environment.persistence."/persist" = {
    files = [
      "/var/lib/hdd_key"
    ];
    directories = [
      "/var/snapraid"
      "/root/.ssh"
    ];
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

  # Secure settings
  users.mutableUsers = false;
  security.sudo.enable = false;
  # If I disable it I cannot boot anymore
  nix.enable = true;

  # Networking
  networking = {
    hostName = "nas-nfs";
    domain = "";
  };

  # Remote updates and flakes
  nix.settings.trusted-users = [ "root" "@wheel" ]; # Allow remote updates

  boot.tmp.cleanOnBoot = true;
}
