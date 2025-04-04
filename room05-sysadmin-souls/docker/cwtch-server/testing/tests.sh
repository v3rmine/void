#!/bin/bash

set -e
pwd
GORACE="haltonerror=1"
go test -race ${1} -coverprofile=server.metrics.cover.out -v ./metrics
go test -race ${1} -coverprofile=server.storage.cover.out -v ./storage
go test -race ${1} -coverprofile=server.cover.out -v ./
echo "mode: set" > coverage.out && cat *.cover.out | grep -v mode: | sort -r | \
awk '{if($1 != last) {print $0;last=$1}}' >> coverage.out
rm -rf *.cover.out
