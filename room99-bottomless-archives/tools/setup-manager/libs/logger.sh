#!/bin/bash
source "${BASH_SOURCE%/*}/colors.sh"

function logger() {
	printf "%s: [$(date +%Y-%m-%dT%H:%M:%S)] %s\n" "$1" "$2"
}

function error() {
	logger "$(red "$(bold ERROR)")" "$(red "$1")"
}; export error

function warn() {
	logger "$(yellow "$(bold ERROR)")" "$(yellow "$1")"
}; export warn

function info() {
	logger "$(green "$(bold INFO)")" "$(green "$1")"
}; export info

function mute_stdout() {
	"$@" >> /dev/null
}; export mute_stdout

function mute() {
	"$@" >> /dev/null 2>&1
}; export mute
