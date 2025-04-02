#!/bin/bash
function is_os() {
	uname -r | grep -q "MANJARO"
}; export is_os
export is_manjaro=$(is_os)