FROM rust:slim AS build
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl
RUN apt -y update && apt install -y musl-tools upx-ucl

RUN mkdir -p /root
COPY ./sfs /root/build

WORKDIR /root/build
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN strip target/x86_64-unknown-linux-musl/release/static-file-server

RUN upx --best --lzma target/x86_64-unknown-linux-musl/release/static-file-server

FROM scratch AS latest
MAINTAINER v3rmine <v3rmine@astriiid.fr>

VOLUME /var/www
EXPOSE 80

#HEALTHCHECK CMD "if test $(curl -s -o /dev/null -I -w '%{http_code}' http://0.0.0.0:80/) -neq 404; then exit 1; fi"

COPY --from=build /root/build/target/x86_64-unknown-linux-musl/release/static-file-server /static-file-server

WORKDIR /var/www
ENTRYPOINT ["/static-file-server"]
