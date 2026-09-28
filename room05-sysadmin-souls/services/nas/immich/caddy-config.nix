{ ... }: {
  services.caddy.virtualHosts = {
    "http://photos.reta.re".extraConfig = ''
      reverse_proxy 127.0.0.1:63007
    '';
  };
}
