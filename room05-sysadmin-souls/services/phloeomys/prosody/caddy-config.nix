{ ... }: {
  services.caddy.virtualHosts = {
    "http://files.astriiid.fr".extraConfig = ''
      handle_path /~xmpp* {
        reverse_proxy 127.0.0.1:63013
      }
      handle {
        redir https://astriiid.fr/about permanent
      }
    '';
    "http://muc.astriiid.fr".extraConfig = ''
      redir https://astriiid.fr/about permanent
    '';
  };
}
