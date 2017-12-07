# allan - variance and deviation tools for stability analysis

allan is a library implementing Allan Variance and Deviation
for stability analysis of oscillators, gyroscopes, etc

[![conduct-badge][]][conduct] [![travis-badge][]][travis] [![downloads-badge][] ![release-badge][]][crate] [![license-badge][]](#license)

[conduct-badge]: https://img.shields.io/badge/%E2%9D%A4-code%20of%20conduct-blue.svg
[travis-badge]: https://img.shields.io/travis/brayniac/allan/master.svg
[downloads-badge]: https://img.shields.io/crates/d/allan.svg
[release-badge]: https://img.shields.io/crates/v/allan.svg
[license-badge]: https://img.shields.io/crates/l/allan.svg
[conduct]: https://brayniac.github.io/conduct
[travis]: https://travis-ci.org/brayniac/allan
[crate]: https://crates.io/crates/allan
[Cargo]: https://github.com/rust-lang/cargo

## Code of Conduct

**NOTE**: All conversations and contributions to this project shall adhere to the [Code of Conduct][conduct]

## Usage

To use `allan`, first add this to your `Cargo.toml`:

```toml
[dependencies]
allan = "*"
```

Then, add this to your crate root:

```rust
extern crate allan;
```

The API documentation of this library can be found at
[docs.rs/allan](https://docs.rs/allan/).

## Features

* Calculate overlapping Allan Deviation and Variance

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
