FROM alpine AS patcher
COPY . /project
WORKDIR /project
ARG REPLACEMENT_DOMAIN=
RUN if [ ! -z "${REPLACEMENT_DOMAIN}" ]; then sed -iE 's|^base_url.*|base_url = "http://kkpos3jxlzmovsxu4tpdj2rgfu5hwprefyyyxbmzmtjh5vcnltxfiiid.onion/"|' config.toml; fi

FROM ghcr.io/getzola/zola:v0.22.1 AS zola
COPY --from=patcher /project /project
WORKDIR /project
RUN ["zola", "build"]

FROM golang:1.22-alpine AS compressor
RUN apk add --no-cache build-base
RUN go install github.com/scottlaird/incremental-compress@latest
COPY --from=zola /project/public /uncached-public
ARG CACHE_BREAKER=1337
ARG CACHE_ID=blog
# find /compressed ! -name "*.br" -o -name "*.gz" -o -name "*.zst" -type f -exec rm -i {} \; fi &&\
RUN --mount=type=cache,id="${CACHE_ID}",target=/compressed \
    if [ ! -f /compressed/.cache_breaker ]; then echo "${CACHE_BREAKER}" > /compressed/.cache_breaker; fi && \
    if cat /compressed/.cache_breaker | grep -qv "${CACHE_BREAKER}"; then \
        rm -rf /compressed/* && \
        echo "${CACHE_BREAKER}" > /compressed/.cache_breaker; \
    fi && \
    cp -r /uncached-public/* /compressed
ARG COMPRESS=true
RUN --mount=type=cache,id="${CACHE_ID}",target=/compressed \
    --mount=type=cache,id="${CACHE_ID}-state",target=/state ${COMPRESS} &&\
    incremental-compress --dir=/compressed --statedir=/state/state.sqlite \
        --gzip_level=9 --zstd_level=19 --brotli_level=11 \
        --types html,css,js,json,xml,ico,svg
    RUN --mount=type=cache,id="${CACHE_ID}",target=/compressed cp -r /compressed /public

FROM ghcr.io/static-web-server/static-web-server:2
WORKDIR /
ENV SERVER_ROOT=/public
COPY --from=compressor /public /public

ENV SERVER_COMPRESSION_STATIC=true
# CMD [ "-g=trace" ]
