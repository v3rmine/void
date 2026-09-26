{ ... }: {
  services.caddy.virtualHosts = {
    "http://remote.reta.re".extraConfig = ''
      reverse_proxy 127.0.0.1:63004
    '';
  };
}
