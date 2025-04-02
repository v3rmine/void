FROM golang:1.18 as build-tools

WORKDIR /usr/src
# RUN apt-get install -y git
RUN git clone https://git.unix.lgbt/diamondburned/sysmet app
WORKDIR /usr/src/app
RUN mkdir ./dist

ENV GOOS=linux CGO_ENABLED=1

# RUN go build -ldflags='-linkmode external -extldflags -static' -v -o ./dist ./cmd/*
RUN go build -v -o ./dist ./cmd/*
RUN ls ./dist

VOLUME /build-assets
CMD cp ./dist/* /build-assets

FROM debian:bullseye-slim as server

WORKDIR /app
COPY --from=build-tools /usr/src/app/dist/sysmet-http /app/sysmet-http

VOLUME /app/db
EXPOSE 5000
CMD /app/sysmet-http -db /app/db/database 0.0.0.0:5000
