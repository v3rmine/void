#!/bin/bash
is_unix=$(uname -s | grep -q '^Linux' && echo true || echo false)
has_bc=$(command -v bc >> /dev/null)

function get_time() {
	if $has_bc && $is_unix; then
		date -u +%s.%N
	else
		date -u +%s
	fi
}; export get_time
function elapsed_time() {
	start_time="$1"
	end_time="$2"

	if $has_bc; then
		echo "$end_time-$start_time" | bc
	else
		echo "$((end_time - start_time))"
	fi
}