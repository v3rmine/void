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
in {
  system.stateVersion = "25.11";
  imports = [
    ./hardware-configuration.nix
    ../../room09-chaotic-library/caddy-config.nix
    ../../room09-chaotic-library/docker-compose.nix
    ../../room02-trash-pile-of-tools/quick-but-mostly-dirty/addy-smtp-proxy/docker-compose.nix
  ] ++ (get_import_dir ./nixos-modules/.) 
    ++ (get_import_dir ../../room05-sysadmin-souls/uncloud/phloeomys/.);

  networking.firewall = {
    enable = true;
    interfaces = {
      "eth0" = {
        allowedTCPPorts = [ 22 80 443 3478 5349 6697 8080 22000 51000 ];
        allowedUDPPorts = [ 53 443 3478 5349 21027 21820 22000 51001 51820 51830 ];
        allowedUDPPortRanges = [
          { from = 49160; to = 49170; }
        ];
      };
    };
  };

  environment.systemPackages = with pkgs; [
    docker-compose
    git
    vim
    htop
    yq-go
    cifs-utils
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
        backblaze: &backblaze
          to:
            - backblaze
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
        pangolin:
          <<: *standard
          from: /root/pangolin
          cron: '0 * * * *'
        filestash:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/filestash-config
          cron: '0 * * * *'
        goatcounter:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/goatcounter-data
          cron: '0 * * * *'
        miniflux:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/miniflux-data
          cron: '0 * * * *'
        linkding:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/linkding-data
          cron: '0 * * * *'
        continuwuity:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/continuwuity-data
          cron: '0 * * * *'
        sharkey:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/sharkey-files
            - /persist/var/lib/docker/volumes/sharkey-db
          cron: '0 * * * *'
        vaultwarden:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/vaultwarden-data
          cron: '0 * * * *'
        drive:
          <<: *backblaze
          from:
            - /persist/mnt/drive
          cron: '0 2 * * *'
          options:
            backup:
              compression: max
              skip-if-unchanged: true
              exclude-file: /persist/mnt/drive/.backup-ignore
            forget:
              <<: *backup-policy
        personal-website:
          <<: *standard
          from:
            - /persist/var/lib/docker/volumes/tor-data
            - /persist/var/lib/docker/volumes/townsquare-data
          cron: '0 * * * *'
    '';
  };

  users.users = {
    root = {
      hashedPasswordFile = "/persist/etc/shadowRoot";
    };
  };

  services.caddy = {
    enable = true;
    httpPort = 8000;
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
      "/root/pangolin"
      "/root/.ssh"
    ];
  };

  # SSH
  services.openssh = {
    enable = true;
    ports = [ 8080 ];
    openFirewall = true;
    allowSFTP = false;
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
  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIHt8EvWzOBZxA7JEQCnknW+hjEi8Id6dfMtb1ONw1xbw astrid@astrid-lux"
  ];

  # Secure settings
  users.mutableUsers = false;
  services.lvm.enable = false;
  security.sudo.enable = false;
  nix.enable = true;

  # Hostname
  networking.hostName = "phloeomys";
  networking.domain = "";

  # Remote updates and flakes
  nix.settings.trusted-users = [ "root" "@wheel" ]; # Allow remote updates

  boot.tmp.cleanOnBoot = true;

  swapDevices = [{
    device = "/persist/var/lib/swapfile";
    size = 8 * 1024; # 8 GiB
  }];

  networking = {
    nameservers = [ "8.8.8.8" "1.1.1.1" ];
  };
}
