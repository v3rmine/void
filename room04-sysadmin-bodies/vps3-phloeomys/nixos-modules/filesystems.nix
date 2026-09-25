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
        device = "/dev/sda1";
        fsType = "ext4";
        neededForBoot = true;
      };
      "/persist/mnt/drive" = {
        device = "//u264248-sub2.your-storagebox.de/u264248-sub2";
        fsType = "cifs";
        options = let
          # this line prevents hanging on network split
          automount_opts = "x-systemd.automount,noauto,x-systemd.idle-timeout=60,x-systemd.device-timeout=5s,x-systemd.mount-timeout=5s";
        in ["ro,${automount_opts},credentials=/persist/etc/cifs-drive-credentials"];
        neededForBoot = false;
      };
    };
  };
}
