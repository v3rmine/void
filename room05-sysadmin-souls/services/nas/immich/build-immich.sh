#!/bin/bash
source .env
docker build . \
  -t compose2nix/immich_server:latest \
  --build-arg "IMMICH_VERSION=${IMMICH_VERSION:-release}"
