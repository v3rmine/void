{ ... }: {
  services.caddy.virtualHosts = {
    "http://townsquare.astriiid.fr".extraConfig = ''
      reverse_proxy 127.0.0.1:63007
    '';
  };
}
