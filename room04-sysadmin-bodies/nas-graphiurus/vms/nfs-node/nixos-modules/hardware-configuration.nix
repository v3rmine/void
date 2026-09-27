{ modulesPath, pkgs, ... }: {
  imports = [ (modulesPath + "/profiles/qemu-guest.nix") ];
  options = {};

  config = {
    services.qemuGuest.enable = true; # Enable QEMU Guest for Proxmox
    services.rpcbind.enable = true;

    boot.kernelPackages = pkgs.linuxPackages_latest;
    boot.loader.grub.enable = true; # Use the boot drive for GRUB
    boot.loader.grub.device = "/dev/vda";
    boot.loader.grub.storePath = "/persist//nix/store";
    boot.initrd.availableKernelModules = [ "ata_piix" "uhci_hcd" "xen_blkfront" "vmw_pvscsi" ];
    boot.initrd.kernelModules = [ "nvme" ];
    boot.supportedFilesystems = [ "nfs" ];
    
    networking = {
      nameservers = [ "10.0.0.1" ];
      defaultGateway = {
        address = "10.0.0.1";
        interface = "ens18";
      };
      defaultGateway6 = {
        address = "";
        interface = "ens18";
      };
      dhcpcd.enable = true;
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
        fsType = "ext4";
        neededForBoot = true;
      };
    };
  };
}
