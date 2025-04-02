#!/bin/bash
c_red=""
c_yellow=""
c_green=""
c_gray=""
c_reset=""
c_strike=""
c_reset_strike=""
c_bold=""
c_reset_bold=""
# If the shell has an output, then we can use colors
if [ -t 0 ]; then
	c_red="\033[31m"
	c_yellow="\033[33m"
	c_green="\033[32m"
	c_gray="\033[90m"
	c_reset="\033[0m"
	c_strike="\033[9m"
	c_reset_strike="\033[29m"
	c_bold="\033[1m"
	c_reset_bold="\033[22m"
fi
function red() { printf "%b%s%b\n" "$c_red" "$@" "$c_reset"; }; export red
function yellow() { printf "%b%s%b\n" "$c_yellow" "$@" "$c_reset"; }; export yellow
function green() { printf "%b%s%b\n" "$c_green" "$@" "$c_reset"; }; export green
function gray() { printf "%b%s%b\n" "$c_gray" "$@" "$c_reset"; }; export gray
function bold() { printf "%b%s%b\n" "$c_bold" "$@" "$c_reset_bold"; }; export bold
function deprecated() { printf "%b%s%b\n" "$c_strike" "$@" "$c_reset_strike"; }; export deprecated

gray_wrap() {
	begin="$1"
	content="$2"
	end="$3"

	echo "$(gray "$begin")$content$(gray "$end")"
}