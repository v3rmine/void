{ modulesPath, pkgs, lib, ... }:
let
  get_import_dir =
    dir:
    lib.flatten (
      lib.pipe dir [
        builtins.readDir
        (lib.filterAttrs (name: type: type == "directory" || lib.hasSuffix ".nix" name))
        (lib.mapAttrsToList (
          name: type: if type == "directory" then get_import_dir (dir + ("/" + name)) else dir + ("/" + name)
        ))
      ]
    );
  trusted-ssh-keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIHt8EvWzOBZxA7JEQCnknW+hjEi8Id6dfMtb1ONw1xbw astrid@astrid-lux"
  ];
in {
  system.stateVersion = "25.11";
  imports = [] 
    ++ (get_import_dir ./nixos-modules/.)
    ++ (get_import_dir ../../../../room05-sysadmin-souls/services/nas/.);

  networking.firewall = {
    enable = false;
    allowedTCPPorts = [ 8080 ];
    allowedUDPPorts = [];
  };

  environment.systemPackages = with pkgs; [
    docker-compose
    vim
    iperf
    htop
    yq-go
  ];

  environment.etc."autorestic.yml" = {
    text = ''
      version: 2

      extras:
        policies: &backup-policy
          keep-daily: 7
          keep-weekly: 8
          keep-yearly: 2
        standard: &standard
          to:
            - backblaze
            - hetzner
          options:
            backup:
              compression: max
              skip-if-unchanged: true
            forget:
              <<: *backup-policy

      locations:
        systemd-services:
          <<: *standard
          from: /persist/var/lib/systemd/system
          cron: '0 * * * *'
        syncthing:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/syncthing-config
            - /persist/var/lib/docker/volumes/syncthing-data
          cron: '0 * * * *'
        garage-hot:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/garage-hot-meta
            - /persist/var/lib/docker/volumes/garage-hot-data
          cron: '0 * * * *'
        garage-cold:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/garage-cold-meta
          cron: '0 * * * *'
        grafana:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/grafana-data
          cron: '0 * * * *'
        suwayomi:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/suwayomi-data
          cron: '0 * * * *'
        forgejo:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/forgejo-data
        immich:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/immich-postgres
        memos:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/memos-data
        bichon:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/bichon-data
    '';
  };

  users.users = {
    root = {
      hashedPasswordFile = "/persist/etc/shadowRoot";
    };
  };

  services.caddy = {
    enable = true;
    httpPort = 80;
    httpsPort = 0;
    package = pkgs.caddy.withPlugins {
      plugins = [ "github.com/caddyserver/cache-handler@v0.16.0" ];
      hash = "sha256-41uiGaunQpwi2vpK+Kuy4eGpbTqjSdRhekPT1UeT9AY=";
    };
    globalConfig = ''
      auto_https off
    '';
  };

  services.tailscale = {
    enable = true;
    extraUpFlags = [ "--ssh" ];
  };

  time.timeZone = "Europe/Paris";

  # Impermanence
  environment.persistence."/persist" = {
    directories = [
      "/var/lib/tailscale"
      "/root/.ssh"
    ];
  };

  # SSH
  services.openssh = {
    enable = true;
    ports = [ 8080 ];
    openFirewall = true;
    allowSFTP = true;
    settings = {
      PermitRootLogin = "without-password";
      PasswordAuthentication = false;
      KbdInteractiveAuthentication = false;
    };
    extraConfig = ''
      AllowTcpForwarding yes
      X11Forwarding no
      AllowAgentForwarding no
      AllowStreamLocalForwarding yes
      AuthenticationMethods publickey
      ClientAliveCountMax 2
      LogLevel verbose
      MaxAuthTries 3
      MaxSessions 4
      TCPKeepAlive no
    '';
  };
  users.users.root.openssh.authorizedKeys.keys = trusted-ssh-keys;
  users.users.podman.openssh.authorizedKeys.keys = trusted-ssh-keys;

  # Secure settings
  users.mutableUsers = false;
  services.lvm.enable = false;
  security.sudo.enable = false;
  # If I disable it I cannot boot anymore
  nix.enable = true;

  # Hostname
  networking.hostName = "nas-uncloud";
  networking.domain = "";

  # Remote updates and flakes
  nix.settings.trusted-users = [ "root" "@wheel" ]; # Allow remote updates

  boot.tmp.cleanOnBoot = true;
  
  swapDevices = [{
    device = "/persist/var/lib/swapfile";
    size = 8 * 1024; # 8 GiB
  }];
}
