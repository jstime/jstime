# GitHub Copilot Instructions for jstime

## Project Overview

jstime is a minimal and performant JavaScript runtime built on top of the V8 JavaScript engine, written in Rust. The project provides a CLI tool and an embeddable core library for executing JavaScript code.

### Architecture

- **`jstime_core`** (`core/`): The core runtime library that wraps V8 and provides JavaScript APIs
  - Built-in APIs organized by standards:
    - **WHATWG**: console, timers, fetch, URL, events (Event/EventTarget), base64, structured clone, microtask queue, streams, text encoding
    - **W3C**: performance, crypto (Web Cryptography API subset)
    - **Node.js compatible**: file system (fs/promises), process
  - Module loading system with ES modules and JSON modules support
  - Event loop implementation for async operations
  - WebAssembly support via V8
- **`jstime`** (`cli/`): Command-line interface with REPL support
  - Interactive REPL with JavaScript auto-completion and command history
  - Script execution from files or stdin
  - V8 flags configuration

## Coding Standards

### Rust Conventions

- **Edition**: Rust 2024
- **Formatting**: Code must pass `cargo fmt --all -- --check` (warnings treated as errors). Auto-format with `cargo fmt`.
- **Linting**: Code must pass `cargo clippy --all-targets -- -D warnings` (warnings treated as errors)
- **Testing**: Run `cargo test` before committing
- **Error Handling**: Use `Result<T, E>` for fallible operations; prefer descriptive error messages

### Code Style

- Follow standard Rust naming conventions:
  - `snake_case` for functions, variables, modules
  - `PascalCase` for types, traits, enums
  - `SCREAMING_SNAKE_CASE` for constants
- Use `pub(crate)` for internal APIs that don't need to be public
- Keep functions focused and single-purpose
- Prefer explicit error handling over panicking in library code
- Add doc comments (`///`) for public APIs

## Key Dependencies

- **v8**: V8 JavaScript engine bindings
- **ada-url**: Fast URL parsing library
- **ureq**: HTTP client for fetch API implementation with connection pooling
- **rustc-hash**: Fast non-cryptographic hashing for module maps
- **filetime**: File timestamp manipulation for fs API
- **ring**: Cryptographic operations for crypto API
- **align-data**: Data alignment utilities
- **smallvec**: Stack-allocated vectors to reduce heap allocations
- **base64-simd**: SIMD-accelerated base64 encoding/decoding
- **getrandom**: Fast random number generation for crypto API
- **rustyline**: REPL implementation with line editing (in cli crate)
- **structopt**: CLI argument parsing (in cli crate)
- **dirs**: User directories helper (in cli crate)

## Working with V8

### Common Patterns

1. **Scopes**: V8 requires careful scope management
   ```rust
   v8::scope!(let scope, &mut isolate);
   let context = v8::Local::new(scope, global_context);
   ```

2. **External Functions**: Register Rust functions callable from JavaScript
   ```rust
   fn my_function(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut retval: v8::ReturnValue) {
       // Implementation
   }
   ```

3. **Isolate State**: Access per-isolate state using `IsolateState::get(isolate)`

4. **Global Values**: Use `v8::Global` for values that persist beyond a scope

### Memory Management

- V8 uses garbage collection; be careful with raw pointers
- Use `v8::Global` for long-lived JavaScript values
- Always work within proper `HandleScope`s
- Clean up resources in Drop implementations

## Built-in APIs Implementation

Built-in APIs are located in `core/src/builtins/` and organized by standards body:

**WHATWG Standards** (`whatwg/`):
- **console_impl.rs / console.js**: Console API (console.log, console.error, etc.)
- **timers_impl.rs / timers.js**: setTimeout, setInterval, clearTimeout, clearInterval
- **fetch_impl.rs / fetch.js**: Fetch API (fetch, Headers, Request, Response)
- **url_impl.rs / url.js**: URL and URLSearchParams
- **event_impl.rs / event.js**: Event and EventTarget
- **base64_impl.rs / base64.js**: atob() and btoa() for base64 encoding/decoding
- **structured_clone_impl.rs / structured_clone.js**: structuredClone() for deep cloning
- **queue_microtask_impl.rs / queue_microtask.js**: queueMicrotask()
- **streams_impl.rs / streams.js**: ReadableStream, WritableStream, TransformStream
- **text_encoding_impl.rs / text_encoding.js**: TextEncoder and TextDecoder for UTF-8

**W3C Standards** (`w3c/`):
- **performance_impl.rs / performance.js**: performance.now() and performance.timeOrigin
- **crypto_impl.rs / crypto.js**: crypto.getRandomValues(), crypto.randomUUID(), crypto.subtle.digest()

