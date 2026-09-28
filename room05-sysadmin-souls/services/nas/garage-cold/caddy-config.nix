{ ... }: {
  services.caddy.virtualHosts = {
    "http://garage-cold.serv4.reta.re".extraConfig = ''
      reverse_proxy 127.0.0.1:63004
    '';
  };
}
