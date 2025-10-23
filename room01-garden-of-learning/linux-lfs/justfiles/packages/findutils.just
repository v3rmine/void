set working-directory := "../.."

build-pass-2:
    #!/usr/bin/env -S sudo -u lfs -- bash
    source ~/.bashrc
    set -euo pipefail

    pushd $LFS/sources
    findutils_file="$(find -name "findutils-*.tar.xz" | head -n1)"
    findutils_folder="$(echo "$findutils_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")"

    if [ ! -d "$findutils_folder" ]; then
        tar -xvf "$findutils_file"
    fi
    pushd "$findutils_folder"

    build_n_install() {
        set -x
        ./configure --prefix=/usr                   \
                    --localstatedir=/var/lib/locate \
                    --host=$LFS_TGT                 \
                    --build=$(build-aux/config.guess)
        make
        make DESTDIR=$LFS install
    }

    time build_n_install
