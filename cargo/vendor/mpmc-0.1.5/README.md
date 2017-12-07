# mpmc - multi-producer multi-consumer queue

mpmc is a multi-produce multi-consumer queue which has been copied from the old Rust stdlib

[![conduct-badge][]][conduct] [![travis-badge][]][travis] [![downloads-badge][] ![release-badge][]][crate] [![license-badge][]](#license)

[conduct-badge]: https://img.shields.io/badge/%E2%9D%A4-code%20of%20conduct-blue.svg
[travis-badge]: https://img.shields.io/travis/brayniac/mpmc/master.svg
[downloads-badge]: https://img.shields.io/crates/d/mpmc.svg
[release-badge]: https://img.shields.io/crates/v/mpmc.svg
[license-badge]: https://img.shields.io/crates/l/mpmc.svg
[conduct]: https://brayniac.github.io/conduct
[travis]: https://travis-ci.org/brayniac/mpmc
[crate]: https://crates.io/crates/mpmc
[Cargo]: https://github.com/rust-lang/cargo

## Code of Conduct

**NOTE**: All conversations and contributions to this project shall adhere to the [Code of Conduct][conduct]

## Usage

To use `mpmc`, first add this to your `Cargo.toml`:

```toml
[dependencies]
mpmc = "*"
```

Then, add this to your crate root:

```rust
extern crate mpmc;
```

The API documentation of this library can be found at
[docs.rs/mpmc](https://docs.rs/mpmc/).
