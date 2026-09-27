{ pkgs, config, ... }: 
let
  custom-newt = pkgs.stdenv.mkDerivation rec {
    pname = "newt";
    version = "1.13.0";
    src = builtins.fetchurl {
      url = "https://github.com/fosrl/newt/releases/download/${version}/newt_linux_amd64";
      sha256 = "070sixc3yg9w7wif714gfmjg1phgrs3vq8xyywcq9mq3wg30pnw1";
    };
    dontUnpack = true;

    installPhase = ''
      runHook preInstall
      mkdir -p $out/bin
      cp $src $out/bin/newt
      chmod +x $out/bin/newt
      runHook postInstall
    '';
  };
in {
  imports = [];
  options = {};

  config = {
    environment.systemPackages = with pkgs; [
      custom-newt
    ];

    systemd.services."newt" = {
      enable = true;
      after = [ "network-online.target" ];
      wants = [ "network-online.target" ];
      serviceConfig = {
        Type = "simple";
        Restart = "always";
        RestartSec = 2;
        # Hardening options.
        NoNewPrivileges = true;
        PrivateTmp = true;
        ProtectControlGroups = true;
        ProtectHome = true;
        ProtectKernelTunables = true;
        ProtectSystem = "full";
        RestrictAddressFamilies = "AF_INET AF_INET6 AF_UNIX AF_NETLINK";
        RestrictNamespaces = true;
      };
      wantedBy = [ "multi-user.target" ];
    };

    boot.extraSystemdUnitPaths = [ "/var/lib/systemd/system/" ];

    systemd.services."reload-daemon-after-boot" = {
      enable = true;
      after = [ "persist.mount" ];
      path = [ pkgs.systemd ];
      serviceConfig = {
        Type = "oneshot";
        ExecStart = "/bin/sh -c 'systemctl daemon-reload && systemctl start newt'";
      };
      wantedBy = [ "multi-user.target" ];
    };
  };
}
