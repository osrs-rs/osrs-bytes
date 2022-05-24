# osrs-buffer

[![Build](https://github.com/runecore/osrs-buffer/workflows/build/badge.svg)](https://github.com/runecore/osrs-buffer)
[![API](https://docs.rs/osrs-buffer/badge.svg)](https://docs.rs/osrs-buffer)
[![Crate](https://img.shields.io/crates/v/osrs-buffer)](https://crates.io/crates/osrs-buffer)
[![dependency status](https://deps.rs/repo/github/runecore/osrs-buffer/status.svg)](https://deps.rs/repo/github/runecore/osrs-buffer)
[![Discord](https://img.shields.io/discord/926860365873184768?color=5865F2)](https://discord.gg/CcTa7TZfSc)

A buffer for Oldschool Runescape data types.

## Installation

Add this to your `Cargo.toml` file:

```toml
[dependencies]
osrs-buffer = "0.6.0"
```

## Example

```rust
use osrs_buffer::{ReadExt, WriteExt};

fn main() {
    let mut vec = Vec::new();
    vec.write_i8(123);

    assert_eq!(vec.read_i8(), 123);
}
```

## License

This project is licensed under the [MIT license](license-mit).

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `osrs-buffer` by you, shall be licensed as MIT, without any additional terms or conditions.
