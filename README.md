# chain-assertions

[![Workflow Status](https://github.com/mezum/chain-assertions-rs/workflows/ci/badge.svg)](https://github.com/mezum/chain-assertions-rs/actions?query=workflow%3A%22ci%22)
[![crates.io](https://img.shields.io/crates/v/chain-assertions.svg)](https://crates.io/crates/chain-assertions)
[![docs.rs](https://docs.rs/chain-assertions/badge.svg)](https://docs.rs/chain-assertions)

This crate provides the assertion that can be insert between method chains.

## Usage

Install the crate into your `Cargo.toml`:

```toml
[dependencies]
chain-assertions = "0.1"

# Add `passthrough` to disable checking on debug builds.
# chain-assertions = { version = "0.1", features = ["disable"] }

# Set default-features to false in no-std environment:
# chain-assertions = { version = "0.1", default-features = false }
```

and then use it like following:

```rust
use chain_assertions::prelude::*;

let target = i32::from_str_radix("21", 10)
    .debug_assert_ok()
    .map(|v| v * 2);
assert_eq!(target, Ok(42));
```

```rust,ignore
use chain_assertions::prelude::*;

let target = i32::from_str_radix("foobar", 10)
    .debug_assert_ok()
    // ^-- panic occurred here if debug_assertion is enabled
    //     but if debug_assertion is disabled, no error happened.
    .map(|v| v * 2);
assert!(matches!(target, Err(_)), "Should be Err");
```

## Motivation

This crate makes it easy to declare and validate intermediate assumptions in `Result`/`Option` method chains.
It panics in debug builds for rapid feedback, and in release builds it minimizes runtime overhead and favors fail-safe behavior to avoid unexpected panics.

This is especially useful for applications (e.g., games) where avoiding mid-run panics is important.


## License

Copyright (c) 2025 Kitsunesaki Mezumona

Licensed under either of

* [Apache License, Version 2.0](./LICENSE-APACHE)
* [MIT license](./LICENSE-MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
