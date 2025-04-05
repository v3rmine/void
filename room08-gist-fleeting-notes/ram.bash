#!/bin/bash

# Bash script to write / store in RAM 

OS=$(uname -s)

# NOTE: Define Temp directory
if test "$OS" == 'Darwin'; then TMPDIR="/tmp"; else TMPDIR="/dev/shm"; fi
