#!/bin/bash
set -eu

docker build . -t compose2nix/blog-hidden-website \
  --build-arg CACHE_BREAKER=1213 \
  --build-arg CACHE_ID=hidden-blog \
  --build-arg "REPLACEMENT_DOMAIN=http://astridoy2oropabiis37gjxfxrfcp7ig24gb32c6mbwuow3opstrasid.onion"
