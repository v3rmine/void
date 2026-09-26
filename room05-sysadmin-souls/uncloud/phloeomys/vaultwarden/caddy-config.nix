{ ... }: {
  services.caddy.virtualHosts = {
    "http://vault.reta.re".extraConfig = ''
      reverse_proxy 127.0.0.1:63006
    '';
  };
}
