#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
iana_etc_file="$(find . -name "iana-etc-*.tar.gz" | head -n1)"
iana_etc_folder="$(echo "$iana_etc_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")-pass-4"

if [ ! -d "$iana_etc_folder" ]; then
    mkdir -vp "$iana_etc_folder"
    tar -xvf "$iana_etc_file" -C "$iana_etc_folder" --strip-component 1
fi
pushd "$iana_etc_folder"

build_n_install() {
    set -x
    cp services protocols /etc
}

time build_n_install
