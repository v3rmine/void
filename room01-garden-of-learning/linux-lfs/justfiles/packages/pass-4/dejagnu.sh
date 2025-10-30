#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
dejagnu_file="$(find . -name "dejagnu-*.tar.gz" | head -n1)"
dejagnu_folder="$(echo "$dejagnu_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")-pass-4"
dejagnu_version="$(echo "$dejagnu_folder" | cut -d'-' -f2)"

if [ ! -d "$dejagnu_folder" ]; then
    mkdir -vp "$dejagnu_folder"
    tar -xvf "$dejagnu_file" -C "$dejagnu_folder" --strip-component 1
fi
pushd "$dejagnu_folder"

mkdir -vp build
pushd build

build_n_install() {
    set -x
    ../configure --prefix=/usr

    makeinfo \
        --html \
        --no-split \
        -o doc/dejagnu.html \
        ../doc/dejagnu.texi
    makeinfo --plaintext \
        -o doc/dejagnu.txt \
        ../doc/dejagnu.texi

    make check
    make install

    install -v -dm755 "/usr/share/doc/dejagnu-$dejagnu_version"
    install -v -m644 doc/dejagnu.{html,txt} "/usr/share/doc/dejagnu-$dejagnu_version"
}

time build_n_install
