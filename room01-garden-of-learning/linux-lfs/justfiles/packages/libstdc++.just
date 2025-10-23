set working-directory := "../.."

build-pass-1:
    #!/usr/bin/env -S sudo -u lfs -- bash
    source ~/.bashrc
    set -euo pipefail

    pushd $LFS/sources
    gcc_file="$(find -name "gcc-*.tar.xz" | head -n1)"
    gcc_folder="$(echo "$gcc_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")"
    gcc_version="$(echo "$gcc_folder" | cut -d'-' -f2)"

    if [ ! -d "$gcc_folder" ]; then
        tar -xvf "$gcc_file"
    fi
    pushd "$gcc_folder"

    mkdir -vp libstdcpp_build
    pushd libstdcpp_build

    build_n_install() {
        set -x
        ../libstdc++-v3/configure      \
            --host=$LFS_TGT            \
            --build=$(../config.guess) \
            --prefix=/usr              \
            --disable-multilib         \
            --disable-nls              \
            --disable-libstdcxx-pch    \
            --with-gxx-include-dir=/tools/$LFS_TGT/include/c++/15.2.0
        make
        make DESTDIR=$LFS install
    }

    time build_n_install
    rm -v $LFS/usr/lib/lib{stdc++{,exp,fs},supc++}.la
