# Take - A simple container utility for Rust

`Take` is a Cell allowing the inner value to be consumed without a
mutable reference.

In order to maintain safety, it is not possible to get access to the
inner value without consuming it.

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
take = "0.1.0"
```

Next, add this to your crate:

```rust
extern crate take;
```

And then, use Take!

## License

Take is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.
