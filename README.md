# axos

[![CI Build Status]][actions]
[![Release]][actions]
[![Tag Build Status]][actions]
[![License]][mit-license]
[![Docs]][Docs-rs]
[![Latest Version]][crates.io]
[![rustc 1.31+]][Rust 1.31]

[CI Build Status]: https://img.shields.io/github/actions/workflow/status/refcell/axos/ci.yml?branch=main&label=build
[Tag Build Status]: https://img.shields.io/github/actions/workflow/status/refcell/axos/tag.yml?branch=main&label=tag
[Release]: https://img.shields.io/github/actions/workflow/status/refcell/axos/release.yml?branch=main&label=release
[actions]: https://github.com/refcell/axos/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/axos.svg
[crates.io]: https://crates.io/crates/axos
[rustc 1.31+]: https://img.shields.io/badge/rustc_1.31+-lightgray.svg
[Rust 1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[License]: https://img.shields.io/badge/license-MIT-7795AF.svg
[mit-license]: https://github.com/refcell/axos/blob/main/LICENSE.md
[Docs-rs]: https://docs.rs/axos/
[Docs]: https://img.shields.io/docsrs/axos.svg?color=319e8c&label=docs.rs

**Portable no-std Derivation Pipeline engineered in pure Rust** https://github.com/refcell/axos/labels/alpha

![](./etc/banner.png)

**[Install](#usage)**
| [Usage](#usage)
| [Development](#developing)
| [Crate Docs][crates.io]
| [Reference][Docs-rs]
| [Contributing](#contributing)
| [License](#license)

## What is axos?

Portable no-std Derivation Pipeline engineered in pure Rust.

## Usage

`axos` is meant to be used as a library with [cargo][crates.io].
Add it to your project with [`cargo add`](https://github.com/rust-lang/cargo/tree/master/src/cargo/ops/cargo_add).

```text
cargo add axos
```

## Developing

To get started locally, make sure to have [`just`][just] installed.
This will allow you to get up and running by executing the `just`
command in the root of the `axos` repository. This will run the
following `Justfile` targets:

- *fmt*: Formats all rust files with [rustfmt][fmt].
- *clippy*: Lints all targets with [clippy][clippy]. (all features)
- *tests*: Runs all workspace tests with [nextest][nextest]. (all features)

To run the `axos` derivation pipeline as a binary, there is a convenience
binary crate `axt`, inside [`./bins/`][bins], that can run `axos` using
mock data, an offline provider, or with live chain data. _Just_ run `axt`
in mock provider mode using the `axt` target: `just axt`.

[bins]: ./bins/
[nextest]: https://github.com/nextest-rs/nextest
[clippy]: https://github.com/rust-lang/rust-clippy
[fmt]: https://github.com/rust-lang/rustfmt
[just]: https://github.com/casey/just

## Contributing

Contributions to `axos` are very welcome and highly appreciated. When
opening a pull request or issue, please provide concise and descriptive
detail such that anyone with little context reading your issue or
reviewing your pull request can easily understand and provide meaningful
feedback.

Before opening a pull request, make sure all tests pass. Execute `just`
to run the default Justfile target, formating, linting, and testing
all rust files in the cargo workspace. If the `just` command succeeds
locally, ci should pass in your pull request.

## Troubleshooting

Please check existing issues for similar bugs or
[open an issue](https://github.com/refcell/axos/issues/new)
if no relevant issue already exists.

## License

Fully licensed under the [MIT License](LICENSE.md). Open source forever.
