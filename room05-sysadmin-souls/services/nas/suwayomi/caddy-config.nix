{ ... }: {
  services.caddy.virtualHosts = {
    "http://suwayomi.serv4.reta.re".extraConfig = ''
      reverse_proxy 127.0.0.1:63009
    '';
  };
}
