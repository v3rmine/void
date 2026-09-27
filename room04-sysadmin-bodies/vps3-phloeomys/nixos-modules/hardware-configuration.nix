{ modulesPath, pkgs, lib, ... }: {
  imports = [ (modulesPath + "/profiles/qemu-guest.nix") ];
  options = {};

  config = {
    services.qemuGuest.enable = true;

    boot.kernelPackages = pkgs.linuxPackages_latest;
    boot.loader.grub.device = "/dev/sda";
    boot.loader.grub.storePath = "/persist//nix/store";
    boot.initrd.availableKernelModules = [ "ata_piix" "uhci_hcd" "xen_blkfront" "vmw_pvscsi" ];
    boot.initrd.kernelModules = [ "nvme" ];

    networking = {
      defaultGateway = {
        address = "172.31.1.1";
        interface = "eth0";
      };
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
  };
}
