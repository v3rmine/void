FROM nixos/nix:2.31.2 AS builder

# Because we'll be doing some cursed things to the store
RUN cp -R "$(readlink -f "$(which sh)")" /bin/bash
SHELL ["/bin/bash", "-c"]

# Clean cached dir
RUN --mount=type=cache,id=store,target=/nix/cached-store rm -rf /nix/cached-store/*

# Bye bye nix store
RUN --mount=type=cache,id=store,target=/nix/cached-store mv /nix/store/* /nix/cached-store

# Now we must provide the cache on each command c: but at least build artifacts are cached
RUN --mount=type=cache,id=store,target=/nix/store nix-channel --update

# Copy our source and setup our working dir.
RUN --mount=type=cache,id=store,target=/nix/store mkdir -p /tmp/build
COPY Cargo.lock Cargo.toml flake.nix flake.lock /tmp/build/
WORKDIR /tmp/build

# Cache deps, expected to fail because src and migration are not yet copied
RUN --mount=type=cache,id=store,target=/nix/store \
  nix \
  --extra-experimental-features "nix-command flakes" \
  --option filter-syscalls false \
  build || true

COPY src /tmp/build/src
COPY migration /tmp/build/migration

# Build
RUN --mount=type=cache,id=store,target=/nix/store \
  nix \
  --extra-experimental-features "nix-command flakes" \
  --option filter-syscalls false \
  build

# Copy the Nix store closure into a directory. The Nix store closure is the
# entire set of Nix store values that we need for our build.
RUN --mount=type=cache,id=store,target=/nix/store \
  mkdir /tmp/nix-store-closure && \
  cp -R $(nix-store -qR result/) /tmp/nix-store-closure

# Prepare result dir
RUN --mount=type=cache,id=store,target=/nix/store \
  mkdir -p /tmp/result && \
  mv result /tmp/result/app

# Final image is based on scratch. We copy a bunch of Nix dependencies
# but they're fully self-contained so we don't need Nix anymore.
FROM scratch

# Copy /nix/store
COPY --from=builder /tmp/nix-store-closure /nix/store
COPY --from=builder /tmp/result /

WORKDIR /app

VOLUME /app/assets
VOLUME /app/config
VOLUME /app/contents

# Copy runtime assets
COPY assets /app/assets
COPY config /app/config
COPY contents /app/contents

CMD ["/app/bin/blog-cli", "start", "--server-and-worker"]
