#!/bin/bash
name=${IMAGE_NAME:-codeserver}
with_cache=$(test -z "$NO_CACHE" && echo "" || echo '--no-cache')
# shellcheck disable=2046
nohup docker build -t "$name" $with_cache --build-arg GIT_NAME="$GIT_NAME" --build-arg GIT_EMAIL="$GIT_EMAIL" dockerfile > build-image-bg.log &
disown
