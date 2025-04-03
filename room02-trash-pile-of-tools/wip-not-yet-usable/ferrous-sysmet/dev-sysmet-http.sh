#!/bin/bash
installing=false
if ! command -v cargo-watch >/dev/null; then
	installing=true
	echo "cargo-watch not found, installing..."
	cargo install cargo-watch &
fi
if ! command -v yarn >/dev/null; then
	installing=true
	echo "yarn not found, installing..."
	npm i -g yarn &
fi
if $installing; then wait; fi

trap 'kill $CSS_WATCHER 2>/dev/null && echo css watcher gracefull shutdown; kill $BROWSER_SYNC 2>/dev/null && echo browser-sync gracefull shutdown; exit' INT
root="bin/sysmet-http"

yarn run serve >/dev/null &
BROWSER_SYNC=$!

cd "$root/css" && yarn run watch >/dev/null &
CSS_WATCHER=$!

cargo watch \
  -c \
	--no-gitignore \
	--watch "$root/src" \
	--watch "$root/Cargo.toml" \
	--watch "$root/css/exports" \
	--watch "$root/../../services" \
	-- cargo-clif run -p sysmet-http -- --db test.db