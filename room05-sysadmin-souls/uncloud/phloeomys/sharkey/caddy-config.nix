{ ... }: {
  services.caddy.virtualHosts = {
    "http://tracto.pl".extraConfig = ''
      reverse_proxy 127.0.0.1:63012
    '';
  };
}
