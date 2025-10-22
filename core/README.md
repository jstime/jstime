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
jstime_core = "0.59.0"
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

## Built-in APIs

jstime_core provides these JavaScript APIs:

### Core APIs
- **Console**: `console.log()`, `console.error()`, etc.
- **Timers**: `setTimeout()`, `setInterval()`
- **Performance**: `performance.now()`
- **Microtasks**: `queueMicrotask()`

### Web APIs
- **Fetch**: `fetch()`, `Headers`, `Request`, `Response`
- **URL**: `URL`, `URLSearchParams`
- **Events**: `Event`, `EventTarget`

### Data APIs
- **Base64**: `atob()`, `btoa()`
- **Structured Clone**: `structuredClone()`

### System APIs
- **File System**: `node:fs/promises` module

### Advanced Features
- **ES Modules**: `import`/`export` with top-level `await`
- **WebAssembly**: Full WebAssembly support via V8

See [FEATURES.md](../docs/FEATURES.md) for detailed API documentation.

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

- [FEATURES.md](../docs/FEATURES.md) - Complete API documentation
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
