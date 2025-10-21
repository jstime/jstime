# GitHub Copilot Instructions for jstime

## Project Overview

jstime is a minimal and performant JavaScript runtime built on top of the V8 JavaScript engine, written in Rust. The project provides a CLI tool and an embeddable core library for executing JavaScript code.

### Architecture

- **`jstime_core`** (`core/`): The core runtime library that wraps V8 and provides JavaScript APIs
  - Built-in APIs: console, timers, fetch, URL, performance, microtask queue
  - Module loading system with ES modules support
  - Event loop implementation for async operations
- **`jstime`** (`cli/`): Command-line interface with REPL support
  - Interactive REPL with JavaScript auto-completion
  - Script execution from files or stdin
  - V8 flags configuration

## Coding Standards

### Rust Conventions

- **Edition**: Rust 2021
- **Formatting**: Use `cargo fmt` with default rustfmt settings
- **Linting**: Code must pass `cargo clippy -- -D warnings` (warnings treated as errors)
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

- **v8** (140.2.0): V8 JavaScript engine bindings
- **ureq** (2.10): HTTP client for fetch API implementation
- **url** (2.5): URL parsing for URL API implementation
- **lazy_static** (1.5.0): Lazy static initialization
- **structopt**: CLI argument parsing (in cli crate)
- **rustyline**: REPL implementation with line editing (in cli crate)

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

Built-in APIs are located in `core/src/builtins/`:

- **console_impl.rs**: Console API (console.log, console.error, etc.)
- **timers_impl.rs**: setTimeout, setInterval, clearTimeout, clearInterval
- **fetch_impl.rs**: Fetch API (fetch, Headers, Request, Response)
- **url_impl.rs**: URL and URLSearchParams
- **performance_impl.rs**: performance.now()
- **queue_microtask_impl.rs**: queueMicrotask()

### Adding New Built-ins

1. Create implementation in `core/src/builtins/`
2. Register in `core/src/builtins/mod.rs`
3. Add external references for V8
4. Write conformance tests in `core/tests/`
5. Update documentation in `docs/FEATURES.md`

## Testing

### Test Organization

- **`core/tests/test_api.rs`**: Core API tests (run_script, import)
- **`core/tests/test_builtins.rs`**: Built-in function tests
- **`core/tests/test_conformance_*.rs`**: WHATWG/W3C spec conformance tests
- **`core/tests/common/mod.rs`**: Shared test utilities

### Test Patterns

```rust
#[test]
fn test_example() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script("1 + 1", "test");
    assert_eq!(result.unwrap(), "2");
}
```

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
cargo clippy         # Lint code
cargo run            # Run REPL
cargo run -- file.js # Execute JavaScript file
```

### CI Requirements

All PRs must pass:
- Build on stable, beta, and nightly Rust
- All tests must pass
- Code must be formatted (`cargo fmt --all -- --check`)
- No clippy warnings (`cargo clippy -- -D warnings`)

## Module System

jstime supports ES modules with:
- `import` and `export` statements
- Top-level `await`
- Dynamic `import()`
- File-based module resolution

### Module Loading

- Located in `core/src/js_loading.rs` and `core/src/module.rs`
- Uses V8's module system
- Resolves file paths relative to importing module

## Event Loop

- Implementation in `core/src/event_loop.rs`
- Handles timers, fetch requests, and other async operations
- Uses `std::sync::mpsc` for communication between JS and Rust
- Integrates with V8's microtask queue

## Common Development Tasks

### Adding a New JavaScript API

1. Create implementation file in `core/src/builtins/`
2. Define the Rust callback functions
3. Register in `builtins/mod.rs` external references
4. Initialize in the global context
5. Write tests in `core/tests/`
6. Document in `docs/FEATURES.md`

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
- Update FEATURES.md when adding APIs
- Add inline doc comments for public APIs
- Follow existing documentation style

## Best Practices

1. **Minimize unsafe code**: Avoid `unsafe` unless necessary for V8 FFI
2. **Test thoroughly**: Add both unit and conformance tests
3. **Keep it minimal**: jstime aims to be lightweight; avoid feature bloat
4. **Follow specs**: Implement APIs according to WHATWG/W3C specifications
5. **Performance matters**: Profile and optimize hot paths
6. **Error messages**: Provide clear, actionable error messages
7. **API stability**: Maintain backward compatibility for public APIs

## Resources

- [V8 Embedder's Guide](https://v8.dev/docs/embed)
- [Rust V8 Bindings](https://docs.rs/v8/)
- [WHATWG Standards](https://spec.whatwg.org/)
- [Project Repository](https://github.com/jstime/jstime)

## Getting Help

- Check existing issues and discussions
- Review conformance tests for examples
- Read V8 documentation for engine-specific questions
- Follow project governance in GOVERNANCE.md
