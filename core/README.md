# jstime_core

The core JavaScript runtime library for jstime. This crate provides the fundamental functionality for executing JavaScript code using the V8 engine.

## Overview

`jstime_core` is designed to be:
- **Embeddable**: Easy to integrate into Rust applications
- **Minimal**: Only essential APIs included
- **Performant**: Optimized for speed and low memory usage
- **Standards-compliant**: Follows web standards (WHATWG, W3C)

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
jstime_core = "0.65.0"
```

### Basic Example

```rust
use jstime_core as jstime;

fn main() {
    // Initialize V8
    jstime::init(None);
    
    // Create runtime instance
    let options = jstime::Options::default();
    let mut runtime = jstime::JSTime::new(options);
    
    // Execute JavaScript
    match runtime.run_script("console.log('Hello from jstime!')", "script.js") {
        Ok(result) => println!("Result: {}", result),
        Err(error) => eprintln!("Error: {}", error),
    }
}
```

### Module Import Example

```rust
use jstime_core as jstime;

fn main() {
    jstime::init(None);
    
    let options = jstime::Options::default();
    let mut runtime = jstime::JSTime::new(options);
    
    // Import and execute a module
    match runtime.import("./my-module.js") {
        Ok(_) => println!("Module executed successfully"),
        Err(error) => eprintln!("Error: {}", error),
    }
}
```

### Performance: JIT Warmup

For performance-critical code or benchmarking, enable JIT warmup to allow V8's TurboFan compiler to optimize the code:

```rust
use jstime_core as jstime;

fn main() {
    jstime::init(None);
    
    // Create runtime with 10 warmup iterations
    let options = jstime::Options::default()
        .with_warmup(10);
    let mut runtime = jstime::JSTime::new(options);
    
    // Script will be executed 10 times for warmup, then once for actual execution
    match runtime.run_script("/* compute-intensive code */", "bench.js") {
        Ok(result) => println!("Result: {}", result),
        Err(error) => eprintln!("Error: {}", error),
    }
}
```

The warmup runs execute the script multiple times before the actual execution, allowing V8's JIT compiler to profile and optimize hot code paths. This is particularly useful for:
- Benchmarking JavaScript code
- Performance-critical scripts that will be executed multiple times
- Testing optimized execution paths

**Note**: Use warmup judiciously - it adds upfront cost. Default is 0 (no warmup) for optimal startup time.

## Built-in APIs

jstime_core provides these JavaScript APIs:

### Core APIs
- **Console**: `console.log()`, `console.error()`, etc.
- **Timers**: `setTimeout()`, `setInterval()`
- **Performance**: `performance.now()`
- **Microtasks**: `queueMicrotask()`

### Web APIs
- **Fetch**: `fetch()`, `Headers`, `Request`, `Response`
- **Streams**: `ReadableStream`, `WritableStream`, `TransformStream`
- **URL**: `URL`, `URLSearchParams`
- **Events**: `Event`, `EventTarget`

### Data APIs
- **Base64**: `atob()`, `btoa()`
- **Structured Clone**: `structuredClone()`
- **Text Encoding**: `TextEncoder`, `TextDecoder`

### Cryptography
- **Crypto**: `crypto.getRandomValues()`, `crypto.randomUUID()`, `crypto.subtle.*` (digest, sign, verify, encrypt, decrypt, generateKey, importKey, exportKey)

### System APIs
- **File System**: `node:fs/promises` module
- **Process**: `process.env`, `process.argv`, `process.cwd()`, `process.exit()`, `process.stdout`, `process.stderr`, `process.stdin`
- **Buffer**: `node:buffer` module for binary data operations
- **UDP Sockets**: `node:dgram` module for UDP networking

### Advanced Features
- **ES Modules**: `import`/`export` with top-level `await`
- **WebAssembly**: Full WebAssembly support via V8

See [docs/README.md](../docs/README.md) for overview and [docs/apis/](../docs/apis/) for detailed API documentation.

## Project Structure

```
core/
├── src/
│   ├── builtins/          # Built-in JavaScript APIs
│   │   ├── README.md      # Built-ins documentation
│   │   ├── *_impl.rs      # Rust implementations
│   │   └── *.js           # JavaScript polyfills
│   ├── event_loop.rs      # Event loop implementation
│   ├── isolate_state.rs   # V8 isolate state management
│   ├── js_loading.rs      # Script compilation
│   ├── module.rs          # ES module system
│   ├── script.rs          # Script execution
│   └── lib.rs             # Public API
├── tests/                 # Integration tests
│   ├── README.md          # Testing documentation
│   ├── common/            # Test utilities
│   ├── fixtures/          # Test fixtures
│   └── test_*.rs          # Test files
├── Cargo.toml             # Package configuration
└── README.md              # This file
```

## Documentation

- [README.md](../docs/README.md) - Features overview
- [Web APIs](../docs/apis/web-apis.md) - Detailed web API documentation
- [System APIs](../docs/apis/system.md) - Process and file system APIs
- [Module System](../docs/apis/modules.md) - ES Modules and WebAssembly
- [Built-ins README](./src/builtins/README.md) - Built-in API implementation guide
- [Tests README](./tests/README.md) - Testing guide
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Development workflow
- [ARCHITECTURE.md](../ARCHITECTURE.md) - Architecture overview

## Development

### Building

```bash
# Debug build
cargo build -p jstime_core

# Release build
cargo build -p jstime_core --release
```

### Testing

```bash
# Run all tests
cargo test -p jstime_core

# Run with output
cargo test -p jstime_core -- --nocapture
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../LICENSE) for details.

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.
