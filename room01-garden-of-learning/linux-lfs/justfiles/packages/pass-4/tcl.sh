#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
tcl_file="$(find . -name "tcl*-src.tar.gz" | head -n1)"
tcl_folder="$(echo "$tcl_file" | sed -E "s/(^\.\/|-src\.tar\.gz)//g")-pass-4"

if [ ! -d "$tcl_folder" ]; then
    mkdir -vp "$tcl_folder"
    tar -xvf "$tcl_file" -C "$tcl_folder" --strip-component 1
fi
pushd "$tcl_folder"

build_n_install() {
    set -x

    SRCDIR=$(pwd)
    pushd unix
    ./configure \
        --prefix=/usr \
        --mandir=/usr/share/man \
        --disable-rpath

    make

    sed -e "s|$SRCDIR/unix|/usr/lib|" \
        -e "s|$SRCDIR|/usr/include|" \
        -i tclConfig.sh
    sed -e "s|$SRCDIR/unix/pkgs/tdbc1.1.10|/usr/lib/tdbc1.1.10|" \
        -e "s|$SRCDIR/pkgs/tdbc1.1.10/generic|/usr/include|" \
        -e "s|$SRCDIR/pkgs/tdbc1.1.10/library|/usr/lib/tcl8.6|" \
        -e "s|$SRCDIR/pkgs/tdbc1.1.10|/usr/include|" \
        -i pkgs/tdbc1.1.10/tdbcConfig.sh
    sed -e "s|$SRCDIR/unix/pkgs/itcl4.3.2|/usr/lib/itcl4.3.2|" \
        -e "s|$SRCDIR/pkgs/itcl4.3.2/generic|/usr/include|" \
        -e "s|$SRCDIR/pkgs/itcl4.3.2|/usr/include|" \
        -i pkgs/itcl4.3.2/itclConfig.sh

    unset SRCDIR

    LC_ALL=C.UTF-8 make test
    make install
    chmod 644 /usr/lib/libtclstub8.6.a
    chmod -v u+w /usr/lib/libtcl8.6.so

    make install-private-headers
}

time build_n_install

ln -sfv tclsh8.6 /usr/bin/tclsh
mv /usr/share/man/man3/{Thread,Tcl_Thread}.3
