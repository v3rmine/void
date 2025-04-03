# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

check-all:
    cargo clippy --locked -- -D warnings
check-lib:
    cargo clippy --locked --package wasm-plugins -- -D warnings
build-lib *PARAMS:
    cargo build --locked --package wasm-plugins {{PARAMS}}

build-example EXAMPLE:
    cargo build --locked --package {{EXAMPLE}}-runner
    cargo build --locked --package {{EXAMPLE}}-plugin --target wasm32-unknown-unknown
_build-example EXAMPLE:
    if [ ! -f ./target/wasm32-unknown-unknown/debug/{{replace(EXAMPLE, "-", "_")}}_plugin.wasm ]; then just build-example {{EXAMPLE}}; fi
run-example EXAMPLE: (_build-example EXAMPLE)
    cargo run --locked --package {{EXAMPLE}}-runner -- ./target/wasm32-unknown-unknown/debug/{{replace(EXAMPLE, "-", "_")}}_plugin.wasm

docs:
    cargo doc --locked --package wasm-plugins --all-features

test-all:
    cargo test --locked
    cargo test --locked --package wasm-plugins --features wasmer-rt-sys
    cargo test --locked --package wasm-plugins --features wasmer

nextest-all:
    cargo nextest run
    cargo nextest run --package wasm-plugins --features wasmer-rt-sys
    cargo nextest run --package wasm-plugins --features wasmer

clean:
    cargo clean --locked
