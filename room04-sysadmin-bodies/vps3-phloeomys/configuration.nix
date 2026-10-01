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
  acme-json-to-certs = pkgs.writeShellScriptBin "acme-json-to-certs.sh" ''
     DOMAIN="$1"
     ACME="$2"
     if [ -z "$DOMAIN" ] || [ -z "$ACME" ]; then
       echo "Usage: acme-json-to-certs.sh example.com path/to/acme.json"
       exit 1
     fi

     mkdir -p "$DOMAIN"
     ${pkgs.jq}/bin/jq -r --arg d "$DOMAIN" '.[].Certificates[] | select(.domain.main==$d) | .certificate' "$ACME" | base64 -d > "$DOMAIN/fullchain.pem"
     ${pkgs.jq}/bin/jq -r --arg d "$DOMAIN" '.[].Certificates[] | select(.domain.main==$d) | .key' "$ACME" | base64 -d > "$DOMAIN/privkey.pem" 
  '';
in {
  system.stateVersion = "25.11";
  imports = [
    ../../room09-chaotic-library/caddy-config.nix
    ../../room09-chaotic-library/docker-compose.nix
    ../../room02-trash-pile-of-tools/quick-but-mostly-dirty/addy-smtp-proxy/docker-compose.nix
  ] ++ (get_import_dir ./nixos-modules/.) 
    ++ (get_import_dir ../../room05-sysadmin-souls/services/phloeomys/.);

  networking.firewall = {
    enable = true;
    interfaces = {
      "eth0" = {
        allowedTCPPorts = [ 22 80 443 3478 5222 5269 5349 6697 8080 22000 51000 ];
        allowedUDPPorts = [ 53 443 3478 5349 21027 21820 22000 51001 51820 51830 ];
        allowedUDPPortRanges = [
          { from = 49160; to = 49170; }
        ];
      };
    };
  };

  services.fail2ban = {
    enable = true;
    ignoreIP = [
      "172.19.0.1"
      "fd8e:820:47f1::1"
    ];
    extraPackages = with pkgs; [ ipset ];
    banaction = "iptables-ipset[type=multiport]";
    banaction-allports = "iptables-ipset[type=allports]";
    jails = {
      "DEFAULT".settings = {
        action_ = ''
          %(banaction)s[name=%(__name__)s-inp, port="%(port)s", protocol="tcp,udp", chain="INPUT"]
                    %(banaction)s[name=%(__name__)s-fwd, port="%(port)s", protocol="tcp,udp", chain="FORWARD"]'';
      };
      traefik-429.settings = {
        enabled = true;
        filter = "traefik-429";
        logpath = "/var/log/logs/traefik/access.log";
        maxretry = "10";
        findtime = "10m";
        bantime = "1h";
        backend = "auto";
      };
      traefik-http.settings = {
        enabled = true;
        filter = "traefik-http";
        logpath = "/var/log/logs/traefik/access.log";
        maxretry = "20";
        findtime = "1h";
        bantime = "24h";
        backend = "auto";
      };
      traefik-wordpress.settings = {
        enabled = true;
        filter = "traefik-wordpress";
        logpath = "/var/log/logs/traefik/access.log";
        maxretry = "1";
        findtime = "5min";
        bantime = "24h";
        backend = "auto";
      };
      iocaine-garbage.settings = {
        enabled = true;
        filter = "iocaine-garbage";
        journalmatch = "CONTAINER_NAME=pangolin-iocaine";
        maxretry = "50";
        findtime = "10m";
        bantime = "1h";
        backend = "systemd";
      };
    };
  };
  environment.etc."fail2ban/filter.d/traefik-429.conf".text = ''
    [INCLUDES]
    before = common.conf
    [Definition]
    failregex = ^\{.*"ClientHost":"<HOST>".*"DownstreamStatus":429.*\}$
  '';
  environment.etc."fail2ban/filter.d/traefik-http.conf".text = ''
    [INCLUDES]
    before = common.conf
    [Definition]
    failregex = ^\{.*"ClientHost":"<HOST>".*"RequestScheme":"http".*\}$
  '';
  environment.etc."fail2ban/filter.d/traefik-wordpress.conf".text = ''
    [INCLUDES]
    before = common.conf
    [Definition]
    failregex = ^\{.*"ClientHost":"<HOST>".*"RequestPath":"/(wp-admin|wp-content|wp-includes)[^"]+".*\}$
  '';
  environment.etc."fail2ban/filter.d/iocaine-garbage.conf".text = ''
    [INCLUDES]
    before = common.conf
    [Definition]
    failregex = ^.*\{.*"x-forwarded-for":"<HOST>".*"verdict\.outcome":"garbage".*\}$
                ^.*\{.*"verdict\.outcome":"garbage".*"x-forwarded-for":"<HOST>".*\}$
  '';

  environment.systemPackages = with pkgs; [
    docker-compose
    git
    vim
    htop
    yq-go
    jq
    acme-json-to-certs
    cifs-utils
    ipset
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
            - /persist/var/lib/rootless-podman/.local/share/docker/volumes/filestash-config
          cron: '0 * * * *'
        goatcounter:
          <<: *standard
          from:
            - /persist/var/lib/rootless-podman/.local/share/docker/volumes/goatcounter-data
          cron: '0 * * * *'
        miniflux:
          <<: *standard
          from:
            - /persist/var/lib/rootless-podman/.local/share/docker/volumes/miniflux-data
          cron: '0 * * * *'
        linkding:
          <<: *standard
          from:
            - /persist/var/lib/rootless-podman/.local/share/docker/volumes/linkding-data
          cron: '0 * * * *'
        continuwuity:
          <<: *standard
          from:
            - /persist/var/lib/rootless-podman/.local/share/docker/volumes/continuwuity-data
          cron: '0 * * * *'
        sharkey:
          <<: *standard
          from:
            - /persist/var/lib/rootless-podman/.local/share/docker/volumes/sharkey-files
            - /persist/var/lib/rootless-podman/.local/share/docker/volumes/sharkey-db
          cron: '0 * * * *'
        vaultwarden:
          <<: *standard
          from:
            - /persist/var/lib/vaultwarden/.local/share/docker/volumes/vaultwarden-data
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
            - /persist/var/lib/rootless-podman/.local/share/docker/volumes/tor-data
            - /persist/var/lib/rootless-podman/.local/share/docker/volumes/townsquare-data
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
      "/var/lib/fail2ban"
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
  users.users.root.openssh.authorizedKeys.keys = trusted-ssh-keys;
  users.users.podman.openssh.authorizedKeys.keys = trusted-ssh-keys;
  users.users.vaultwarden.openssh.authorizedKeys.keys = trusted-ssh-keys;

  services.cron.systemCronJobs = [
    "0 * * * * root ${pkgs.bash}/bin/bash -c 'cd /var/lib/rootless-podman/.local/share/docker/volumes/prosody_prosody-data/_data && ${acme-json-to-certs}/bin/acme-json-to-certs.sh astriiid.fr /root/pangolin/letsencrypt/acme.json && ${acme-json-to-certs}/bin/acme-json-to-certs.sh files.astriiid.fr /root/pangolin/letsencrypt/acme.json && ${acme-json-to-certs}/bin/acme-json-to-certs.sh muc.astriiid.fr /root/pangolin/letsencrypt/acme.json && chown -R 100099:100101 * && DOCKER_HOST=unix:///run/user/993/docker.sock ${pkgs.docker}/bin/docker exec -it prosody-prosody prosodyctl --root cert import /var/lib/prosody/letsencrypt'"
  ];

  # Secure settings
  users.mutableUsers = false;
  services.lvm.enable = false;
  security.sudo.enable = false;
  nix.enable = true;

  networking = {
    hostName = "phloeomys";
    domain = "";
    nameservers = [ "8.8.8.8" "1.1.1.1" ];
  };

  nix.settings.trusted-users = [ "root" "@wheel" ]; # Allow remote updates

  boot.tmp.cleanOnBoot = true;

  swapDevices = [{
    device = "/persist/var/lib/swapfile";
    size = 8 * 1024; # 8 GiB
  }];

  systemd.services."user@".serviceConfig = {
    Delegate = "pids memory cpu cpuset";
  };

}
