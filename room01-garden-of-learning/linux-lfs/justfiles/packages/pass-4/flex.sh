#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
flex_file="$(find . -name "flex-*.tar.gz" | head -n1)"
flex_folder="$(echo "$flex_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")-pass-4"
flex_version="$(echo "$flex_folder" | cut -d'-' -f2)"

if [ ! -d "$flex_folder" ]; then
    mkdir -vp "$flex_folder"
    tar -xvf "$flex_file" -C "$flex_folder" --strip-component 1
fi
pushd "$flex_folder"

build_n_install() {
    set -x

    ./configure \
        --prefix=/usr \
        --disable-static \
        --docdir="/usr/share/doc/flex-$flex_version"

    make
    make check
    make install
}

time build_n_install

ln -sv flex   /usr/bin/lex
ln -sv flex.1 /usr/share/man/man1/lex.1
