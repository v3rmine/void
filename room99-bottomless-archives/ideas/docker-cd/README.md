# docker-cd
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