{ pkgs, config, ... }: {
  imports = [];
  options = {};

  config = {
    environment.persistence."/persist" = {
      directories = [
        "/var/lib/docker"
        "/var/lib/rootless-podman/.local/share/docker"
        "/var/lib/immich/.local/share/docker"
      ];
    };

    virtualisation = {
      docker = {
        enable = true;
        rootless = {
          enable = true;
          setSocketVariable = true;
        };
      };

      oci-containers.backend = "docker";
    };

    users.users.podman = {
      uid = 993;
      isSystemUser = true;
      createHome = true;
      home = "/var/lib/rootless-podman";
      shell = pkgs.bash;
      group = "nogroup";
      linger = true;

      subUidRanges = [ { startUid = 100000; count = 65536; } ];
      subGidRanges = [ { startGid = 100000; count = 65536; } ];

      extraGroups = [];
    };
    
    users.users.immich = {
      uid = 1001;
      isSystemUser = true;
      createHome = true;
      home = "/var/lib/immich";
      shell = pkgs.bash;
      group = "immich";
      linger = true;

      subUidRanges = [ { startUid = 100000; count = 65536; } ];
      subGidRanges = [ { startGid = 100000; count = 65536; } ];

      extraGroups = [];
    };
    users.groups.immich = { gid = 1001; };
  };
}
