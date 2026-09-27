{ ... }: {
  imports = [];
  options = {};

  config = {
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
  };
}
