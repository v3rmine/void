{ ... }: {
  services.caddy.virtualHosts = {
    "http://links.astriiid.fr".extraConfig = ''
      reverse_proxy 127.0.0.1:63010
    '';
  };
}
