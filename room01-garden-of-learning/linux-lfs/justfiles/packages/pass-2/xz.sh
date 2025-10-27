#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
xz_file="$(find . -name "xz-*.tar.xz" | head -n1)"
xz_folder="$(echo "$xz_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-2"

if [ ! -d "$xz_folder" ]; then
    mkdir -vp "$xz_folder"
    tar -xvf "$xz_file" -C "$xz_folder" --strip-component 1
fi
pushd "$xz_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --host="$LFS_TGT" \
        --build="$(build-aux/config.guess)" \
        --disable-static \
        --docdir=/usr/share/doc/xz-5.8.1
    make
    make DESTDIR="$LFS" install
}

time build_n_install

rm -vf "$LFS/usr/lib/liblzma.la"
