{ ... }: {
  services.caddy.virtualHosts = {
    "http://mail-archive.reta.re".extraConfig = ''
      reverse_proxy 127.0.0.1:63001
    '';
  };
}
