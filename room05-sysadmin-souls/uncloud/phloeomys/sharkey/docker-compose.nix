
{ pkgs, lib, ... }:

{


  # Containers
  virtualisation.oci-containers.containers."sharkey-sharkey" = {
    image = "registry.activitypub.software/transfem-org/sharkey:2025.4.7";
    environmentFiles = [
      "/persist/etc/sharkey.env"
    ];
    volumes = [
      "sharkey-config:/sharkey/.config:rw"
      "sharkey-files:/sharkey/files:rw"
    ];
    ports = [
      "63012:3000/tcp"
    ];
    dependsOn = [
      "sharkey-sharkey-db"
      "sharkey-sharkey-meilisearch"
      "sharkey-sharkey-redis"
    ];
    user = "991:991";
    log-driver = "journald";
    extraOptions = [
      "--memory=4294967296b"
      "--network-alias=sharkey"
      "--network=sharkey_default"
    ];
  };
  systemd.services."docker-sharkey-sharkey" = {
    serviceConfig = {
      Restart = lib.mkOverride 90 "always";
      RestartMaxDelaySec = lib.mkOverride 90 "1m";
      RestartSec = lib.mkOverride 90 "100ms";
      RestartSteps = lib.mkOverride 90 9;
    };
    after = [
      "docker-network-sharkey_default.service"
      "docker-volume-sharkey_sharkey-config.service"
      "docker-volume-sharkey_sharkey-files.service"
    ];
    requires = [
      "docker-network-sharkey_default.service"
      "docker-volume-sharkey_sharkey-config.service"
      "docker-volume-sharkey_sharkey-files.service"
    ];
    partOf = [
      "docker-compose-sharkey-root.target"
    ];
    wantedBy = [
      "docker-compose-sharkey-root.target"
    ];
  };
  virtualisation.oci-containers.containers."sharkey-sharkey-db" = {
    image = "groonga/pgroonga:4.0.1-alpine-17";
    environmentFiles = [
      "/persist/etc/sharkey.env"
    ];
    volumes = [
      "sharkey-db:/var/lib/postgresql/data:rw"
    ];
    log-driver = "journald";
    extraOptions = [
      "--health-cmd=pg_isready -U $POSTGRES_USER -d $POSTGRES_DB"
      "--health-interval=5s"
      "--health-retries=20"
      "--network-alias=sharkey-db"
      "--network=sharkey_default"
    ];
  };
  systemd.services."docker-sharkey-sharkey-db" = {
    serviceConfig = {
      Restart = lib.mkOverride 90 "always";
      RestartMaxDelaySec = lib.mkOverride 90 "1m";
      RestartSec = lib.mkOverride 90 "100ms";
      RestartSteps = lib.mkOverride 90 9;
    };
    after = [
      "docker-network-sharkey_default.service"
      "docker-volume-sharkey_sharkey-db.service"
    ];
    requires = [
      "docker-network-sharkey_default.service"
      "docker-volume-sharkey_sharkey-db.service"
    ];
    partOf = [
      "docker-compose-sharkey-root.target"
    ];
    wantedBy = [
      "docker-compose-sharkey-root.target"
    ];
  };
  virtualisation.oci-containers.containers."sharkey-sharkey-meilisearch" = {
    image = "getmeili/meilisearch:v1.13.0";
    environmentFiles = [
      "/persist/etc/sharkey.env"
    ];
    volumes = [
      "sharkey-meilisearch:/meili_data:rw"
    ];
    log-driver = "journald";
    extraOptions = [
      "--network-alias=sharkey-meilisearch"
      "--network=sharkey_default"
    ];
  };
  systemd.services."docker-sharkey-sharkey-meilisearch" = {
    serviceConfig = {
      Restart = lib.mkOverride 90 "always";
      RestartMaxDelaySec = lib.mkOverride 90 "1m";
      RestartSec = lib.mkOverride 90 "100ms";
      RestartSteps = lib.mkOverride 90 9;
    };
    after = [
      "docker-network-sharkey_default.service"
      "docker-volume-sharkey_sharkey-meilisearch.service"
    ];
    requires = [
      "docker-network-sharkey_default.service"
      "docker-volume-sharkey_sharkey-meilisearch.service"
    ];
    partOf = [
      "docker-compose-sharkey-root.target"
    ];
    wantedBy = [
      "docker-compose-sharkey-root.target"
    ];
  };
  virtualisation.oci-containers.containers."sharkey-sharkey-redis" = {
    image = "redis:7-alpine";
    volumes = [
      "sharkey-redis:/data:rw"
    ];
    log-driver = "journald";
    extraOptions = [
      "--health-cmd=redis-cli ping"
      "--health-interval=5s"
      "--health-retries=20"
      "--network-alias=sharkey-redis"
      "--network=sharkey_default"
    ];
  };
  systemd.services."docker-sharkey-sharkey-redis" = {
    serviceConfig = {
      Restart = lib.mkOverride 90 "always";
      RestartMaxDelaySec = lib.mkOverride 90 "1m";
      RestartSec = lib.mkOverride 90 "100ms";
      RestartSteps = lib.mkOverride 90 9;
    };
    after = [
      "docker-network-sharkey_default.service"
      "docker-volume-sharkey_sharkey-redis.service"
    ];
    requires = [
      "docker-network-sharkey_default.service"
      "docker-volume-sharkey_sharkey-redis.service"
    ];
    partOf = [
      "docker-compose-sharkey-root.target"
    ];
    wantedBy = [
      "docker-compose-sharkey-root.target"
    ];
  };

  # Networks
  systemd.services."docker-network-sharkey_default" = {
    path = [ pkgs.docker ];
    serviceConfig = {
      Type = "oneshot";
      RemainAfterExit = true;
      ExecStop = "docker network rm -f sharkey_default";
    };
    script = ''
      docker network inspect sharkey_default || docker network create sharkey_default
    '';
    partOf = [ "docker-compose-sharkey-root.target" ];
    wantedBy = [ "docker-compose-sharkey-root.target" ];
  };

  # Volumes
  systemd.services."docker-volume-sharkey_sharkey-config" = {
    path = [ pkgs.docker ];
    serviceConfig = {
      Type = "oneshot";
      RemainAfterExit = true;
    };
    script = ''
      docker volume inspect sharkey-config || docker volume create sharkey-config
    '';
    partOf = [ "docker-compose-sharkey-root.target" ];
    wantedBy = [ "docker-compose-sharkey-root.target" ];
  };
  systemd.services."docker-volume-sharkey_sharkey-db" = {
    path = [ pkgs.docker ];
    serviceConfig = {
      Type = "oneshot";
      RemainAfterExit = true;
    };
    script = ''
      docker volume inspect sharkey-db || docker volume create sharkey-db
    '';
    partOf = [ "docker-compose-sharkey-root.target" ];
    wantedBy = [ "docker-compose-sharkey-root.target" ];
  };
  systemd.services."docker-volume-sharkey_sharkey-files" = {
    path = [ pkgs.docker ];
    serviceConfig = {
      Type = "oneshot";
      RemainAfterExit = true;
    };
    script = ''
      docker volume inspect sharkey-files || docker volume create sharkey-files
    '';
    partOf = [ "docker-compose-sharkey-root.target" ];
    wantedBy = [ "docker-compose-sharkey-root.target" ];
  };
  systemd.services."docker-volume-sharkey_sharkey-meilisearch" = {
    path = [ pkgs.docker ];
    serviceConfig = {
      Type = "oneshot";
      RemainAfterExit = true;
    };
    script = ''
      docker volume inspect sharkey-meilisearch || docker volume create sharkey-meilisearch
    '';
    partOf = [ "docker-compose-sharkey-root.target" ];
    wantedBy = [ "docker-compose-sharkey-root.target" ];
  };
  systemd.services."docker-volume-sharkey_sharkey-redis" = {
    path = [ pkgs.docker ];
    serviceConfig = {
      Type = "oneshot";
      RemainAfterExit = true;
    };
    script = ''
      docker volume inspect sharkey-redis || docker volume create sharkey-redis
    '';
    partOf = [ "docker-compose-sharkey-root.target" ];
    wantedBy = [ "docker-compose-sharkey-root.target" ];
  };

  # Root service
  # When started, this will automatically create all resources and start
  # the containers. When stopped, this will teardown all resources.
  systemd.targets."docker-compose-sharkey-root" = {
    unitConfig = {
      Description = "Root target generated by compose2nix.";
    };
    wantedBy = [ "multi-user.target" ];
  };
}
