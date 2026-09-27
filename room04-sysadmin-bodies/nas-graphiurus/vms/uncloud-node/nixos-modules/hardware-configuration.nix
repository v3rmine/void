{ modulesPath, pkgs, lib, ... }: {
  imports = [ (modulesPath + "/profiles/qemu-guest.nix") ];
  options = {};

  config = {
    services.qemuGuest.enable = true;
    services.rpcbind.enable = true;

    boot.growPartition = true;
    boot.kernelPackages = pkgs.linuxPackages_latest;
    boot.loader.grub.device = "/dev/vda";
    boot.loader.grub.storePath = "/persist//nix/store";
    boot.initrd.availableKernelModules = [ "ata_piix" "uhci_hcd" "xen_blkfront" "vmw_pvscsi" ];
    boot.initrd.kernelModules = [ "nvme" ];
    boot.supportedFilesystems = [ "nfs" ];

    networking = {
      nameservers = [ "10.0.0.1" ];
      defaultGateway = "10.0.0.1";
      defaultGateway6 = {
        address = "";
        interface = "eth0";
      };
      dhcpcd.enable = true;
    };
  };
}
