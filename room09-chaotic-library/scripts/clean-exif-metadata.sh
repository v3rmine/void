#!/usr/bin/env bash
set -eu
exiftool -all:all= -r static/media
rm -f static/media/*.jpg_original
