[![Build and Test](https://github.com/ImperialBower/pkstate/actions/workflows/CI.yaml/badge.svg)](https://github.com/ImperialBower/pkstate/actions/workflows/CI.yaml)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)
[![Crates.io Version](https://img.shields.io/crates/v/rs_blank_example.svg)](https://crates.io/crates/rs_blank_example)
[![Rustdocs](https://docs.rs/rs_blank_example/badge.svg)](https://docs.rs/rs_blank_example/)

---

# PKState

## Resources

- [Poker Hand History File Format Specification](https://github.com/uoftcprg/phh-std)

### Cargo Make

This program uses [cargo make](https://github.com/sagiegurari/cargo-make) to manage tasks. Install it with:

```shell
cargo install cargo-make
```

The default `cargo make` runs the following tasks:

* `cargo fmt`
* `cargo clean`
* `cargo build`
* `carg test`
* `cargo clippy` with `clippy::pedantic` lint settings
* `cargo doc --no-deps`

```shell
❯ cargo make
````

To open the generated docs in your browser:

```shell
❯ cargo make docs
```

