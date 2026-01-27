FROM ghcr.io/getzola/zola:v0.22.1 as zola

COPY . /project
WORKDIR /project
RUN ["zola", "build"]

FROM ghcr.io/static-web-server/static-web-server:2
WORKDIR /
ENV SERVER_ROOT=/public
COPY --from=zola /project/public /public
