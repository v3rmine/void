{ ... }: {
  services.caddy.virtualHosts = {
    "http://stats.astriiid.fr".extraConfig = ''
      reverse_proxy 127.0.0.1:63011
    '';
  };
}
