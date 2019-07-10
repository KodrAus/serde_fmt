# `serde_fmt`

[![Build Status](https://travis-ci.com/KodrAus/serde_fmt.svg?branch=master)](https://travis-ci.com/KodrAus/serde_fmt)
[![Latest version](https://img.shields.io/crates/v/serde_fmt.svg)](https://crates.io/crates/serde_fmt)
[![Documentation Latest](https://docs.rs/serde_fmt/badge.svg)](https://docs.rs/serde_fmt)

Convert any `serde::Serialize` into a `std::fmt::Debug`:

```rust
fn takes_serialize(v: impl Serialize) {
    dgb!(serde_fmt::to_debug(&v));

    // Do something with `v`
}
```

## Supported `rustc`

This library requires a very recent **nightly** compiler.
