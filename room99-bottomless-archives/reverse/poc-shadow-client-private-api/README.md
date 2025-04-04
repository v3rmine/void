# shadowtech-api
![GitHub workflow Status](https://img.shields.io/github/workflow/status/joxcat/shadow-private-api/master?style=flat-square)
![License GitHub](https://img.shields.io/github/license/joxcat/shadow-private-api?style=flat-square)
![GitHub release (lasted SemVer)](https://img.shields.io/github/release/joxcat/shadow-private-api?sort=semver&style=flat-square)
[![Doc](https://img.shields.io/badge/documentation-rustdoc-purple?style=flat-square)](https://joxcat.github.io/shadow-private-api/shadowtech_api)

Library to use to Shadow.tech private API (Fairly small API for the moment). Endpoints where found using Web Debugging Proxy and test where made using [Insomnia](https://github.com/Kong/insomnia).

<!--
| OS      | Build Status |
| ------- | ------------ |
| Linux   |              |
| Windows |              |
| OSX     |              |
-->

## Exemple
```rust
// {Insert small example here}
```
<!--
For fuller examples, take a look at [`examples/some_example.rs`](examples/some_example.rs).
-->
## Installation
Add the following to your Cargo.toml file:

```toml
[dependencies]
shadowtech-api = { git = "https://github.com/joxcat/shadow-private-api" }
```

### Features
Features can be enabled or disabled by configuring the library through Cargo.toml:

```toml
[dependencies.shadowtech-api]
default-features = false
features = ["pick", "your", "feature", "names", "here"]
```
<!--
The default features are: {Features list}.

The following is a full list of features:
- **feature:** description

### Dependencies
{External dependencies if needed}
-->
## Roadmap
- [ ] Lib for Shadow API
- [ ] Somes examples
- [ ] Documentation for Shadow private API
- [ ] Scanning the mobile app for more endpoints

<!--
## FAQ
{Insert common errors and response to questions}
-->
## License
Licensed under [Unlicense](LICENSE)

## Contributing
Please read the [Contributing Guide](.github/CONTRIBUTING.md).

### Contributors
- [joxcat](https://github.com/joxcat)
<!--
## Related Projects
- {Some related project}

## Alternatives
{Insert alternatives if it exist somes}
-->
