# jstime Core Tests

This directory contains integration tests for jstime_core.

## Test Organization

### Test Categories

- **`test_api.rs`** - Core API tests (run_script, import)
- **`test_builtins.rs`** - Basic functionality tests for built-in APIs
- **`test_conformance_*.rs`** - Standards compliance tests (see [CONFORMANCE_TESTS.md](./CONFORMANCE_TESTS.md))
- **`test_*.rs`** - Feature-specific tests (timers, fetch, fs, webassembly)

### Supporting Files

- **`common/mod.rs`** - Shared test utilities
- **`fixtures/`** - Test data and sample files

## Writing Tests

Basic pattern:

```rust
use jstime_core as jstime;
mod common;

#[test]
fn test_something() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    
    let result = jstime.run_script("console.log('test')", "test");
    assert!(result.is_ok());
}
```

## Running Tests

```bash
cargo test                    # All tests
cargo test test_name          # Specific test
cargo test -- --nocapture     # Show output
```
