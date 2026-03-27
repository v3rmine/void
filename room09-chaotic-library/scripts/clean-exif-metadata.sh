#!/usr/bin/env bash
set -eu
exiftool -all:all= -r static/media
rm -f static/media/**/*.*_original
