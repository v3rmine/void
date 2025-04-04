#!/bin/bash
if [[ -f "simple-server.log" ]]; then
  echo "Cleaning up old log"
  mv simple-server.log simple-server.log.old
fi
if [[ ! -f "./target/release/simple-server" ]]; then
  echo "Building"
  cargo build --release
fi
./target/release/simple-server >> simple-server.log 2>&1 &
