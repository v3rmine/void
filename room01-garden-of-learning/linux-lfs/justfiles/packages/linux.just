set working-directory := "../.."

build-pass-1:
    #!/usr/bin/env -S sudo -u lfs -- bash
    source ~/.bashrc
    set -euo pipefail

    pushd $LFS/sources
    linux_file="$(find -name "linux-*.tar.xz" | head -n1)"
    linux_folder="$(echo "$linux_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")"

    if [ ! -d "$linux_folder" ]; then
        tar -xvf "$linux_file"
    fi
    pushd "$linux_folder"

    build_n_install() {
        set -x
        make mrproper
        make headers
        find usr/include -type f ! -name '*.h' -delete
        cp -rv usr/include $LFS/usr
    }

    time build_n_install
