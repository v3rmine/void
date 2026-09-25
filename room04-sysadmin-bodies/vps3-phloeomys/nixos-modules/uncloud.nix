{ pkgs, config, ... }: 
let
  uncloud = pkgs.stdenv.mkDerivation rec {
    pname = "uncloud";
    version = "0.20.0";
    src = builtins.fetchTarball {
      url = "https://github.com/psviderski/uncloud/releases/download/v${version}/uncloudd_linux_amd64.tar.gz";
      sha256 = "13nalmnlja3j8s5xa9b88b429ybrq06bkadl7n4am9fkbi2v0n2l";
    };

    installPhase = ''
      runHook preInstall
      mkdir -p $out/bin
      cp uncloudd $out/bin
      runHook postInstall
    '';
  };
in {
  imports = [];
  options = {};

  config = {
    environment.persistence."/persist".directories = [
      "/var/lib/uncloud"
      "/var/lib/docker"
      "/var/lib/containerd"
    ];

    virtualisation = {
      containerd.enable = true;
      docker = {
        enable = true;
        daemon.settings = {
          features = {
            containerd-snapshotter = true;
          };
          storage-driver = "overlayfs";
          live-restore = true;
        };
      };
    };

    environment.systemPackages = with pkgs; [
      uncloud
    ];

    users.users.uncloud = {
      isSystemUser = true;
      createHome = false;
      home = "/var/empty";
      shell = pkgs.shadow;
      group = "uncloud";
    };

    users.groups.uncloud = {};

    systemd.services."uncloud" = {
      enable = true;
      after = [ "network-online.target" "docker.service" ];
      wants = [ "network-online.target" "docker.service" "firewall.service" "tailscaled.service" ];
      path = [ pkgs.iptables ];
      serviceConfig = {
        Type = "notify";
        ExecStart = "${uncloud}/bin/uncloudd";
        TimeoutStartSec = 15;
        Restart = "always";
        RestartSec = 2;
        # Hardening options.
        NoNewPrivileges = true;
        ProtectSystem = "full";
        ProtectControlGroups = true;
        ProtectHome = "read-only";
        ProtectKernelTunables = true;
        PrivateTmp = true;
        RestrictAddressFamilies = "AF_INET AF_INET6 AF_UNIX AF_NETLINK";
        RestrictNamespaces = true;
      };
      wantedBy = [ "multi-user.target" ];
    };
  };
}
