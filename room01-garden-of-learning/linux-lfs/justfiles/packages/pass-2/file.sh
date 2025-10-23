#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
file_file="$(find . -name "file-*.tar.gz" | head -n1)"
file_folder="$(echo "$file_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")-pass-2"

if [ ! -d "$file_folder" ]; then
    mkdir -vp "$file_folder"
    tar -xvf "$file_file" -C "$file_folder" --strip-component 1
fi
pushd "$file_folder"

mkdir -vp build

build_n_install() {
    set -x
    pushd build
        ../configure \
            --disable-bzlib \
            --disable-libseccomp \
            --disable-xzlib \
            --disable-zlib
        make
    popd

    ./configure \
        --prefix=/usr \
        --host="$LFS_TGT" \
        --build="$(./config.guess)"
    make FILE_COMPILE="$(pwd)/build/src/file"
    make DESTDIR="$LFS" install
}

time build_n_install
rm -v "$LFS/usr/lib/libmagic.la"
