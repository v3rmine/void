#!/bin/bash
i=$((0))
while test $i -lt $1; do 
	i=$((i+1))
	cargo run --release -p sysmet-update -- --db $2
done