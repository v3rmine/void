#!/bin/bash
# Sauce: https://endler.dev/2020/rust-compile-times/#use-a-ramdisk-for-compilation
mkdir -p "/tmp/$(basename $(pwd))/target"
ln -s "/tmp/$(basename $(pwd))/target" "$(pwd)/"