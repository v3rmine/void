# arsonist
**Please create the DB before running migrations**  
And fill the `.env` with your values from the template `.env.example`.

## If ever need to write tests
- for algorithms and a lot of edges cases => https://model-checking.github.io/kani/rust-feature-support.html
- helper everyday => https://docs.rs/assay/latest/assay

## CICD
- Benchmark report after update => https://crates.io/crates/cargo-benchcmp
- Bug and various mistakes checker (mid level interpreter) => https://github.com/rust-lang/miri
- List outdated dependencies => https://crates.io/crates/cargo-outdated
- List vulnerable dependencies => https://crates.io/crates/cargo-audit
- Code coverage => https://crates.io/crates/cargo-tarpaulin (cargo tarpaulin --ignore-tests)
- Fast test runner => https://nexte.st/
- Unsafe code finder => https://github.com/rust-secure-code/cargo-geiger
- Check if dependencies have been audited by third parties => https://github.com/mozilla/cargo-vet
- Dependencies linter => https://github.com/EmbarkStudios/cargo-deny
- Dependencies size checker => https://github.com/RazrFalcon/cargo-bloat
- Another code fuzzer => https://github.com/rust-fuzz/cargo-fuzz

## Deps
```toml
[dependencies]
# Generate and verify passwords
argon2 = "0.4"
rand_core = { version = "0.6", features = ["std"] }
# Http server
axum = { version = "0.5", features = ["http2"] }
# Http / Https / Unix client
hyper = { version = "0.14", features = ["http1", "http2", "client"] }
hyper-tls = "0.5.0"
futures = { version = "0.3", default-features = false }
urlencoding = "2.1"
# Handle dates management
chrono = "0.4"
# SQL ORM
entities = { path = "./services/models" }
sea-orm = { version = "0.8", features = ["sqlx-sqlite", "runtime-tokio-rustls", "debug-print"] }
# Load .env file in the environment
dotenvy = "0.15"
# Handle errors
eyre = "0.6"
color-eyre = { version = "0.6", default-features = false }
# Serialize and Deserialize everything
serde = { version = "1", features = ["derive"] }
# Serialize and Deserialize json
serde_json = "1"
# Async runtime
tokio = { version = "1", features = ["full"] }
# Logging
tower-http = { version = "0.3", features = ["trace"] }
tracing = "0.1"
tracing-appender = "0.2"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tracing-futures = "0.2" # needed so intrument works with async functions.
# Library helper (builder pattern)
typed-builder = "0.10"
```