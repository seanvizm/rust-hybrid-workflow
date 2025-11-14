# WASI (WebAssembly System Interface) Implementation Guide

## Overview

This project includes **WASI (WebAssembly System Interface) capability** through Wasmtime, enabling WebAssembly modules to interact with system resources in a secure, standardized way.

## Current Status

✅ **WASI Dependencies Included**: `wasmtime-wasi` 26.0 is available  
✅ **Implementation Example Provided**: See `src/runners/wasm_runner_wasi.rs.example`  
📝 **Ready to Enable**: Follow the steps below to activate WASI support

> **Note**: WASI is currently optional and disabled by default for security and simplicity. The default WASM runner works with pure WebAssembly modules.

## What is WASI?

WASI is a system interface for WebAssembly that provides:
- **Standardized APIs** for system operations
- **Secure sandboxing** with capability-based security
- **Cross-platform portability** (Linux, macOS, Windows)
- **System resource access** through well-defined interfaces

Think of WASI as "POSIX for WebAssembly" - it provides a standard way for WASM modules to interact with the operating system.

## WASI Capabilities Enabled

Our implementation provides WASM modules with access to:

### ✅ Implemented Features

1. **Standard I/O (stdio)**
   - Read from `stdin`
   - Write to `stdout` and `stderr`
   - Perfect for logging and debugging

2. **Environment Variables**
   - Access to process environment
   - Read system configuration
   - Pass configuration to WASM modules

3. **Command-line Arguments**
   - Pass arguments to WASM modules
   - Configure module behavior dynamically

4. **System Clock**
   - Read current time
   - Measure execution duration
   - Timestamp events

5. **Random Number Generation**
   - Cryptographically secure random numbers
   - Perfect for security-sensitive operations

### 🔒 Security Model

WASI uses **capability-based security**:
- WASM modules only get access to explicitly granted capabilities
- No ambient authority (unlike traditional processes)
- Fine-grained control over system access
- Sandboxed execution by default

## Implementation Details

###  Enabling WASI Support

The project includes everything needed for WASI support. To enable it:

**Option 1: Use the Example Implementation**
```bash
# Replace the default runner with the WASI-enabled version
cp src/runners/wasm_runner_wasi.rs.example src/runners/wasm_runner.rs

# Rebuild
cargo build --release
```

**Option 2: Manual Implementation**

Edit `src/runners/wasm_runner.rs` and uncomment the WASI code sections marked with comments.

### Code Architecture (WASI-Enabled Version)

```rust
use wasmtime_wasi::sync::WasiCtxBuilder;

// Build WASI context with capabilities
let wasi = WasiCtxBuilder::new()
    .inherit_stdio()      // Enable stdio access
    .inherit_args()?      // Enable argument passing
    .inherit_env()?       // Enable environment variables
    .build();

let mut store = Store::new(&engine, wasi);

// Linker connects WASI functions to WASM module
let mut linker = Linker::new(&engine);
wasmtime_wasi::sync::add_to_linker(&mut linker, |s| s)?;

// Instantiate module with WASI support
let instance = linker.instantiate(&mut store, &module)?;
```

### How It Works

1. **WASI Context Creation**: Build a `WasiCtx` with specific capabilities
2. **Linker Setup**: Connect WASI functions to the WASM runtime
3. **Module Instantiation**: Load WASM module with WASI support
4. **Function Execution**: WASM code can now call WASI functions

## Usage Examples

### Example 1: Basic WASI Module (Rust → WASM)

```rust
// Compile this to WASM with: cargo build --target wasm32-wasi
use std::io::{self, Write};

fn main() {
    // Use stdio (enabled by WASI)
    println!("Hello from WASI!");
    eprintln!("This goes to stderr");
    
    // Read environment variables (enabled by WASI)
    if let Ok(path) = std::env::var("PATH") {
        println!("PATH: {}", path);
    }
    
    // Access system time (enabled by WASI)
    println!("Current time: {:?}", std::time::SystemTime::now());
}
```

### Example 2: WASI in Workflow

```lua
-- workflows/wasi_example.lua
workflow = {
  name = "wasi_demo",
  steps = {
    run_wasi_module = {
      language = "wasm",
      module = "my_wasi_module.wasm",
      func = "run"
      -- Module can now use stdio, env vars, etc.
    }
  }
}
```

### Example 3: Advanced WASI Features

```rust
// Advanced WASI example with file system access
// To enable file system: .preopened_dir(path, "/")

use std::fs;
use std::env;

fn main() {
    // Environment access
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);
    
    // Environment variables
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
    
    // Time access
    let start = std::time::Instant::now();
    // ... do work ...
    println!("Elapsed: {:?}", start.elapsed());
    
    // Random numbers (if WASI supports it)
    use std::collections::hash_map::RandomState;
    let random_state = RandomState::new();
    println!("Random state: {:?}", random_state);
}
```

