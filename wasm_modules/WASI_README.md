# WASI Example Module

This directory contains example Rust code that can be compiled to WASM with WASI support.

## Building the WASI Module

```bash
# From this directory
rustup target add wasm32-wasi
cargo build --target wasm32-wasi --release

# Output will be in: target/wasm32-wasi/release/wasi_example.wasm
```

## Using in Workflows

```lua
workflow = {
  steps = {
    run_wasi = {
      language = "wasm",
      module = "wasm_modules/target/wasm32-wasi/release/wasi_example.wasm",
      func = "run"
    }
  }
}
```

## WASI Capabilities Demonstrated

This example shows:
- Standard I/O (println!, eprintln!)
- Environment variable access
- Command-line argument handling
- System time access
- Random number generation (via RandomState)
