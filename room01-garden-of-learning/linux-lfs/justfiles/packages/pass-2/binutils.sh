#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
binutils_file="$(find . -name "binutils-*.tar.xz" | head -n1)"
binutils_folder="$(echo "$binutils_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-2"

if [ ! -d "$binutils_folder" ]; then
    mkdir -vp "$binutils_folder"
    tar -xvf "$binutils_file" -C "$binutils_folder" --strip-component 1
fi
pushd "$binutils_folder"

sed '6031s/$add_dir//' -i ltmain.sh

mkdir -vp build
pushd build

build_n_install() {
    set -x
    ../configure \
        --prefix=/usr \
        --build="$(../config.guess)" \
        --host="$LFS_TGT" \
        --disable-nls \
        --enable-shared \
        --enable-gprofng=no \
        --disable-werror \
        --enable-64-bit-bfd \
        --enable-new-dtags \
        --enable-default-hash-style=gnu
    make
    make DESTDIR="$LFS" install
}

time build_n_install

rm -v "$LFS"/usr/lib/lib{bfd,ctf,ctf-nobfd,opcodes,sframe}.{a,la}
