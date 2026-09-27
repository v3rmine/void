{ ... }: {
  services.caddy.virtualHosts = {
    "http://notes.astriiid.fr".extraConfig = ''
      reverse_proxy 127.0.0.1:63008
    '';

    "http://notes-oauth.astriiid.fr".extraConfig = ''
      @masto_oauth {
        query code=*
        path /oauth:mastodon
      }

      redir @masto_oauth https://notes.astriiid.fr{path}:{query.code} temporary
      redir https://notes.astriiid.fr permanent
    '';
  };
}
