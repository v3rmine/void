FROM ghcr.io/getzola/zola:v0.22.1 AS zola

COPY . /project
WORKDIR /project
RUN ["zola", "build"]

FROM golang:1.22-alpine AS compressor
RUN apk add --no-cache build-base
RUN go install github.com/scottlaird/incremental-compress@latest
COPY --from=zola /project/public /uncached-public
ARG CACHE_BREAKER=1337
RUN --mount=type=cache,id=blog,target=/compressed \
    if [ ! -f /compressed/.cache_breaker ]; then echo "${CACHE_BREAKER}" > /compressed/.cache_breaker; fi && \
    if cat /compressed/.cache_breaker | grep -qv "${CACHE_BREAKER}"; then rm -rf /compressed/*; fi && \
    find /compressed ! -name "*.br" -o -name "*.gz" -o -name "*.zst" -type f -exec rm -i {} \; &&\
    cp -r /uncached-public/* /compressed
ARG COMPRESS=true
RUN --mount=type=cache,id=blog,target=/compressed \
    --mount=type=cache,id=blog-state,target=/state ${COMPRESS} &&\
    incremental-compress --dir=/compressed --statedir=/state/state.sqlite \
        --gzip_level=9 --zstd_level=19 --brotli_level=11 \
        --types html,css,js,json,xml,ico,svg
    RUN --mount=type=cache,id=blog,target=/compressed cp -r /compressed /public

FROM ghcr.io/static-web-server/static-web-server:2
WORKDIR /
ENV SERVER_ROOT=/public
COPY --from=compressor /public /public

ENV SERVER_COMPRESSION_STATIC=true
# CMD [ "-g=trace" ]
