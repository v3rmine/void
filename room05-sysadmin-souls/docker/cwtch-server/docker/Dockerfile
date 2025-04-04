ARG ALPINE_VERSION=3.18

FROM alpine:${ALPINE_VERSION} AS tor-build-stage

# Install prerequisites, grab tor, compile it and move to /usr/local
RUN apk --no-cache add --update \
        gnupg \
        build-base \
        libevent \
        libevent-dev \
        libressl \
        libressl-dev \
        xz-libs \
        xz-dev \
        zlib \
        zlib-dev \
        zstd \
        zstd-dev \
      && wget -q https://dist.torproject.org/tor-0.4.8.9.tar.gz \
      && tar xf tor-0.4.8.9.tar.gz \
      && cd tor-0.4.8.9 \
      && ./configure \
      && make install \
      && ls -R /usr/local/

FROM golang:alpine${ALPINE_VERSION} AS cwtch-build-stage
# Need additional packages for cgo etc
RUN apk --no-cache add --update gcc build-base

# Copy source files from the repo to /go/src
COPY . src/

#Build Cwtch
RUN cd src/app && go build 


FROM alpine:${ALPINE_VERSION}
#Specify various env vars
ENV TOR_USER=_tor CWTCH_USER=_cwtch CWTCH_HOME=/var/lib/cwtch

# Installing dependencies of Tor
RUN apk --no-cache add --update \
      libevent \
      libressl \
      xz-libs \
      zlib \
      zstd \
      zstd-dev

# Copy Tor
COPY --from=tor-build-stage /usr/local/ /usr/local/

#Copy cwtch app
COPY --from=cwtch-build-stage /go/src/app/app /usr/local/bin/cwtch

# Create unprivileged users
RUN mkdir -p /run/tor && mkdir /var/lib/cwtch && addgroup -S $TOR_USER && adduser -G $TOR_USER -S $TOR_USER && adduser -S $CWTCH_USER

# Copy Tor configuration file
COPY ./docker/torrc /etc/tor/torrc

# Copy docker-entrypoint
COPY ./docker/docker-entrypoint /usr/local/bin/

# Persist data
VOLUME /etc/tor /var/lib/tor /var/lib/cwtch

ENTRYPOINT ["docker-entrypoint"]
CMD ["/usr/local/bin/cwtch","--exportServerBundle"]

