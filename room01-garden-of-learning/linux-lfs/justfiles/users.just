set working-directory := ".."

LFS := "/mnt/lfs"

# Display this help screen
help:
    @just --list users --list-submodules

# Create group lfs on host
create-lfs-group:
    if grep -q -E "^lfs:" /etc/group; then exit 1 fi
    groupadd lfs

# Create user lfs on host
create-lfs-user:
    if id lfs; then exit 1; fi
    useradd -s /bin/bash -g lfs -m -k /dev/null lfs

# Create home files for the user lfs
setup-lfs-user:
    #!/usr/bin/env -S sudo -u lfs -- /bin/bash
    set -euxo pipefail

    cat > ~/.bash_profile << "EOF"
    exec env -i HOME="$HOME" TERM="$TERM" PS1='\u:\w\$ ' /bin/bash
    EOF

    cat > ~/.bashrc << "EOF"
    set +h
    umask 022
    LFS="{{ LFS }}"
    LC_ALL=POSIX
    LFS_TGT="$(uname -m)-lfs-linux-gnu"
    PATH=/usr/bin
    if [ ! -L /bin ]; then PATH="/bin:$PATH"; fi
    PATH="$LFS/tools/bin:$PATH"
    CONFIG_SITE="$LFS/usr/share/config.site"
    export LFS LC_ALL LFS_TGT PATH CONFIG_SITE
    EOF

    cat >> ~/.bashrc << "EOF"
    export MAKEFLAGS="-j$(nproc)"
    EOF
