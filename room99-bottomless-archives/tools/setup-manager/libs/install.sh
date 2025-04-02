#!/bin/bash
is_manjaro=$(bash -c "source ${BASH_SOURCE%/*}/../os/manjaro.sh; is_manjaro")

function install() {
	if $is_manjaro; then
		if echo "$@" | grep -q "--aur"; then
			yay -S $(echo "$@" | sed 's/\--aur//g')
		else
			sudo pacman -S "$@"
		fi
	fi
}; export install