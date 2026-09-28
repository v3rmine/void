{ ... }: {
  services.caddy.virtualHosts = {
    "http://garage-hot.serv4.reta.re".extraConfig = ''
      reverse_proxy 127.0.0.1:63005
      @cdn {
        method GET
        path_regexp cdn_path ^/cdn/(?P<bucket>[^/]+)/.*$
      }
      handle @cdn {
        uri strip_prefix /cdn/{re.bucket}
        reverse_proxy 127.0.0.1:63006 {
          header_up Host {re.bucket}.web.garage.localhost
        }
      }
    '';
  };
}