**Node.js Compatible** (`node/`):
- **fs_impl.rs / fs.js**: File system API (node:fs/promises module)
- **process_impl.rs / process.js**: process.env, process.argv, process.cwd(), process.exit(), process.stdout, process.stderr, process.stdin
- **buffer_impl.rs / buffer.js**: Node.js Buffer API (node:buffer module)
- **dgram_impl.rs / dgram.js**: UDP/Datagram sockets API (node:dgram module)
- **events.js**: Node.js EventEmitter wrapper around WHATWG EventTarget

**Polyfills** (`polyfills/`):
- **date_locale.js**: Date.prototype.toLocaleString() and related methods

### Adding New Built-ins

1. Create implementation in `core/src/builtins/` in the appropriate subdirectory:
   - `whatwg/` for WHATWG standards (Fetch, URL, Console, Events, etc.)
   - `w3c/` for W3C standards (Performance, etc.)
   - `node/` for Node.js compatible APIs (fs/promises, etc.)
2. Create both `*_impl.rs` (Rust) and `*.js` (JavaScript polyfill) files
3. Register in `core/src/builtins/mod.rs` with external references
4. Initialize in the global context
5. Write tests in `core/tests/` (both feature tests and conformance tests)
6. Update documentation in the appropriate file:
   - Web APIs: `docs/apis/web-apis.md`
   - Text Encoding/Crypto: `docs/apis/encoding-crypto.md`
   - System APIs: `docs/apis/system.md`
   - Module System: `docs/apis/modules.md`
   - Runtime features: `docs/runtime.md`
   - Update `docs/README.md` if adding a new API category
7. **Update REPL autocomplete** in `cli/main.rs`:
   - Add global names (classes, functions, objects) to the `keywords` list in `JsCompleter::complete()`
   - Add property completions for objects with methods/properties to the `properties` match statement

## Testing

### Test Organization

- **`core/tests/test_api.rs`**: Core API tests (run_script, import)
- **`core/tests/test_builtins.rs`**: Built-in function tests
- **`core/tests/test_conformance_*.rs`**: WHATWG/W3C spec conformance tests
  - `test_conformance_base64.rs`: Base64 encoding (29 tests)
  - `test_conformance_console.rs`: Console API (13 tests)
  - `test_conformance_crypto.rs`: Crypto API (34 tests)
  - `test_conformance_event.rs`: Event and EventTarget (33 tests)
  - `test_conformance_fetch.rs`: Fetch API (32 tests)
  - `test_conformance_json_modules.rs`: JSON module imports (12 tests)
  - `test_conformance_performance.rs`: Performance API (19 tests)
  - `test_conformance_streams.rs`: Streams API (26 tests)
  - `test_conformance_structured_clone.rs`: Structured clone (21 tests)
  - `test_conformance_text_encoding.rs`: Text Encoding API (39 tests)
  - `test_conformance_timers.rs`: Timers API (17 tests)
  - `test_conformance_url.rs`: URL API (26 tests)
  - `test_conformance_webassembly.rs`: WebAssembly API (28 tests)
- **`core/tests/test_*.rs`**: Feature-specific tests (timers, fetch, fs, process, crypto, webassembly)
- **`core/tests/common/mod.rs`**: Shared test utilities (setup guards, helper functions)
- **`core/tests/fixtures/`**: Test data and sample files organized by feature

### Test Patterns

```rust
use jstime_core as jstime;
mod common;

#[test]
fn test_example() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script("1 + 1", "test");
    assert_eq!(result.unwrap(), "2");
}
```

For conformance tests, see [core/tests/CONFORMANCE_TESTS.md](../core/tests/CONFORMANCE_TESTS.md) for detailed coverage information.

### Running Tests

- All tests: `cargo test`
- Specific test: `cargo test test_name`
- With output: `cargo test -- --nocapture`

## Build and CI

### Local Development

```bash
cargo build          # Build the project
cargo test           # Run tests
cargo fmt            # Format code
cargo clippy --all-targets  # Lint code (including tests)
cargo run            # Run REPL
cargo run -- file.js # Execute JavaScript file
```

### Optimization

The project uses aggressive optimization for release builds:
- **LTO (Link-Time Optimization)**: Enabled for cross-crate optimizations
- **codegen-units = 1**: Better runtime performance
- **opt-level = 3**: Maximum optimization
- **strip = true**: Smaller binary size

See [PERFORMANCE.md](../PERFORMANCE.md) for detailed performance information.

### CI Requirements

All PRs must pass:
- Build on stable, beta, and nightly Rust
- All tests must pass
- Code must be formatted (`cargo fmt --all -- --check`)
- No clippy warnings (`cargo clippy --all-targets -- -D warnings`)

## Module System

jstime supports ES modules with:
- `import` and `export` statements
- Top-level `await`
- Dynamic `import()`
- File-based module resolution
- JSON module imports (import data from './data.json')
- `import.meta.url` for getting current module's URL

### Module Loading

