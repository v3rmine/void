FROM rust:1.61.0-alpine
WORKDIR /app
VOLUME /app

RUN rustup component add rust-src
RUN rustup component add clippy
RUN rustup component add rustfmt

RUN apk add --no-cache -X http://dl-cdn.alpinelinux.org/alpine/edge/testing \
	mold \
	musl-dev \
	clang \
	pkgconfig \
	openssl-dev \
	git \
	lld

RUN cargo install cargo-edit

# Rust analyzer
RUN git clone https://github.com/rust-lang/rust-analyzer.git --depth 1 --branch 2022-05-30
RUN cd rust-analyzer && cargo xtask install --server
RUN rm -rf rust-analyzer

# osxcross
COPY .res/MacOSX11.1.sdk.tar.xz /etc
RUN rustup target add x86_64-apple-darwin
RUN git clone https://github.com/tpoechtrager/osxcross --depth 1 /etc/osxcross
RUN apk add --no-cache -X http://dl-cdn.alpinelinux.org/alpine/edge/testing \
	bash \
	make \
	cmake \
	patch \
	libxml2-dev \
	fts-dev \
	build-base \
	clang-dev \
	bsd-compat-headers \
	python3 \
	xz
RUN cd /etc/osxcross && \
	cp /etc/MacOSX11.1.sdk.tar.xz tarballs/ && \
	UNATTENDED=yes OSX_VERSION_MIN=10.11 ./build.sh
ENV PATH=$PATH:/etc/osxcross/target/bin

# Windows
RUN rustup target add x86_64-pc-windows-gnu
RUN apk add --no-cache -X http://dl-cdn.alpinelinux.org/alpine/edge/testing mingw-w64-gcc