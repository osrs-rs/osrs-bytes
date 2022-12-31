# osrs-bytes

[![Build](https://github.com/osrs-rs/osrs-bytes/workflows/build/badge.svg)](https://github.com/osrs-rs/osrs-bytes)
[![API](https://docs.rs/osrs-bytes/badge.svg)](https://docs.rs/osrs-bytes)
[![Crate](https://img.shields.io/crates/v/osrs-bytes)](https://crates.io/crates/osrs-bytes)
[![dependency status](https://deps.rs/repo/github/osrs-rs/osrs-bytes/status.svg)](https://deps.rs/repo/github/osrs-rs/osrs-bytes)
[![Discord](https://img.shields.io/discord/926860365873184768?color=5865F2)](https://discord.gg/CcTa7TZfSc)

Traits for working with bytes in Oldschool RuneScape.

## Installation

Add this crate as a dependency to your `Cargo.toml` file.

```toml
[dependencies]
osrs-bytes = "0.0.0"
```

## Example

```rust
use osrs_bytes::{ReadExt, WriteExt};
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

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `osrs-bytes` by you, shall be licensed as MIT, without any additional terms or conditions.
