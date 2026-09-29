
{ pkgs, lib, ... }:

{


  # Containers
  virtualisation.oci-containers.containers."grafana" = {
    image = "grafana/grafana:latest";
    environment = {
      "GF_AUTH_ANONYMOUS_ENABLED" = "true";
      "GF_AUTH_ANONYMOUS_ORG_ROLE" = "Admin";
      "GF_FEATURE_TOGGLES_NEWLOGSPANEL" = "false";
      "GF_PATHS_PROVISIONING" = "/etc/grafana/provisioning";
      "GF_PLUGINS_PREINSTALL_SYNC" = "victoriametrics-logs-datasource@0.24.1@https://grafana.com/api/plugins/victoriametrics-logs-datasource/versions/0.24.1/download victoriametrics-metrics-datasource@0.24.0@https://grafana.com/api/plugins/victoriametrics-metrics-datasource/versions/0.24.0/download";
      "GF_RENDERING_CALLBACK_URL" = "http://grafana:3000/";
      "GF_RENDERING_SERVER_URL" = "http://grafana-image-renderer:8081/render";
      "GF_RENDERING_RENDERER_TOKEN" = "fuck-this-shit";
      "GF_SERVER_ROOT_URL" = "https://grafana.serv4.reta.re";
    };
    user = "0:0";
    volumes = [
      "grafana-data:/var/lib/grafana:rw"
      "monitoring-config:/etc/grafana/provisioning:rw"
    ];
    ports = [
      "63010:3000/tcp"
    ];
    log-driver = "journald";
    extraOptions = [
      "--network-alias=grafana"
      "--network=monitoring_default"
    ];
    rootless = { user = "podman"; };
  };
  systemd.services."docker-grafana" = {
    serviceConfig = {
      Restart = lib.mkOverride 90 "always";
      RestartMaxDelaySec = lib.mkOverride 90 "1m";
      RestartSec = lib.mkOverride 90 "100ms";
      RestartSteps = lib.mkOverride 90 9;
    };
    after = [
      "docker-network-monitoring_default.service"
      "docker-volume-monitoring_grafana-data.service"
      "docker-volume-monitoring_monitoring-config.service"
    ];
    requires = [
      "docker-network-monitoring_default.service"
      "docker-volume-monitoring_grafana-data.service"
      "docker-volume-monitoring_monitoring-config.service"
    ];
    partOf = [
      "docker-compose-monitoring-root.target"
    ];
    wantedBy = [
      "docker-compose-monitoring-root.target"
    ];
  };
  virtualisation.oci-containers.containers."grafana-image-renderer" = {
    image = "grafana/grafana-image-renderer:latest";
    environment = {
      "GOMEMLIMIT" = "1GiB";
      "AUTH_TOKEN" = "fuck-this-shit";
    };
    log-driver = "journald";
    extraOptions = [
      "--network-alias=grafana-image-renderer"
      "--network=monitoring_default"
    ];
    rootless = { user = "podman"; };
  };
  systemd.services."docker-grafana-image-renderer" = {
    serviceConfig = {
      Restart = lib.mkOverride 90 "always";
      RestartMaxDelaySec = lib.mkOverride 90 "1m";
      RestartSec = lib.mkOverride 90 "100ms";
      RestartSteps = lib.mkOverride 90 9;
    };
    after = [
      "docker-network-monitoring_default.service"
    ];
    requires = [
      "docker-network-monitoring_default.service"
    ];
    partOf = [
      "docker-compose-monitoring-root.target"
    ];
    wantedBy = [
      "docker-compose-monitoring-root.target"
    ];
  };
  virtualisation.oci-containers.containers."monitoring-vmagent" = {
    image = "victoriametrics/vmagent:latest";
    volumes = [
      "monitoring-config:/etc/prometheus:rw"
      "victoriagent-data:/vmagentdata:rw"
    ];
    cmd = [ "--promscrape.config=/etc/prometheus/prometheus.yml" "--remoteWrite.url=http://victoriametrics:8428/api/v1/write" ];
    dependsOn = [
      "victoriametrics"
    ];
    log-driver = "journald";
    extraOptions = [
      "--network-alias=vmagent"
      "--network=monitoring_default"
    ];
    rootless = { user = "podman"; };
  };
  systemd.services."docker-monitoring-vmagent" = {
    serviceConfig = {
      Restart = lib.mkOverride 90 "always";
      RestartMaxDelaySec = lib.mkOverride 90 "1m";
      RestartSec = lib.mkOverride 90 "100ms";
      RestartSteps = lib.mkOverride 90 9;
    };
    after = [
      "docker-network-monitoring_default.service"
      "docker-volume-monitoring_monitoring-config.service"
      "docker-volume-monitoring_victoriagent-data.service"
    ];
    requires = [
      "docker-network-monitoring_default.service"
      "docker-volume-monitoring_monitoring-config.service"
      "docker-volume-monitoring_victoriagent-data.service"
    ];
    partOf = [
      "docker-compose-monitoring-root.target"
    ];
    wantedBy = [
      "docker-compose-monitoring-root.target"
    ];
  };
  virtualisation.oci-containers.containers."victorialogs" = {
    image = "victoriametrics/victoria-logs:latest";
    volumes = [
      "victorialogs-data:/victoria-logs-data:rw"
    ];
    ports = [
      "9428:9428/tcp"
    ];
    cmd = [ "-storageDataPath=victoria-logs-data" "-retentionPeriod=1w" ];
    log-driver = "journald";
    extraOptions = [
      "--network-alias=victorialogs"
      "--network=monitoring_default"
    ];
    rootless = { user = "podman"; };
  };
  systemd.services."docker-victorialogs" = {
    serviceConfig = {
      Restart = lib.mkOverride 90 "always";
      RestartMaxDelaySec = lib.mkOverride 90 "1m";
      RestartSec = lib.mkOverride 90 "100ms";
      RestartSteps = lib.mkOverride 90 9;
    };
    after = [
      "docker-network-monitoring_default.service"
      "docker-volume-monitoring_victorialogs-data.service"
    ];
    requires = [
      "docker-network-monitoring_default.service"
      "docker-volume-monitoring_victorialogs-data.service"
    ];
    partOf = [
      "docker-compose-monitoring-root.target"
    ];
    wantedBy = [
      "docker-compose-monitoring-root.target"
    ];
  };
  virtualisation.oci-containers.containers."victoriametrics" = {
    image = "victoriametrics/victoria-metrics:latest";
    volumes = [
      "victoriametrics-data:/storage:rw"
    ];
    ports = [
      "8428:8428/tcp"
    ];
    cmd = [ "-storageDataPath=/storage" "-httpListenAddr=:8428" "-retentionPeriod=1w" ];
    log-driver = "journald";
    extraOptions = [
      "--network-alias=victoriametrics"
      "--network=monitoring_default"
    ];
    rootless = { user = "podman"; };
  };
  systemd.services."docker-victoriametrics" = {
    serviceConfig = {
      Restart = lib.mkOverride 90 "always";
      RestartMaxDelaySec = lib.mkOverride 90 "1m";
      RestartSec = lib.mkOverride 90 "100ms";
      RestartSteps = lib.mkOverride 90 9;
    };
    after = [
      "docker-network-monitoring_default.service"
      "docker-volume-monitoring_victoriametrics-data.service"
    ];
    requires = [
      "docker-network-monitoring_default.service"
      "docker-volume-monitoring_victoriametrics-data.service"
    ];
    partOf = [
      "docker-compose-monitoring-root.target"
    ];
    wantedBy = [
      "docker-compose-monitoring-root.target"
    ];
  };

  # Networks
  systemd.services."docker-network-monitoring_default" = {
    path = [ pkgs.docker ];
    serviceConfig = {
      Type = "oneshot";
      RemainAfterExit = true;
      ExecStop = "docker network rm -f monitoring_default";
    };
    script = ''
      docker network inspect monitoring_default || docker network create monitoring_default
    '';
    partOf = [ "docker-compose-monitoring-root.target" ];
    wantedBy = [ "docker-compose-monitoring-root.target" ];
  };

  # Volumes
  systemd.services."docker-volume-monitoring_grafana-data" = {
    path = [ pkgs.docker ];
    serviceConfig = {
      Type = "oneshot";
      RemainAfterExit = true;
    };
    script = ''
      docker volume inspect grafana-data || docker volume create grafana-data
    '';
    partOf = [ "docker-compose-monitoring-root.target" ];
    wantedBy = [ "docker-compose-monitoring-root.target" ];
  };
  systemd.services."docker-volume-monitoring_monitoring-config" = {
    path = [ pkgs.docker ];
    serviceConfig = {
      Type = "oneshot";
      RemainAfterExit = true;
    };
    script = ''
      docker volume inspect monitoring-config || docker volume create monitoring-config
    '';
    partOf = [ "docker-compose-monitoring-root.target" ];
    wantedBy = [ "docker-compose-monitoring-root.target" ];
  };
  systemd.services."docker-volume-monitoring_victoriagent-data" = {
    path = [ pkgs.docker ];
    serviceConfig = {
      Type = "oneshot";
      RemainAfterExit = true;
    };
    script = ''
      docker volume inspect victoriagent-data || docker volume create victoriagent-data
    '';
    partOf = [ "docker-compose-monitoring-root.target" ];
    wantedBy = [ "docker-compose-monitoring-root.target" ];
  };
  systemd.services."docker-volume-monitoring_victorialogs-data" = {
    path = [ pkgs.docker ];
    serviceConfig = {
      Type = "oneshot";
      RemainAfterExit = true;
    };
    script = ''
      docker volume inspect victorialogs-data || docker volume create victorialogs-data
    '';
    partOf = [ "docker-compose-monitoring-root.target" ];
    wantedBy = [ "docker-compose-monitoring-root.target" ];
  };
  systemd.services."docker-volume-monitoring_victoriametrics-data" = {
    path = [ pkgs.docker ];
    serviceConfig = {
      Type = "oneshot";
      RemainAfterExit = true;
    };
    script = ''
      docker volume inspect victoriametrics-data || docker volume create victoriametrics-data
    '';
    partOf = [ "docker-compose-monitoring-root.target" ];
    wantedBy = [ "docker-compose-monitoring-root.target" ];
  };

  # Root service
  # When started, this will automatically create all resources and start
  # the containers. When stopped, this will teardown all resources.
  systemd.targets."docker-compose-monitoring-root" = {
    unitConfig = {
      Description = "Root target generated by compose2nix.";
    };
    wantedBy = [ "multi-user.target" ];
  };
}
