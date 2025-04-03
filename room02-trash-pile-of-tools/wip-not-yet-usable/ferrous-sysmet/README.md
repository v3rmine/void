# ferrous-sysmet
Daemonless server metrics collector and frontend.
Full rewrite of https://github.com/diamondburned/sysmet in Rust (store data in a MessagePack file, sysmet-update and sysmet-http) 

**Warning** it is not compatible with the original databases

## CRON
For example you can run `sysmet-update` with cron every 5min and purge data older than 2 days
```
*/5 * * * * /<path to>/sysmet-update -db /<path to>/database -gc 2
```

<!--
# Need reporting panel
https://lib.rs/crates/tracing-honeycomb

# Need cache?
https://lib.rs/crates/moka

## If ever need to write tests
- for algorithms and a lot of edges cases => https://model-checking.github.io/kani/rust-feature-support.html
- helper everyday => https://docs.rs/assay/latest/assay

## CICD
- Benchmark report after update => https://lib.rs/crates/cargo-benchcmp
- Bug and various mistakes checker (mid level interpreter) => https://github.com/rust-lang/miri
- List outdated dependencies => https://lib.rs/crates/cargo-outdated
- List vulnerable dependencies => https://lib.rs/crates/cargo-audit
- Code coverage => https://lib.rs/crates/cargo-tarpaulin (cargo tarpaulin --ignore-tests)
- Fast test runner => https://nexte.st/
- Unsafe code finder => https://github.com/rust-secure-code/cargo-geiger
- Check if dependencies have been audited by third parties => https://github.com/mozilla/cargo-vet
- Dependencies linter => https://github.com/EmbarkStudios/cargo-deny
- Dependencies size checker => https://github.com/RazrFalcon/cargo-bloat
- Another code fuzzer => https://github.com/rust-fuzz/cargo-fuzz
-->
