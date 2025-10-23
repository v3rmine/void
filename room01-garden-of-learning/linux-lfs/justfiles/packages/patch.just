set working-directory := "../.."

build-pass-2:
    #!/usr/bin/env -S sudo -u lfs -- bash
    source ~/.bashrc
    set -euo pipefail

    pushd $LFS/sources
    patch_file="$(find -name "patch-*.tar.xz" | head -n1)"
    patch_folder="$(echo "$patch_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")"

    if [ ! -d "$patch_folder" ]; then
        tar -xvf "$patch_file"
    fi
    pushd "$patch_folder"

    build_n_install() {
        set -x
        ./configure --prefix=/usr   \
                    --host=$LFS_TGT \
                    --build=$(build-aux/config.guess)
        make
        make DESTDIR=$LFS install
    }

    time build_n_install
