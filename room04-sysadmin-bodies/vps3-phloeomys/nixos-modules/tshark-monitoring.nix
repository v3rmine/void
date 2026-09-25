{ pkgs, lib, config, ... }:
let
  log-network-traffic = pkgs.writeShellScriptBin "log-network-traffic.sh" ''
    ${pkgs.tshark}/bin/tshark -l \
      -i eth0 \
      -T fields \
      -E header=y -E separator=, -E quote=d -E occurrence=f \
      -e _ws.col.protocol \
      -e ip.proto \
      -e ip.geoip.dst_asnum \
      -e ip.geoip.dst_org \
      -e ip.geoip.dst_country \
      -e ip.geoip.dst_city \
      -e ip.src \
      -e ip.dst \
      -e ipv6.geoip.dst_asnum \
      -e ipv6.geoip.dst_org \
      -e ipv6.geoip.dst_country \
      -e ipv6.geoip.dst_city \
      -e ipv6.src \
      -e ipv6.dst \
      -e tcp.ack \
      -e tcp.srcport \
      -e tcp.dstport \
      -e udp.srcport \
      -e udp.dstport \
      -e http.host \
      -e tls.handshake.extensions_server_name \
      -f '(src net 46.225.11.44/32) or (src net 2a01:4f8:1c19:cc7a::/64)' \
      -Y '(tls.handshake.extensions_server_name || http.host || (tcp.flags.syn==1 && tcp.flags.ack==0) || udp) && !wg && !ssdp && !stun && !nat-pmp && !(pcp || ip.proto==17)' 2>/dev/null
  '';
in {
  imports = [];
  options = {
    services.tshark-monitoring.enable = lib.mkEnableOption "Enable tshark monitoring module";
  };

  config = lib.mkIf config.services.tshark-monitoring.enable {
    environment.persistence."/persist".directories = [
      "/var/lib/GeoIP"
    ];

    systemd.services."tshark-monitoring" = {
      enable = false;
      after = [ "network-online.target" ];
      wants = [ "network-online.target" ];
      serviceConfig = {
        Type = "simple";
        ExecStart = "${log-network-traffic}/bin/log-network-traffic.sh";
        StandardOutput = "file:/var/log/logs/tshark/traffic.log";
      };
      wantedBy = [ "multi-user.target" ];
    };
  };
}
