{ pkgs, config, lib, ... }: {
  imports = [];
  options = {};

  config = {
    environment.persistence."/persist".directories = [
      "/var/lib/private/vector"
    ];

    services.logrotate = {
      enable = true;
      settings = {
        "/var/log/logs/traefik/access.log" = {
          frequency = "daily";
          maxsize = "50M"; # Rotate when size reach 50MB
          rotate = 5; # Keep the last 5 version for ipset
          missingok = true; # Ignore if file is missing
          postrotate = ''
            ${pkgs.systemd}/bin/systemctl start traefik-logrotate
          '';
          nocompress = true; # Do not compress for postprocess by ipset
          notifempty = true; # Do not rotate if empty
        };
        "/var/log/logs/tshark/traffic.log" = {
          frequency = "daily";
          maxsize = "50M"; # Rotate when size reach 50MB
          rotate = 1; # Keep the last version
          missingok = true; # Ignore if file is missing
          postrotate = ''
            ${pkgs.systemd}/bin/systemctl restart tshark-monitoring
          '';
          nocompress = true; # Do not compress for postprocess by ipset
          notifempty = true; # Do not rotate if empty
        };
      };
    };

    services.vector = {
      enable = false;
      journaldAccess = false;
      settings = {
        api.enabled = true;
        data_dir = "/var/lib/private/vector";
        sources = {
          journald.type = "journald";
          traefik = {
            type = "file";
            include = [ "/var/log/logs/traefik/access.log" ];
            fingerprint.strategy = "device_and_inode";
            rotate_wait_secs = 30;
          };
        };

        transforms = {
          traefik_json = {
            type = "remap";
            inputs = [ "traefik" ];
            source = ''
              .source = "phloeomys_traefik"
              result, err = parse_json(string!(.message))
              if err == null {
                .data = result
                request_method = string!(result.RequestMethod)
                request_host, err = to_string(result.RequestHost)
                downstream_status = to_string(int!(result.DownstreamStatus))
                request_path = string!(result.RequestPath)
                .message = "{{ request_method }} {{ request_host }} - {{ downstream_status }} {{ request_path }}"
              } else {
                .vector_err = err
              }
            '';
          };
          journald_json = {
            type = "remap";
            inputs = [ "journald" ];
            source = ''
              .source = "phloeomys_journald"
              result, err = parse_json(string!(.message))
              if err == null {
                .data = result
                .message = result._msg
              } else {
                .vector_err = err
              }
            '';
          };
        };

        sinks = {
          victorialogs_journald = {
            type = "http";
            inputs = [ "journald_json" ];
            uri = "http://nas-uncloud:9428/insert/jsonline?_stream_fields=host,source_type,CONTAINER_NAME,source&_msg_field=message&_time_field=timestamp";
            request.headers = {
              VL-Extra-Fields = "source=phloeomys_journald";
            };
            encoding = { codec = "json"; };
            compression = "gzip";
            framing = { method = "newline_delimited"; };
            healthcheck = { enabled = false; };
            buffer = { type = "disk"; max_size = 512 * 1024 * 1024; when_full = "drop_newest"; };
          };
          victorialogs_traefik = {
            type = "http";
            inputs = [ "traefik_json" ];
            uri = "http://nas-uncloud:9428/insert/jsonline?_stream_fields=host,source_type,source&_msg_field=message&_time_field=timestamp";
            request.headers = {
              VL-Extra-Fields = "source=phloeomys_traefik";
            };
            encoding = { codec = "json"; };
            compression = "gzip";
            framing = { method = "newline_delimited"; };
            healthcheck = { enabled = false; };
            buffer = { type = "disk"; max_size = 512 * 1024 * 1024; when_full = "drop_newest"; };
          };
        };
      };
    };
    systemd.services."vector".serviceConfig = {
      AmbientCapabilities = lib.mkForce "CAP_NET_BIND_SERVICE CAP_DAC_READ_SEARCH";
      CapabilityBoundingSet = "CAP_DAC_READ_SEARCH";
    };
    systemd.services."logrotate".serviceConfig = {
      PrivateNetwork = lib.mkForce false;
      RestrictAddressFamilies = lib.mkForce "AF_UNIX";
      BindPaths = "/run/systemd/private /run/dbus/system_bus_socket";
    };
    systemd.services."traefik-logrotate" = {
      serviceConfig = {
        Type = "oneshot";
        ExecStart = "${pkgs.docker}/bin/docker kill -s USR1 traefik";
      };
    };

    services.prometheus.exporters.node = {
      enable = true;
      port = 9000;
      enabledCollectors = [
        "diskstats"
        "filesystem"
        "loadavg"
        "meminfo"
        "netdev"
        "netstat"
        "processes"
        "tcpstat"
      ];
      extraFlags = [ "--collector.disable-defaults" "--collector.textfile.directory='/persist/etc/node-exporter/textfile'" ];
    };
  };
}
