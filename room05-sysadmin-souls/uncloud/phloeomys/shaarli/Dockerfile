ARG VERSION

FROM alpine AS cloner
RUN apk add git
RUN git clone https://github.com/v3rmine/shaarli-stack.git /tmp/shaarli-stack

FROM ghcr.io/shaarli/shaarli:${VERSION}

COPY --from=cloner /tmp/shaarli-stack/stack /var/www/shaarli/tpl/stack
