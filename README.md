# NeoColor

A Rust-based Neovim plugin that highlights color values in real-time.

## Features

- **Hex Colors:**
  - `#RRGGBB`, `#RRGGBBAA` (in any file)
  - `#RGB`, `#RGBA` (only in CSS-family files: `css`, `scss`, `sass`, `less`, `postcss`, `stylus`, `html`, `js`, `ts`, `vue`, `svelte`, etc.)
- **Rust Specific:**
  - `rgb(0x3c3c3c)`
  - `rgba(0x3c3c3cff)`
  - `hsla(0.2, 0.8, 0.6, 1.0)`
- **Automatic Highlighting:** Updates on buffer entry and text changes.

## Installation (Lazy.nvim)

#### Using prebuilt binaries (Recommended)

```lua
{
  'Pajn/neocolor',
  build = "./scripts/install.sh", -- Or "powershell ./scripts/install.ps1" on Windows
  config = true,
}
```

#### Building from source

```lua
{
  'Pajn/neocolor',
  -- On macOS, you might need:
  -- build = "RUSTFLAGS='-C link-arg=-undefined -C link-arg=dynamic_lookup' cargo build --release && cp target/release/libneocolor.* lua/neocolor_lib.so",
  build = "cargo build --release && cp target/release/libneocolor.* lua/neocolor_lib.so",
  config = true,
}
```

## Building

Inside the `neocolor` directory:
```bash
cargo build --release
cp target/release/libneocolor.dylib lua/neocolor_lib.so (macOS)
cp target/release/libneocolor.so lua/neocolor_lib.so (Linux)
```
*(Note: On Windows, copy `target/release/neocolor.dll` to `lua/neocolor_lib.dll`)*