## Building WASI Modules

### Rust to WASI

```bash
# Add WASI target
rustup target add wasm32-wasi

# Build your Rust project
cargo build --target wasm32-wasi --release

# Output: target/wasm32-wasi/release/your_module.wasm
```

### C/C++ to WASI

```bash
# Using WASI SDK
clang --target=wasm32-wasi -o module.wasm module.c

# Using Emscripten
emcc -o module.wasm module.c
```

### AssemblyScript to WASI

```bash
# Install AssemblyScript
npm install -g assemblyscript

# Compile with WASI support
asc module.ts -o module.wasm --runtime stub
```

## Optional File System Access

To enable file system access (disabled by default for security):

```rust
use cap_std::fs::Dir;
use cap_std::ambient_authority;

let wasi = WasiCtxBuilder::new()
    .inherit_stdio()
    .inherit_args()
    .inherit_env()
    // Grant access to specific directory
    .preopened_dir(
        Dir::open_ambient_dir("/path/to/data", ambient_authority())?,
        "/"
    )
    .build();
```

⚠️ **Security Warning**: Only grant file system access when absolutely necessary and limit the scope as much as possible.

## WASI vs Non-WASI WASM

### Non-WASI WASM
- Pure computation only
- No system access
- Limited I/O capabilities
- Return values through function returns

### WASI-enabled WASM
- System integration
- File system access (if granted)
- Network access (if granted)
- Rich I/O capabilities
- Standard library support

## Performance Considerations

### WASI Overhead
- Minimal performance impact (~1-5% overhead)
- Capability checks are fast
- No significant memory overhead

### Best Practices
1. **Use WASI when you need system access**
2. **Use pure WASM for pure computation**
3. **Grant minimal required capabilities**
4. **Test with security in mind**

## Troubleshooting

### Common Issues

**Module fails to load:**
```
Error: unknown import: `wasi_snapshot_preview1::fd_write`
```
**Solution**: Ensure module is compiled with WASI target (`wasm32-wasi`)

**Permission denied:**
```
Error: Capability not granted
```
**Solution**: Add required capability in `WasiCtxBuilder`

**Module hangs:**
```
(Waiting indefinitely on stdin)
```
**Solution**: Ensure stdio is properly configured or mocked

## Testing WASI Modules

### Unit Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wasi_execution() {
        let inputs = HashMap::new();
        let result = run_wasm_step(
            "test",
            "test_wasi.wasm",
            Some("main"),
            &inputs
        );
        assert!(result.is_ok());
    }
}
```

### Integration Test

```bash
# Run WASI workflow
cargo run wasi_workflow.lua

# Expected output:
# ✅ Python: Data prepared for WASI processing
# Hello from WASI!
# 📊 WASI Execution Results:
#   Status: success
# 🎯 WASI Workflow Summary:
#   ✓ WASI-enabled WASM execution successful
```

## Comparison with Other Approaches

| Feature | Pure WASM | WASI | Native Code |
|---------|-----------|------|-------------|
| Security | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ |
| Performance | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| System Access | ❌ | ✅ | ✅ |
| Portability | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ |
| I/O Operations | ❌ | ✅ | ✅ |

## Future Enhancements

### Planned WASI Features
- [ ] **Network Sockets** - TCP/UDP communication
- [ ] **Advanced File System** - Full filesystem hierarchy access
- [ ] **Process Management** - Spawn child processes
- [ ] **Threading** - WASI threads support
- [ ] **Custom Host Functions** - Domain-specific extensions

### WASI Roadmap
1. **Phase 1** (Current): Basic WASI (stdio, env, args) ✅
2. **Phase 2**: File system access (optional)
3. **Phase 3**: Network capabilities
4. **Phase 4**: Advanced features (threads, sockets)

## References

- [WASI Official Website](https://wasi.dev/)
- [Wasmtime Documentation](https://docs.wasmtime.dev/)
- [WASI Specification](https://github.com/WebAssembly/WASI)
- [Rust WASI Target](https://doc.rust-lang.org/rustc/platform-support/wasm32-wasi.html)

## Contributing

We welcome contributions to enhance WASI support! Areas for contribution:
- Additional WASI capabilities
- Example WASI modules
- Performance optimizations
- Security enhancements
- Documentation improvements

## License

WASI implementation follows the same license as the main project (Apache 2.0).

---

**Questions?** Open an issue on GitHub or check the [main README](../README.md).
