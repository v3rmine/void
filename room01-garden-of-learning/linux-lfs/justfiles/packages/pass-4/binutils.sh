#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
binutils_file="$(find . -name "binutils-*.tar.xz" | head -n1)"
binutils_folder="$(echo "$binutils_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-4"

if [ ! -d "$binutils_folder" ]; then
    mkdir -vp "$binutils_folder"
    tar -xvf "$binutils_file" -C "$binutils_folder" --strip-component 1
fi
pushd "$binutils_folder"

mkdir -vp build
pushd build

build_n_install() {
    set -x
    ../configure \
        --prefix=/usr \
        --sysconfdir=/etc \
        --enable-ld=default \
        --enable-plugins \
        --enable-shared \
        --disable-werror \
        --enable-64-bit-bfd \
        --enable-new-dtags \
        --with-system-zlib \
        --enable-default-hash-style=gnu
    make tooldir=/usr

    make -k check
    # shellcheck disable=SC2046
    if grep -q '^FAIL:' $(find . -name '*.log'); then
        echo "[ERROR]: Failed binutils tests"
        grep '^FAIL:' $(find . -name '*.log')
        exit 1
    fi

    make tooldir=/usr install
}

time build_n_install

rm -rfv /usr/lib/lib{bfd,ctf,ctf-nobfd,gprofng,opcodes,sframe}.a \
    /usr/share/doc/gprofng/
