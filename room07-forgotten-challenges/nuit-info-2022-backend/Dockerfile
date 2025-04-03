FROM rust:alpine as builder

RUN apk add --no-cache \
	musl-dev

WORKDIR /app

# Copying sources in order of less updated
COPY xtask /app/xtask
COPY .cargo /app/.cargo
COPY Cargo.toml Cargo.lock LICENSE /app/
COPY bin /app/bin
COPY services /app/services

# Build the project
RUN cargo build --release --locked --target=x86_64-unknown-linux-musl

# We use busybox because it's smaller than alpine
# But it does not suffer from the same issues as sratch
# Because it is a really minimal linux
FROM busybox:musl

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/sis-server /usr/local/bin/sis-server
COPY --from=builder /app/LICENSE /LICENSE

ENV HOST=0.0.0.0
ENV LOG_DIRECTORY=/var/log/sis-server

ENTRYPOINT ["/usr/local/bin/sis-server"]