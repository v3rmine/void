set working-directory := ".."

LFS := "$PWD/lfs"

# Display this help screen
help:
    @just --list filesystem --list-submodules

create-stage-1-filesystem:
    #!/usr/bin/env bash
    umask 022

    mkdir -pv {{ LFS }}/{etc,var} {{ LFS }}/usr/{bin,lib,sbin}

    for i in bin lib sbin; do
    ln -sv usr/$i {{ LFS }}/$i
    done

    case $(uname -m) in
        x86_64) mkdir -pv {{ LFS }}/lib64 ;;
    esac

    mkdir -pv {{ LFS }}/tools

    chown -v lfs {{ LFS }}/{usr{,/*},var,etc,tools}
    case $(uname -m) in
      x86_64) chown -v lfs {{ LFS }}/lib64 ;;
    esac
