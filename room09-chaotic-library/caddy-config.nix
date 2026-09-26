{ ... }: {
  services.caddy.virtualHosts = {
    "http://astriiid.fr".extraConfig = ''
      header Strict-Transport-Security "max-age=300; includeSubDomains"
      header X-Content-Type-Options "nosniff"
      # header X-Frame-Options "DENY"
      header >Via "Rodents"
      header Access-Control-Allow-Origin "*"

      # First script-src hash = Show javascript only elem
      # Second script-src-hash = Wander inline script
      # Third script-src-hash = Comments
      # Fourth and Fifth = Townsquare
      # Removed `frame-ancestors 'none';` for wander
      # Replaced `frame-src https://eldritch.cafe https://open-web-calendar.hosted.quelltext.eu https://invidious.nerdvpn.de https://www.youtube-nocookie.com;` for wander
      header Content-Security-Policy "
        default-src 'self';
        font-src 'self' https://fonts.gstatic.com;
        img-src 'self' https: data: blob:;
        media-src 'self' https:;
        script-src
          'self'
          'sha256-N9chTn5S0hU62Hotw4Ml5LKoceaZ+S+9kh1qlOAjuQM='
          'sha256-1UjZYT1Mt8tj/wXW6L7FetsiNhsuGixA39BHZl9UrFY='
          'sha256-FGB0jyFRAyYNOaypGhfga4z3h+se4555PKwAKnRPOPE='
          https://townsquare.astriiid.fr
          'sha256-AXhdoKkoIcvaLl8AwagXLb3ZnRPhWG9Ub6Ql/i8wTEo=';
        worker-src 'self' blob:;
        style-src 'self' 'unsafe-inline' https://fonts.googleapis.com https://townsquare.astriiid.fr;
        connect-src 'self' https://eldritch.cafe https://tracto.pl https://git.v3rm.in https://stats.astriiid.fr https://townsquare.astriiid.fr wss://townsquare.astriiid.fr;
        frame-src 'self' https:;
        base-uri 'none';
        form-action 'none'"

      redir /feed.rss /rss.xml permanent

      reverse_proxy 127.0.0.1:63001
    '';

    "http://qr.astriiid.fr".extraConfig = ''
      @mine expression {http.request.uri.query.data}.startsWith("https://eldritch.cafe/@v3rmine/") || {http.request.uri.query.data}.startsWith("https://tracto.pl/notes/")
      handle @mine {
        cache {
          key {
            hide
            template QR-{http.request.uri.query.data}
          }
          mode bypass_request
          stale 7d
          ttl 1d
        }
        reverse_proxy https://api.qrserver.com {
          header_up Host "api.qrserver.com"
          header_up -X-Forwarded-*
        }
      }
      respond "Access denied" 403 {
        close
      }
    '';

    "v3rm.in".extraConfig = ''
      redir https://astriiid.fr{uri} permanent
    '';
  };
}
