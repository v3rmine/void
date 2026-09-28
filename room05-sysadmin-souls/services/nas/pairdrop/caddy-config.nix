{ ... }: {
  services.caddy.virtualHosts = {
    "http://drop.reta.re".extraConfig = ''
      reverse_proxy 127.0.0.1:63008
    '';
  };
}
