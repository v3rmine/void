#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
gawk_file="$(find . -name "gawk-*.tar.xz" | head -n1)"
gawk_folder="$(echo "$gawk_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")"

if [ ! -d "$gawk_folder" ]; then
    mkdir -vp "$gawk_folder"
    tar -xvf "$gawk_file" -C "$gawk_folder" --strip-component 1
fi
pushd "$gawk_folder"

sed -i 's/extras//' Makefile.in

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
