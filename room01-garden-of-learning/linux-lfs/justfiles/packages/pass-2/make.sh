#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
make_file="$(find . -name "make-*.tar.gz" | head -n1)"
make_folder="$(echo "$make_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")-pass-2"

if [ ! -d "$make_folder" ]; then
    mkdir -vp "$make_folder"
    tar -xvf "$make_file" -C "$make_folder" --strip-component 1
fi
pushd "$make_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --host="$LFS_TGT" \
        --build="$(build-aux/config.guess)"
    make
    make DESTDIR="$LFS" install
}

time build_n_install
