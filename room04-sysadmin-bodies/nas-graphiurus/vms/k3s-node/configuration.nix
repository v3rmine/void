{ modulesPath, lib, pkgs, ... }: {
  system.stateVersion = "24.11";

  # k3s https://github.com/NixOS/nixpkgs/blob/master/pkgs/applications/networking/cluster/k3s/docs/USAGE.md
  networking.firewall.allowedTCPPorts = [
    6443 # k3s: required so that pods can reach the API server (running on port 6443 by default)
    # 2379 # k3s, etcd clients: required if using a "High Availability Embedded etcd" configuration
    # 2380 # k3s, etcd peers: required if using a "High Availability Embedded etcd" configuration
  ];
  networking.firewall.allowedUDPPorts = [
    # 8472 # k3s, flannel: required if using multi-node for inter-node networking
  ];
  services.k3s = {
    enable = true;
    role = "server";
    token = "e2f487587c0c12e491ea9f26ab22ff8a8f72a147907fbf78165c6f0d1a2afda0005f450138e3783126005a9444c4b7398735f9c6a2ad849db09e6cef63fb07cb";
    extraFlags = toString [
      # "--debug" # Optionally add additional args to k3s
    ];
    clusterInit = true;
  };
  
  # https://github.com/NixOS/nixpkgs/blob/master/pkgs/applications/networking/cluster/k3s/docs/examples/STORAGE.md#longhorn
  environment.systemPackages = with pkgs; [
    nfs-utils
  ];

  # Required for Longhorn
  services.openiscsi = {
    enable = true;
    name = "nas-k3s-initiatorhost";
  };

  boot.supportedFilesystems = [ "nfs" ];
  services.rpcbind.enable = true;

  # System
  services.openssh = {
    enable = true;
    settings = {
      PermitRootLogin = "yes";
      PasswordAuthentication = true;
    };
  };

  time.timeZone = "Europe/Paris";

  networking = {
    hostName = "nas-k3s";
    domain = "";
    nameservers = [ "10.0.0.1" ];
    defaultGateway = "10.0.0.1";
    defaultGateway6 = {
      address = "";
      interface = "eth0";
    };
    dhcpcd.enable = true;
  };

  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIHt8EvWzOBZxA7JEQCnknW+hjEi8Id6dfMtb1ONw1xbw astrid@astrid-lux"
  ];

  # Remote updates and flakes
  nix.settings.trusted-users = [ "root" "@wheel" ]; # Allow remote updates
  nix.settings.experimental-features =
    [ "nix-command" "flakes" ]; # Enable flakes

  # Promox
  services.qemuGuest.enable =
    lib.mkDefault true; # Enable QEMU Guest for Proxmox

  # Hardware configuration
  imports = [ (modulesPath + "/profiles/qemu-guest.nix") ];
  boot.loader.grub.enable = lib.mkDefault true; # Use the boot drive for GRUB
  boot.loader.grub.device = "/dev/vda";
  boot.tmp.cleanOnBoot = true;
  boot.growPartition = lib.mkDefault true;
  boot.initrd.availableKernelModules =
    [ "ata_piix" "uhci_hcd" "xen_blkfront" "vmw_pvscsi" ];
  boot.initrd.kernelModules = [ "nvme" ];
  boot.kernelPackages = pkgs.linuxPackages_latest;
  fileSystems."/" = lib.mkDefault {
    device = "/dev/vda1";
    autoResize = true;
    fsType = "ext4";
  };
}
