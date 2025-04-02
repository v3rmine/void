#!/bin/bash
is_manjaro=$(bash -c "source ${BASH_SOURCE%/*}/../os/manjaro.sh; \$is_manjaro")

function status() {
	if $is_manjaro; then
		pacman -Qi | grep -q "$@"
	fi
}; export install