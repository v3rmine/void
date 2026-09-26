#!/bin/bash
set -eu

docker build . -t compose2nix/blog-personal-website \
  --build-arg CACHE_BREAKER=1312131
