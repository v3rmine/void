#!/bin/bash

# --- Begin Utils ---
c_green=""
c_gray=""
c_reset=""
c_bold=""
c_reset_bold=""
# If the shell has an output, then we can use colors
if [ -t 0 ]; then
	c_green="\033[32m"
	c_gray="\033[90m"
	c_reset="\033[0m"
	c_bold="\033[1m"
	c_reset_bold="\033[22m"
fi
function green() { printf "%b%s%b\n" "$c_green" "$@" "$c_reset"; }
function gray() { printf "%b%s%b\n" "$c_gray" "$@" "$c_reset"; }
function bold() { printf "%b%s%b\n" "$c_bold" "$@" "$c_reset_bold"; }
function logger() { printf "%s: [$(date +%Y-%m-%dT%H:%M:%S)] %s\n" "$1" "$2"; }
function info() { logger "$(green "$(bold INFO)")" "$(green "$1")"; }
function trace() { logger "$(green "$(bold TRACE)")" "$(gray "$1")"; }

# shellcheck disable=2001
function sedize() { echo "$1" | sed 's/^/\\n/g' | tr -d '\n'; }
# --- End Utils ---

source=""
if [ ! -d "$1" ]; then
  # TODO: Download source
  exit 1
else
  source="$1"
fi

outdir=${OUTDIR:="./patched_fuse"}
if [ -d "$outdir" ]; then rm -r "$outdir"; fi
mkdir "$outdir"

info "Patching lib/fuse.c"

trace "Adding define FUSE_SYMVER patch"
patch='#ifdef C2NIM
#  def FUSE_SYMVER(sym1, sym2)
#endif'
sed -i "1s|^|$(sedize "$patch")\n|" "$source/lib/fuse.c"

trace "Adding define container_of patch"
match='^#(define container_of\(ptr, type, member\) \(\{\s*\\\s*.*?\\\s*.*?\}\))'
patch='#ifdef C2NIM
#  def container_of(ptr, type, member) { const typeof( ((type *)0)->member ) *__mptr = (ptr); (type *)( (char *)__mptr - offsetof(type,member) ); }
#endif

#ifndef C2NIM
#  \1
#endif'
perl -i -0pe "s|$match|$(sedize "$patch")|gm" "$source/lib/fuse.c"

trace "Adding define FUSE_LIB_OPT patch"
match='^#define FUSE_LIB_OPT\(t, p, v\) \{ t, offsetof\(struct fuse_config, p\), v \}'
patch='#ifdef C2NIM
#  def FUSE_LIB_OPT(t, p, v) { t, offsetof(struct fuse_config, p), v }
#endif

#ifndef C2NIM
#  define FUSE_LIB_OPT(t, p, v) { t, offsetof(struct fuse_config, p), v }
#endif'
perl -i -pe "s|$match|$(sedize "$patch")|" "$source/lib/fuse.c"

trace "Generating fuse.nim"
c2nim --debug -o:"$outdir/fuse.nim" "$source/lib/fuse.c"
