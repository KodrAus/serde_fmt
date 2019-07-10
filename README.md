# `serde_fmt`

Convert any `serde::Serialize` into a `std::fmt::Debug`:

```rust
fn takes_serialize(v: impl Serialize) {
    dgb!(serde_fmt::to_debug(&v));

    // Do something with `v`
}
```
