#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
gmp_file="$(find . -name "gmp-*.tar.xz" | head -n1)"
gmp_folder="$(echo "$gmp_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-4"
gmp_version="$(echo "$gmp_folder" | cut -d'-' -f2)"

if [ ! -d "$gmp_folder" ]; then
    mkdir -vp "$gmp_folder"
    tar -xvf "$gmp_file" -C "$gmp_folder" --strip-component 1
fi
pushd "$gmp_folder"

# First, make an adjustment for compatibility with gcc-15 and later
# https://www.linuxfromscratch.org/lfs/view/r12.4-37/chapter08/gmp.html
sed -i '/long long t1;/,+1s/()/(...)/' configure

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --enable-cxx \
        --disable-static \
        --docdir="/usr/share/doc/gmp-$gmp_version"
    make
    make html

    make check 2>&1 | tee gmp-check-log
    passed_tests="$(awk '/# PASS:/{total+=$3} ; END{print total}' gmp-check-log)"
    if [ "$passed_tests" -lt 199 ]; then
        echo "[ERROR]: At least 199 tests must pass"
        exit 1
    fi

    make install
    make install-html
}

time build_n_install
