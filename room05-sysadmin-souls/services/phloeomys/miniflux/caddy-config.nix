{ ... }: {
  services.caddy.virtualHosts = {
    "http://rss.reta.re".extraConfig = ''
      reverse_proxy 127.0.0.1:63005
    '';
  };
}
