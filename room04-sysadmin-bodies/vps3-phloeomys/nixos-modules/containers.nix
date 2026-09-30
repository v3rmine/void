{ pkgs, config, ... }: {
  imports = [];
  options = {};

  config = {
    environment.persistence."/persist".directories = [
      "/var/lib/docker"
      "/var/lib/rootless-podman/.local/share/docker"
      "/var/lib/vaultwarden/.local/share/docker"
    ];

    boot.kernel.sysctl = {
      "net.ipv4.ip_unprivileged_port_start" = 22;
      "net.ipv6.conf.all.forwarding" = 1;
      "net.ipv6.conf.default.forwarding" = 1;
    };

    virtualisation = {
      docker = {
        enable = true;
        rootless = {
          enable = true;
          setSocketVariable = true;
          # extraPackages = with pkgs; [ nftables ];
          daemon.settings = {
            fixed-cidr-v6 = "fd00::/80";
            ipv6 = true;
          };
        };
        # extraPackages = with pkgs; [nftables];
        # extraOptions = "--firewall-backend=nftables";
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

    users.users.vaultwarden = {
      uid = 1001;
      isSystemUser = true;
      createHome = true;
      home = "/var/lib/vaultwarden";
      shell = pkgs.bash;
      group = "vaultwarden";
      linger = true;

      subUidRanges = [ { startUid = 100000; count = 65536; } ];
      subGidRanges = [ { startGid = 100000; count = 65536; } ];

      extraGroups = [];
    };
    users.groups.vaultwarden = { gid = 1001; };
  };
}
