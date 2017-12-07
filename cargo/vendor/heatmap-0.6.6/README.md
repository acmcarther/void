# heatmap - a collection of histograms

heatmap is a stats library for rust which provides time-sliced histogram
storage capable of recording the distribution of values over time. It
maintains precision guarentees throughout the range of stored values.

[![conduct-badge][]][conduct] [![travis-badge][]][travis] [![downloads-badge][] ![release-badge][]][crate] [![license-badge][]](#license)

[conduct-badge]: https://img.shields.io/badge/%E2%9D%A4-code%20of%20conduct-blue.svg
[travis-badge]: https://img.shields.io/travis/brayniac/heatmap/master.svg
[downloads-badge]: https://img.shields.io/crates/d/heatmap.svg
[release-badge]: https://img.shields.io/crates/v/heatmap.svg
[license-badge]: https://img.shields.io/crates/l/heatmap.svg
[conduct]: https://brayniac.github.io/conduct
[travis]: https://travis-ci.org/brayniac/heatmap
[crate]: https://crates.io/crates/heatmap
[Cargo]: https://github.com/rust-lang/cargo

## Code of Conduct

**NOTE**: All conversations and contributions to this project shall adhere to the [Code of Conduct][conduct]

## Usage

To use `heatmap`, first add this to your `Cargo.toml`:

```toml
[dependencies]
heatmap = "*"
```

Then, add this to your crate root:

```rust
extern crate heatmap;
```

The API documentation of this library can be found at
[docs.rs/heatmap](https://docs.rs/heatmap/).

## Features

* a basic two-dimensional histogram
* pre-allocated data structures

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
