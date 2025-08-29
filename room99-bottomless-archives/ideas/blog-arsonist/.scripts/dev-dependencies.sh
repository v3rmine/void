#!/bin/sh
if ! command -v sea-orm-cli > /dev/null; then
	cargo install sea-orm-cli --locked --force
fi

cd apps/admin-front
.scripts/dev-dependencies.sh
cd ../..