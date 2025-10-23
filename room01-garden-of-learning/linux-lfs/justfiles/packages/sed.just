set working-directory := "../.."

build-pass-2:
    #!/usr/bin/env -S sudo -u lfs -- bash
    source ~/.bashrc
    set -euo pipefail

    pushd $LFS/sources
    sed_file="$(find -name "sed-*.tar.xz" | head -n1)"
    sed_folder="$(echo "$sed_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")"

    if [ ! -d "$sed_folder" ]; then
        tar -xvf "$sed_file"
    fi
    pushd "$sed_folder"

    build_n_install() {
        set -x
        ./configure --prefix=/usr   \
                    --host=$LFS_TGT \
                    --build=$(./build-aux/config.guess)
        make
        make DESTDIR=$LFS install
    }

    time build_n_install
