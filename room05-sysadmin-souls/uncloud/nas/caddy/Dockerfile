ARG CADDY_VERSION=2.10.2
FROM caddy:${CADDY_VERSION}-builder AS builder
RUN xcaddy build \
    --with github.com/caddyserver/cache-handler@v0.16.0

FROM caddy:${CADDY_VERSION}
COPY --from=builder /usr/bin/caddy /usr/bin/caddy
