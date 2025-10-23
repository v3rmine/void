set working-directory := "../.."

build-pass-2:
    #!/usr/bin/env -S sudo -u lfs -- bash
    source ~/.bashrc
    set -euo pipefail

    pushd $LFS/sources
    grep_file="$(find -name "grep-*.tar.xz" | head -n1)"
    grep_folder="$(echo "$grep_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")"

    if [ ! -d "$grep_folder" ]; then
        tar -xvf "$grep_file"
    fi
    pushd "$grep_folder"

    build_n_install() {
        set -x
        ./configure --prefix=/usr   \
                    --host=$LFS_TGT \
                    --build=$(./build-aux/config.guess)
        make
        make DESTDIR=$LFS install
    }

    time build_n_install
