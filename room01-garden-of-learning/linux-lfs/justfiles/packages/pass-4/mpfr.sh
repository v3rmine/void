#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
mpfr_file="$(find . -name "mpfr-*.tar.xz" | head -n1)"
mpfr_folder="$(echo "$mpfr_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-4"
mpfr_version="$(echo "$mpfr_folder" | cut -d'-' -f2)"

if [ ! -d "$mpfr_folder" ]; then
    mkdir -vp "$mpfr_folder"
    tar -xvf "$mpfr_file" -C "$mpfr_folder" --strip-component 1
fi
pushd "$mpfr_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --disable-static \
        --enable-thread-safe \
        --docdir="/usr/share/doc/mpfr-$mpfr_version"
    make
    make html
    make check

    make install
    make install-html
}

time build_n_install
