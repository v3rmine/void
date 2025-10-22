set working-directory := "../.."

build-pass-2:
    #!/usr/bin/env -S sudo -u lfs -- bash
    source ~/.bashrc

    pushd $LFS/sources
    diffutils_file="$(find -name "diffutils-*.tar.xz" | head -n1)"
    diffutils_folder="$(echo "$diffutils_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")"

    if [ ! -d "$diffutils_folder" ]; then
        tar -xvf "$diffutils_file"
    fi
    pushd "$diffutils_folder"

    build_n_install() {
        set -x
        ./configure --prefix=/usr   \
                    --host=$LFS_TGT \
                    gl_cv_func_strcasecmp_works=y \
                    --build=$(./build-aux/config.guess)
        make
        make DESTDIR=$LFS install
    }

    time build_n_install
