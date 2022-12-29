# osrs-buffer

[![Build](https://github.com/runecore/osrs-buffer/workflows/build/badge.svg)](https://github.com/runecore/osrs-buffer)
[![API](https://docs.rs/osrs-buffer/badge.svg)](https://docs.rs/osrs-buffer)
[![Crate](https://img.shields.io/crates/v/osrs-buffer)](https://crates.io/crates/osrs-buffer)
[![dependency status](https://deps.rs/repo/github/runecore/osrs-buffer/status.svg)](https://deps.rs/repo/github/runecore/osrs-buffer)
[![Discord](https://img.shields.io/discord/926860365873184768?color=5865F2)](https://discord.gg/CcTa7TZfSc)

A buffer for Oldschool Runescape data types.

## Deprecated

This crate has been deprecated in favor of [osrs-bytes](https://github.com/osrs-rs/osrs-bytes). `osrs-buffer` will not be updated anymore.

## Installation

Add this to your `Cargo.toml` file:

```toml
[dependencies]
osrs-buffer = "0.7.0"
```

## Example

```rust
use osrs_buffer::{ReadExt, WriteExt};
use std::io::{self, Cursor};

fn main() -> Result<(), io::Error> {
    // Read data from the cursor
    let mut csr = Cursor::new(vec![123]);
    assert_eq!(csr.read_i8()?, 123);

    // Write data to the vector
    let mut vec = Vec::new();
    vec.write_i8(124)?;
    assert_eq!(vec[0], 124);

    Ok(())
}
```

## License

This project is licensed under the [MIT license](license-mit).

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `osrs-buffer` by you, shall be licensed as MIT, without any additional terms or conditions.
