#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
findutils_file="$(find . -name "findutils-*.tar.xz" | head -n1)"
findutils_folder="$(echo "$findutils_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-2"

if [ ! -d "$findutils_folder" ]; then
    mkdir -vp "$findutils_folder"
    tar -xvf "$findutils_file" -C "$findutils_folder" --strip-component 1
fi
pushd "$findutils_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --localstatedir=/var/lib/locate \
        --host="$LFS_TGT" \
        --build="$(build-aux/config.guess)"
    make
    make DESTDIR="$LFS" install
}

time build_n_install
