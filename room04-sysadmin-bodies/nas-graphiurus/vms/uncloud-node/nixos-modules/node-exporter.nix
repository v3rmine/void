{ ... }: {
  imports = [];
  options = {};

  config = {
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
      extraFlags = [ "--collector.disable-defaults" ];
    };
  };
}
