#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
readline_file="$(find . -name "readline-*.tar.gz" | head -n1)"
readline_folder="$(echo "$readline_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")-pass-4"
readline_version="$(echo "$readline_folder" | cut -d'-' -f2)"

if [ ! -d "$readline_folder" ]; then
    mkdir -vp "$readline_folder"
    tar -xvf "$readline_file" -C "$readline_folder" --strip-component 1
fi
pushd "$readline_folder"

sed -i '/MV.*old/d' Makefile.in
sed -i '/{OLDSUFF}/c:' support/shlib-install

sed -i 's/-Wl,-rpath,[^ ]*//' support/shobj-conf

build_n_install() {
    set -x

    ./configure \
        --prefix=/usr \
        --disable-static \
        --with-curses \
        --docdir="/usr/share/doc/readline-$readline_version"
    make SHLIB_LIBS="-lncursesw"
    make install
}

time build_n_install

install -v -m644 doc/*.{ps,pdf,html,dvi} "/usr/share/doc/readline-$readline_version"