- Located in `core/src/js_loading.rs` and `core/src/module.rs`
- Uses V8's module system
- Resolves file paths relative to importing module
- JSON files detected by `.json` extension and wrapped as ES modules

## Event Loop

- Implementation in `core/src/event_loop.rs`
- Handles timers, fetch requests, file operations, and other async operations
- Uses `std::sync::mpsc` for communication between JS and Rust
- Integrates with V8's microtask queue
- Performance optimizations:
  - Pre-allocated vectors for ready timers
  - Early returns when no work to do
  - Inlined hot path functions

## Common Development Tasks

### Adding a New JavaScript API

1. Choose the appropriate subdirectory based on the standard:
   - `core/src/builtins/whatwg/` for WHATWG standards
   - `core/src/builtins/w3c/` for W3C standards
   - `core/src/builtins/node/` for Node.js compatible APIs
2. Create both `your_api_impl.rs` (Rust) and `your_api.js` (JavaScript polyfill) files
3. Define the Rust callback functions with V8 bindings
4. Register in `builtins/mod.rs` external references
5. Initialize in the global context
6. Write tests in `core/tests/` (both feature tests and conformance tests)
7. Document in the appropriate file:
   - Web APIs: `docs/apis/web-apis.md`
   - Text Encoding/Crypto: `docs/apis/encoding-crypto.md`
   - System APIs: `docs/apis/system.md`
   - Module System: `docs/apis/modules.md`
   - Runtime features: `docs/runtime.md`
   - Update `docs/README.md` if adding a new API category
8. **Update REPL autocomplete** in `cli/main.rs`:
   - Add global names (classes, functions, objects) to the `keywords` list in `JsCompleter::complete()`
   - Add property completions for objects with methods/properties to the `properties` match statement

### Debugging

- Use `println!` or `eprintln!` for quick debugging
- Use `dbg!()` macro for expression debugging
- V8 flags: `--v8-options="--help"` to see available V8 options
- REPL is useful for testing JavaScript behavior

### Error Handling

- Return descriptive error messages to JavaScript
- Use V8's exception system for JavaScript errors
- Propagate Rust errors as JavaScript exceptions when appropriate

## Documentation

- Keep README.md updated with user-facing features
- Update the appropriate documentation file when adding APIs:
  - Web APIs: `docs/apis/web-apis.md`
  - Text Encoding/Crypto: `docs/apis/encoding-crypto.md`
  - System APIs: `docs/apis/system.md`
  - Module System: `docs/apis/modules.md`
  - Runtime features: `docs/runtime.md`
  - Main index: `docs/README.md`
- See ARCHITECTURE.md for detailed architecture information
- See PERFORMANCE.md for performance optimization details
- See CONTRIBUTING.md for development workflow
- Add inline doc comments for public APIs
- Follow existing documentation style
- Update conformance test documentation in core/tests/CONFORMANCE_TESTS.md

## Best Practices

1. **Minimize unsafe code**: Avoid `unsafe` unless necessary for V8 FFI
2. **Test thoroughly**: Add both unit tests and conformance tests
3. **Keep it minimal**: jstime aims to be lightweight; avoid feature bloat
4. **Follow specs**: Implement APIs according to WHATWG/W3C specifications
5. **Performance matters**: Profile and optimize hot paths; use pre-allocation where possible
6. **Error messages**: Provide clear, actionable error messages
7. **API stability**: Maintain backward compatibility for public APIs
8. **Use FxHashMap**: For internal hash maps with small keys (faster than std HashMap)
9. **Inline hot paths**: Add `#[inline]` to frequently called functions
10. **Connection pooling**: Reuse HTTP connections in fetch API implementation

## Resources

- [V8 Embedder's Guide](https://v8.dev/docs/embed)
- [Rust V8 Bindings](https://docs.rs/v8/)
- [WHATWG Standards](https://spec.whatwg.org/)
- [W3C Standards](https://www.w3.org/TR/)
- [Node.js Documentation](https://nodejs.org/api/)
- [Project Repository](https://github.com/jstime/jstime)
- [Architecture Documentation](../ARCHITECTURE.md)
- [Performance Documentation](../PERFORMANCE.md)
- [Features Overview](../docs/README.md)
- [Web APIs](../docs/apis/web-apis.md)
- [System APIs](../docs/apis/system.md)
- [Text Encoding & Crypto](../docs/apis/encoding-crypto.md)
- [Module System](../docs/apis/modules.md)
- [JavaScript Runtime](../docs/runtime.md)

## Getting Help

- Check existing issues and discussions
- Review conformance tests for examples
- Read V8 documentation for engine-specific questions
- See [ARCHITECTURE.md](../ARCHITECTURE.md) for detailed architecture
- See [core/tests/README.md](../core/tests/README.md) for testing patterns
- See [core/src/builtins/README.md](../core/src/builtins/README.md) for built-in API structure
