FROM alpine:3.19

RUN set -euo pipefail; \
    apk --no-cache add \
      bash \
      curl \
      jq && \
    echo http://dl-2.alpinelinux.org/alpine/edge/community/ >> /etc/apk/repositories && \
    apk --no-cache -U add \
      tailscale

WORKDIR /opt/resource

COPY ./check ./in ./out /opt/resource/
