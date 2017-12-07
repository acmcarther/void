# clocksource - high performance clocks for Rust

Clocksource allows access to alternate clocksources like the TSC on your Intel x86 CPU. Most modern processors support 'constant_tsc' allowing us to use this counter as a high resolution clock. The cost of reading this counter can be much lower than calls to 'clock_gettime()' - especially on virtualized environments.

Unfortunately, this requires we use nightly rust until the asm!() macro is stabilized. We provide fallback for users on stable rust, this should allow zero-cost abstraction of clock_gettime() for stable builds without benefit of the high-performance timing.

[![conduct-badge][]][conduct] [![travis-badge][]][travis] [![downloads-badge][] ![release-badge][]][crate] [![license-badge][]](#license)

[conduct-badge]: https://img.shields.io/badge/%E2%9D%A4-code%20of%20conduct-blue.svg
[travis-badge]: https://img.shields.io/travis/brayniac/clocksource/master.svg
[downloads-badge]: https://img.shields.io/crates/d/clocksource.svg
[release-badge]: https://img.shields.io/crates/v/clocksource.svg
[license-badge]: https://img.shields.io/crates/l/clocksource.svg
[conduct]: https://brayniac.github.io/conduct
[travis]: https://travis-ci.org/brayniac/clocksource
[crate]: https://crates.io/crates/clocksource
[Cargo]: https://github.com/rust-lang/cargo

## Code of Conduct

**NOTE**: All conversations and contributions to this project shall adhere to the [Code of Conduct][conduct]

## Getting clocksource

add `clocksource` to your dependencies and start using it

The API documentation of this library can be found at
[docs.rs/clocksource](https://docs.rs/clocksource/).

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
