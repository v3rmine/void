#!/bin/sh
args() {
	if echo "$@" | grep -q '--status'; then
		echo "STATUS_CHECK"
	else 
		echo "MIGRATION"
	fi
}