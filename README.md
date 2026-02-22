# NeoColor

A Rust-based Neovim plugin that highlights color values in real-time.

## Features

- **Hex Colors:** `#440033`, `#RRGGBBAA`, `#RGB`, `#RGBA` (in any file)
- **Rust Specific:**
  - `rgb(0x3c3c3c)`
  - `rgba(0x3c3c3cff)`
  - `hsla(0.2, 0.8, 0.6, 1.0)`
- **Automatic Highlighting:** Updates on buffer entry and text changes.

## Installation (Lazy.nvim)

```lua
{
  'Pajn/neocolor',
  dev = true,
  dir = '/path/to/neocolor', -- Path to this directory
  config = true, -- Automatically calls setup({})
}
```

## Building

Inside the `neocolor` directory:
```bash
RUSTFLAGS="-C link-arg=-undefined -C link-arg=dynamic_lookup -C link-arg=-L/path/to/libiconv" cargo build --release
cp target/release/libneocolor.dylib lua/neocolor_lib.so
```
*(Note: Use the correct path to libiconv for your system, e.g. in the Nix store if on NixOS/Darwin with Nix)*
