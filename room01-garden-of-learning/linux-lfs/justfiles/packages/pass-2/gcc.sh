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
gcc_folder="$(echo "$gcc_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-2"

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

sed '/thread_header =/s/@.*@/gthr-posix.h/' \
    -i libgcc/Makefile.in libstdc++-v3/include/Makefile.in

mkdir -vp build
pushd build

build_n_install() {
    set -x
    ../configure \
        --build="$(../config.guess)" \
        --host="$LFS_TGT" \
        --target="$LFS_TGT" \
        --prefix=/usr \
        --with-build-sysroot="$LFS" \
        --enable-default-pie \
        --enable-default-ssp \
        --disable-nls \
        --disable-multilib \
        --disable-libatomic \
        --disable-libgomp \
        --disable-libquadmath \
        --disable-libsanitizer \
        --disable-libssp \
        --disable-libvtv \
        --enable-languages=c,c++ \
        LDFLAGS_FOR_TARGET="-L$PWD/$LFS_TGT/libgcc"
    make
    make DESTDIR="$LFS" install
}

time build_n_install
ln -sv gcc "$LFS/usr/bin/cc"
