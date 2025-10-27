#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
perl_file="$(find . -name "perl-*.tar.xz" | head -n1)"
perl_folder="$(echo "$perl_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-3"

if [ ! -d "$perl_folder" ]; then
    mkdir -vp "$perl_folder"
    tar -xvf "$perl_file" -C "$perl_folder" --strip-component 1
fi
pushd "$perl_folder"

build_n_install() {
    set -x
    sh Configure \
        -des \
        -D prefix=/usr \
        -D vendorprefix=/usr \
        -D useshrplib \
        -D privlib=/usr/lib/perl5/5.42/core_perl \
        -D archlib=/usr/lib/perl5/5.42/core_perl \
        -D sitelib=/usr/lib/perl5/5.42/site_perl \
        -D sitearch=/usr/lib/perl5/5.42/site_perl \
        -D vendorlib=/usr/lib/perl5/5.42/vendor_perl \
        -D vendorarch=/usr/lib/perl5/5.42/vendor_perl
    make
    make install
}

time build_n_install
