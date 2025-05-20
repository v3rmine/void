{ modulesPath, lib, pkgs, ... }: {
  security.pam.services.sshd.allowNullPassword = true;
  services.openssh = {
    enable = true;
    settings = {
      PermitRootLogin = "yes";
      PasswordAuthentication = true;
    };
  };
  system.stateVersion = "24.11";

  time.timeZone = "Europe/Paris";

  networking = {
    hostName = "k3s";
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
  boot.loader.grub.devices = [ "nodev" ];
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
