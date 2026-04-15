# Gitlab-FS
*Internal tool to manage and compare Gitlab environnements.*

A fuse filesystem to mount Gitlab projects, review and compare environments.

## TODO
- [ ] Allow edition of env variables
- [ ] Allow creation of new environments
- [ ] Build for all platforms
- [ ] Add tests

## Runtime dependencies
- OSX: macfuse (`brew install macfuse`)
- Linux: fuse (debian / ubuntu: `apt-get install fuse`)

## Requirements (build)
- fuse
        - debian / ubuntu: `apt-get install libfuse-dev`
        - OSX: `brew install macfuse`
- rust: https://rustup.rs
- pkg-config
        - debian / ubuntu: `apt-get install pkg-config`
        - OSX: already bundled

## Usage
```bash
Usage: gitlab-fs [OPTIONS] --gitlab-access-token <GITLAB_ACCESS_TOKEN> [QUERY]

Arguments:
  [QUERY]  The Gitlab query to filter projects [default: ]

Options:
      --gitlab-host <GITLAB_HOST>
          [env: GITLAB_HOST=] [default: gitlab.com] [aliases: host]
      --gitlab-access-token <GITLAB_ACCESS_TOKEN>
          [env: GITLAB_ACCESS_TOKEN=] [aliases: access_token]
      --mountpoint <MOUNTPOINT>
          [env: MOUNTPOINT=] [default: ./mnt]
  -v, --verbose...
          -v for info ; -vv for debug ; -vvv for trace (shortcut to set LOG_LEVEL)
  -h, --help
          Print help information
  -V, --version
          Print version information
```

## Rust as beginner
- In french: [blog.guillaume-gomez.fr/Rust](https://blog.guillaume-gomez.fr/Rust)
- With small exercises to solve step by step: [Rustlings](https://github.com/rust-lang/rustlings)
- With code examples: [Rust by example](https://doc.rust-lang.org/rust-by-example/)

## Build
```bash
# To build the project
cargo build --release

# To build the documentation
# Using nightly till https://github.com/rust-lang/cargo/issues/8229 
RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo +nightly-2023-05-22 doc --workspace --document-private-items --no-deps

# To release a version
git commit -am "chore: Release version X.Y.Z"
git push
git tag vX.Y.Z
# This will trigger the release
git push --tags
```
