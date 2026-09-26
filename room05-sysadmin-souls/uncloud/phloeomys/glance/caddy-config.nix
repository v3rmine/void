{ ... }: {
  services.caddy.virtualHosts = {
    "http://dash.reta.re".extraConfig = ''
      reverse_proxy 127.0.0.1:63009
    '';
  };
}
