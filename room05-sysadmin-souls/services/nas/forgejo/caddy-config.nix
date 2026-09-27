{ ... }: {
  services.caddy.virtualHosts = {
    "http://git.v3rm.in".extraConfig = ''
      reverse_proxy 127.0.0.1:63003
    '';
  };
}
