set working-directory := "../.."

build-pass-1:
    #!/usr/bin/env -S sudo -u lfs -- bash
    source ~/.bashrc

    pushd $LFS/sources
    binutils_file="$(find -name "binutils-*.tar.xz" | head -n1)"
    binutils_folder="$(echo "$binutils_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")"

    if [ ! -d "$binutils_folder" ]; then
        tar -xvf "$binutils_file"
    fi
    pushd "$binutils_folder"

    mkdir -vp build
    pushd build

    build_n_install() {
        set -x
        ../configure --prefix=$LFS/tools \
                    --with-sysroot=$LFS \
                    --target=$LFS_TGT \
                    --disable-nls \
                    --enable-gprofng=no \
                    --disable-werror \
                    --enable-new-dtags \
                    --enable-default-hash-style=gnu
        make
        make install
    }

    time build_n_install
