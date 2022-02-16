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
osrs-buffer = "0.4.0"
```

## Example

```rust
use osrs_buffer::ByteBuffer;

fn main() {
    let mut buf = ByteBuffer::new(1);
    buf.write_i8(123);

    assert_eq!(buf.read_i8(), 123);
}
```

## Contributing

This repository is open for contributions. For bigger changes it is advised to [open an issue](https://github.com/runecore/osrs-buffer/issues/new) to discuss these matters.

## License

`osrs-buffer` is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.
