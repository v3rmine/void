#!/usr/bin/env bash
source "$stdenv/setup"

header "exporting $url (rev $rev) into $out"
$SHELL "$fetcher" --builder --url "$url" --out "$out" --rev "$rev"

runHook postFetch
stopNest

