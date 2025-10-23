#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
gcc_file="$(find . -name "gcc-*.tar.xz" | head -n1)"
gcc_folder="$(echo "$gcc_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-1"

mpfr_file="$(find . -name "mpfr-*.tar.xz" | head -n1)"
mpfr_folder="$(echo "$mpfr_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")"
gmp_file="$(find . -name "gmp-*.tar.xz" | head -n1)"
gmp_folder="$(echo "$gmp_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")"
mpc_file="$(find . -name "mpc-*.tar.gz" | head -n1)"
mpc_folder="$(echo "$mpc_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")"

if [ ! -d "$gcc_folder" ]; then
    mkdir -vp "$gcc_folder"
    tar -xvf "$gcc_file" -C "$gcc_folder" --strip-component 1
fi
pushd "$gcc_folder"

tar -xf "../$mpfr_file"
mv -v "$mpfr_folder" mpfr
tar -xf "../$gmp_file"
mv -v "$gmp_folder" gmp
tar -xf "../$mpc_file"
mv -v "$mpc_folder" mpc

case $(uname -m) in
    x86_64)
        sed -e '/m64=/s/lib64/lib/' \
            -i.orig gcc/config/i386/t-linux64
    ;;
esac

mkdir -vp build
pushd build

build_n_install() {
    set -x
    ../configure \
        --target="$LFS_TGT" \
        --prefix="$LFS/tools" \
        --with-glibc-version=2.42 \
        --with-sysroot="$LFS" \
        --with-newlib \
        --without-headers \
        --enable-default-pie \
        --enable-default-ssp \
        --disable-nls \
        --disable-shared \
        --disable-multilib \
        --disable-threads \
        --disable-libatomic \
        --disable-libgomp \
        --disable-libquadmath \
        --disable-libssp \
        --disable-libvtv \
        --disable-libstdcxx \
        --enable-languages=c,c++
    make
    make install
}

time build_n_install

popd
cat gcc/limitx.h gcc/glimits.h gcc/limity.h > \
    "$(dirname "$("$LFS_TGT-gcc" -print-libgcc-file-name)")/include/limits.h"
