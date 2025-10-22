# jstime Development Guide

This guide provides information for developers who want to contribute to jstime or understand how to work with the codebase.

## Table of Contents

- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Building and Testing](#building-and-testing)
- [Development Workflow](#development-workflow)
- [Adding New Built-in APIs](#adding-new-built-in-apis)
- [Testing Guidelines](#testing-guidelines)
- [Code Style](#code-style)
- [Debugging](#debugging)
- [Common Tasks](#common-tasks)

## Getting Started

### Prerequisites

- **Rust**: Install the latest stable Rust toolchain from [rustup.rs](https://rustup.rs/)
- **Git**: For version control
- **Python**: Required for V8 build scripts (usually pre-installed on most systems)

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/jstime/jstime.git
cd jstime

# Build the project
cargo build

# Run tests
cargo test

# Run the REPL
cargo run
```

## Project Structure

jstime is organized as a Cargo workspace with two main crates:

```
jstime/
├── core/                   # Core runtime library (jstime_core)
│   ├── src/
│   │   ├── builtins/      # Built-in JavaScript APIs
│   │   ├── event_loop.rs  # Event loop implementation
│   │   ├── isolate_state.rs  # V8 isolate state management
│   │   ├── js_loading.rs  # JavaScript module loading
│   │   ├── module.rs      # ES module system
│   │   ├── script.rs      # Script execution
│   │   └── lib.rs         # Library entry point
│   └── tests/             # Integration tests
│       ├── common/        # Shared test utilities
│       ├── fixtures/      # Test fixtures
│       └── test_*.rs      # Test files
├── cli/                   # CLI tool (jstime binary)
│   ├── main.rs           # CLI entry point
│   ├── build.rs          # Build script
│   └── tests/            # CLI integration tests
├── docs/                  # Documentation
├── examples/              # Example scripts
└── Cargo.toml            # Workspace configuration
```

### Core Components

- **lib.rs**: Entry point that initializes V8 and creates JSTime instances
- **builtins/**: Implementation of JavaScript APIs (console, timers, fetch, etc.)
- **event_loop.rs**: Manages asynchronous operations (timers, fetch requests)
- **isolate_state.rs**: Per-isolate state storage and management
- **js_loading.rs**: Script compilation and execution
- **module.rs**: ES module loading and resolution

## Building and Testing

### Building

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (optimized)
cargo build --release

# Build only the core library
cargo build -p jstime_core

# Build only the CLI
cargo build -p jstime
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific package
cargo test -p jstime_core
cargo test -p jstime

# Run a specific test
cargo test test_console

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode (faster execution)
cargo test --release
```

### Code Quality Checks

```bash
# Check formatting
cargo fmt --all -- --check

# Format code
cargo fmt --all

# Run clippy (linter)
cargo clippy -- -D warnings

# Check compilation without building
cargo check
```

## Development Workflow

### Typical Development Cycle

1. **Make changes** to the code
2. **Format** the code: `cargo fmt --all`
3. **Check** compilation: `cargo check`
4. **Run** clippy: `cargo clippy -- -D warnings`
5. **Test** your changes: `cargo test`
6. **Commit** your changes with a descriptive message

### Using the REPL for Testing

The REPL is useful for quickly testing changes:

```bash
cargo run

# Or for release mode (faster):
cargo run --release
```

You can test your changes interactively:

```javascript
>> console.log("Testing my changes")
>> setTimeout(() => console.log("Timer works!"), 1000)
>> await fetch("https://api.github.com").then(r => r.json())
```

## Adding New Built-in APIs

To add a new JavaScript API to jstime, follow these steps:

### 1. Create Implementation Files

Create two files in `core/src/builtins/`:

- `your_api_impl.rs` - Rust implementation with V8 bindings
- `your_api.js` - JavaScript polyfill/wrapper code

### 2. Implement the Rust Side

```rust
// core/src/builtins/your_api_impl.rs
use v8;

pub fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: your_function.map_fn_to(),
        },
    ]
}

pub fn register_bindings(scope: &mut v8::HandleScope, bindings: v8::Local<v8::Object>) {
    let key = v8::String::new(scope, "yourFunction").unwrap();
    let func = v8::Function::new(scope, your_function).unwrap();
    bindings.set(scope, key.into(), func.into());
}

fn your_function(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    // Implementation
}
```

### 3. Implement the JavaScript Side

```javascript
// core/src/builtins/your_api.js
(function(bindings) {
  globalThis.yourAPI = function(...args) {
    return bindings.yourFunction(...args);
  };
})(globalThis.__jstime_bindings__);
```

### 4. Register in mod.rs

Update `core/src/builtins/mod.rs`:

```rust
mod your_api_impl;

pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    let mut refs = Vec::with_capacity(50);
    // ... existing references
    refs.extend(your_api_impl::get_external_references());
    refs
}

impl Builtins {
    pub(crate) fn create(scope: &mut v8::PinScope) {
        // ... existing bindings
        your_api_impl::register_bindings(scope, bindings);
        
        // ... existing builtins
        builtin!("./your_api.js");
    }
}
```

### 5. Write Tests

Create tests in `core/tests/`:

```rust
// core/tests/test_your_api.rs
use jstime_core as jstime;

mod common;

#[test]
fn test_your_api_basic() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    
    let result = jstime.run_script("yourAPI()", "test").unwrap();
    assert_eq!(result, "expected value");
}
```

### 6. Add Documentation

Update `docs/FEATURES.md` with documentation for your API.

### 7. Add Examples

Create an example in `examples/your-api-demo.js` demonstrating usage.

## Testing Guidelines

### Test Organization

- **`test_api.rs`**: Core API tests (run_script, import)
- **`test_builtins.rs`**: Basic built-in function tests
- **`test_conformance_*.rs`**: Standards compliance tests
- **`test_*.rs`**: Feature-specific tests

### Writing Tests

Follow these patterns:

```rust
#[test]
fn test_descriptive_name() {
    // Setup
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    
    // Execute
    let result = jstime.run_script("/* JavaScript code */", "test");
    
    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "expected output");
}

#[test]
fn test_error_case() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    
    let result = jstime.run_script("throw new Error('test')", "test");
    assert!(result.is_err());
}
```

### Conformance Tests

Conformance tests verify compliance with web standards:

- Test against official specifications (WHATWG, W3C)
- Cover edge cases and error conditions
- Document the specification being tested
- Group related tests together

## Code Style

### Rust Code

- **Edition**: Rust 2021
- **Formatting**: Use `cargo fmt` (enforced in CI)
- **Linting**: Must pass `cargo clippy -- -D warnings` (enforced in CI)
- **Naming conventions**:
  - `snake_case` for functions, variables, modules
  - `PascalCase` for types, traits, enums
  - `SCREAMING_SNAKE_CASE` for constants

### JavaScript Code

- Use modern ES6+ syntax
- Follow the style of existing builtin JavaScript files
- Keep polyfills minimal and focused

### Comments and Documentation

- Add doc comments (`///`) for public APIs
- Explain complex logic with inline comments
- Document any non-obvious behavior
- Reference relevant specifications when applicable

## Debugging

### Debugging Rust Code

```bash
# Build with debug symbols
cargo build

# Use rust-lldb or rust-gdb
rust-lldb target/debug/jstime
```

### Debugging JavaScript

Use console methods:

```javascript
console.log("Debug value:", value);
console.error("Error occurred:", error);
```

### V8 Flags

Pass V8 flags for debugging:

```bash
# See available V8 options
cargo run -- --v8-options="--help"

# Run with specific V8 flags
cargo run -- --v8-options="--trace-opt --trace-deopt"
```

### Common Issues

**V8 compilation errors**: Make sure you have Python installed and your Rust version is up to date.

**Test failures**: Check if you need to run `cargo clean` to clear old artifacts.

**Memory issues**: V8 can use significant memory; increase your system's available memory if needed.

## Common Tasks

### Update Dependencies

```bash
# Update dependencies
cargo update

# Update specific dependency
cargo update v8
```

### Check for Outdated Dependencies

```bash
# Install cargo-outdated
cargo install cargo-outdated

# Check for updates
cargo outdated
```

### Run Benchmarks

```bash
# Build in release mode
cargo build --release

# Time script execution
time ./target/release/jstime examples/performance-demo.js
```

### Profile Performance

Use profiling tools:

```bash
# On Linux, use perf
cargo build --release
perf record ./target/release/jstime script.js
perf report

# On macOS, use Instruments
cargo build --release
instruments -t "Time Profiler" ./target/release/jstime script.js
```

### Generate Documentation

```bash
# Generate and open documentation
cargo doc --open

# Generate documentation for all dependencies
cargo doc --open --document-private-items
```

### Clean Build Artifacts

```bash
# Remove build artifacts
cargo clean

# Remove only the target directory
rm -rf target/
```

## Additional Resources

- [CONTRIBUTING.md](./CONTRIBUTING.md) - Contribution guidelines
- [ARCHITECTURE.md](./ARCHITECTURE.md) - Architecture overview
- [docs/FEATURES.md](./docs/FEATURES.md) - API documentation
- [V8 Documentation](https://v8.dev/docs) - V8 JavaScript engine
- [Rust V8 Bindings](https://docs.rs/v8/) - rust-v8 crate documentation

## Getting Help

- **Issues**: Open an issue on [GitHub](https://github.com/jstime/jstime/issues)
- **Discussions**: Start a discussion on [GitHub Discussions](https://github.com/jstime/jstime/discussions)
- **Code Review**: Tag maintainers in your pull request

## Tips for Success

1. **Start small**: Begin with small changes and work your way up
2. **Read existing code**: Study how similar features are implemented
3. **Test thoroughly**: Write tests before fixing issues or adding features
4. **Ask questions**: Don't hesitate to ask for clarification in issues or PRs
5. **Follow conventions**: Match the style and patterns used in the existing codebase
6. **Document your changes**: Update relevant documentation when adding features

Happy coding! 🚀
