{ modulesPath, pkgs, lib, ... }:
let
  impermanence = builtins.fetchTarball
    "https://github.com/nix-community/impermanence/archive/master.tar.gz";
in {
  system.stateVersion = "25.11";
  system.autoUpgrade.channel = "https://nixos.org/channels/nixos-25.11-small";

  networking.firewall.enable = false;
  networking.firewall.allowedTCPPorts = [
  ];
  networking.firewall.allowedUDPPorts = [
  ];
  environment.systemPackages = with pkgs; [
    mergerfs
    cryptsetup
    smartmontools
    openseachest
  ];

  boot.kernelModules = [ "dm_crypt" ];
  environment.etc.crypttab = {
    mode = "0600";
    text = ''
      # <volume-name> <encrypted-device> [key-file] [options]
      parity1 UUID=e5e2fa22-6b0d-4516-b5da-ffe826a02736 /var/lib/hdd_key luks,discard,tries=1,noauto
      disk1 UUID=dbb1e30c-44f3-4541-8941-2452a544b93b /var/lib/hdd_key luks,discard,tries=1,noauto
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
  };

  fileSystems."/media/merged" = {
    fsType = "fuse.mergerfs";
    device = "/media/disk1";
    options = [
      "cache.files=auto-full"
      "dropcacheonclose=true"
      "x-systemd.requires=/media/disk1"
      "x-systemd.device-timeout=5"
      "x-systemd.automount"
      "fsname=e5f13730-4f09-4eb3-80f7-b2cfb3285e1d"
      "category.create=pfrd"
      "func.getattr=newest"
    ];
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
    device = "/media/merged/proxmox-backup";
    options = [ "bind" ];
  };
  fileSystems."/export/kube" = {
    device = "/media/merged/kube";
    options = [ "bind" ];
  };
  fileSystems."/export/uncloud-palmr-uploads" = {
    device = "/media/merged/uncloud/palmr-uploads";
    options = [ "bind" ];
  };

  services.nfs.server = {
    enable = true;
    exports = ''
      /export/proxmox-backup 10.0.0.0/16(fsid=1,rw,subtree_check,insecure,all_squash)
      /export/kube 10.0.0.0/16(fsid=2,rw,subtree_check,insecure,root_squash)
      /export/uncloud-palmr-uploads 10.0.0.0/16(fsid=3,rw,subtree_check,insecure,root_squash)
      /export 10.0.0.0/16(ro,fsid=0,root_squash,no_subtree_check,hide)
    '';
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
      "/var/lib/hdd_key"
    ];
    directories = [
      "/boot"
      "/nix"
      "/tmp"
      "/var/tmp"
      "/var/lib/nixos"
      "/var/log"
      "/var/snapraid"
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
  boot.supportedFilesystems = [ "nfs" ];
  services.rpcbind.enable = true;
  zramSwap.enable = false;

  # Secure settings
  users.mutableUsers = false;
  security.sudo.enable = false;
  # If I disable it I cannot boot anymore
  nix.enable = true;

  # Networking
  networking = {
    hostName = "nas-nfs";
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
  services.qemuGuest.enable = true; # Enable QEMU Guest for Proxmox

  # Hardware configuration
  imports = [ (modulesPath + "/profiles/qemu-guest.nix") "${impermanence}/nixos.nix" ];
  boot.loader.grub.enable = lib.mkDefault true; # Use the boot drive for GRUB
  boot.loader.grub.device = "/dev/vda";
  boot.loader.grub.storePath = "/persist/nix/store";
  boot.tmp.cleanOnBoot = true;
  boot.growPartition = lib.mkDefault true;
  boot.initrd.availableKernelModules =
    [ "ata_piix" "uhci_hcd" "xen_blkfront" "vmw_pvscsi" ];
  boot.initrd.kernelModules = [ "nvme" ];
  boot.kernelPackages = pkgs.linuxPackages_latest;
}
