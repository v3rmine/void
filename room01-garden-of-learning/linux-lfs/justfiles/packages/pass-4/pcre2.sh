#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
pcre2_file="$(find . -name "pcre2-*.tar.bz2" | head -n1)"
pcre2_folder="$(echo "$pcre2_file" | sed -E "s/(^\.\/|\.tar\.bz2)//g")-pass-4"
pcre2_version="$(echo "$pcre2_folder" | cut -d'-' -f2)"

if [ ! -d "$pcre2_folder" ]; then
    mkdir -vp "$pcre2_folder"
    tar -xvf "$pcre2_file" -C "$pcre2_folder" --strip-component 1
fi
pushd "$pcre2_folder"

build_n_install() {
    set -x

    ./configure \
        --prefix=/usr \
        --docdir="/usr/share/doc/pcre2-$pcre2_version" \
        --enable-unicode \
        --enable-jit \
        --enable-pcre2-16 \
        --enable-pcre2-32 \
        --enable-pcre2grep-libz \
        --enable-pcre2grep-libbz2 \
        --enable-pcre2test-libreadline \
        --disable-static

    make
    make check
    make install
}

time build_n_install
