# jstime Core Tests

This directory contains integration tests for jstime_core. The tests verify both basic functionality and standards compliance.

## Test Organization

### Test Categories

Tests are organized by purpose and scope:

#### API Tests (`test_api.rs`)
Basic tests for the core JSTime API:
- Script execution (`run_script`, `run_script_no_event_loop`)
- Module imports (`import`)
- JSON module support
- Error handling

#### Built-in Tests (`test_builtins.rs`)
Basic functionality tests for built-in APIs:
- API existence checks
- Basic operations
- Simple error cases
- Quick smoke tests

Each built-in should have basic tests here for fast feedback.

#### Conformance Tests (`test_conformance_*.rs`)
Standards compliance tests following official specifications:

| File | Spec | Coverage |
|------|------|----------|
| `test_conformance_base64.rs` | WHATWG HTML (Base64) | atob, btoa |
| `test_conformance_console.rs` | WHATWG Console | console.* methods |
| `test_conformance_event.rs` | DOM Events | Event, EventTarget |
| `test_conformance_fetch.rs` | WHATWG Fetch | fetch, Headers, Request, Response |
| `test_conformance_json_modules.rs` | ES Modules | JSON imports |
| `test_conformance_performance.rs` | HR Time | performance.now() |
| `test_conformance_structured_clone.rs` | HTML Structured Clone | structuredClone() |
| `test_conformance_timers.rs` | HTML Timers | setTimeout, setInterval |
| `test_conformance_url.rs` | WHATWG URL | URL, URLSearchParams |
| `test_conformance_webassembly.rs` | WebAssembly | WebAssembly.* |

See [CONFORMANCE_TESTS.md](./CONFORMANCE_TESTS.md) for details.

#### Feature Tests (`test_*.rs`)
In-depth tests for specific features:
- `test_timers.rs` - Timer behavior and edge cases
- `test_fetch.rs` - HTTP client functionality
- `test_fs.rs` - File system operations
- `test_webassembly.rs` - WebAssembly functionality

### Supporting Files

#### Common Utilities (`common/mod.rs`)
Shared test utilities:
- `setup()` - Initialize V8 and return a guard
- `create_jstime()` - Create a test JSTime instance
- Helper functions for common test patterns

#### Test Fixtures (`fixtures/`)
Sample files and data for tests:
```
fixtures/
├── fs/                    # File system test files
│   ├── test-readfile.txt
│   ├── test-readdir/
│   └── *.js
└── modules/              # Module loading test files
    ├── simple.js
    └── data.json
```

## Writing Tests

### Basic Test Structure

```rust
use jstime_core as jstime;

mod common;

#[test]
fn test_something() {
    // 1. Setup
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    
    // 2. Execute
    let result = jstime.run_script("console.log('test')", "test");
    
    // 3. Assert
    assert!(result.is_ok());
}
```

### Test Guidelines

**Do:**
- ✅ Use descriptive test names: `test_fetch_returns_promise`
- ✅ Test one thing per test function
- ✅ Include both success and error cases
- ✅ Use `common::setup()` for V8 initialization
- ✅ Clean up resources (happens automatically via Drop)
- ✅ Add comments for complex test logic

**Don't:**
- ❌ Create tests that depend on external services (use mocking)
- ❌ Write tests that depend on other tests
- ❌ Use hardcoded paths (use relative paths from fixtures)
- ❌ Ignore test failures (fix or document expected failures)

### Conformance Test Patterns

Conformance tests verify spec compliance:

```rust
#[test]
fn spec_compliant_behavior() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    
    // Test exact spec behavior
    let script = r#"
        const obj = { value: 42 };
        // Test according to specification
    "#;
    
    let result = jstime.run_script(script, "test").unwrap();
    assert_eq!(result, "expected per spec");
}
```

### Testing Async Operations

Tests involving timers, fetch, or other async operations:

```rust
#[test]
fn test_async_operation() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    
    let script = r#"
        let completed = false;
        setTimeout(() => { completed = true; }, 10);
        // Event loop will run automatically
    "#;
    
    jstime.run_script(script, "test").unwrap();
    // The event loop ran, so timeout executed
}
```

### Testing Modules

```rust
#[test]
fn test_module_import() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    
    let result = jstime.import("fixtures/modules/simple.js");
    assert!(result.is_ok());
}
```

### Testing Error Cases

```rust
#[test]
fn test_error_thrown() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    
    let result = jstime.run_script("throw new Error('test')", "test");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("test"));
}
```

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Specific Test File
```bash
cargo test --test test_console
```

### Run Specific Test
```bash
cargo test test_console_log
```

### Run with Output
```bash
cargo test -- --nocapture
```

### Run Conformance Tests Only
```bash
cargo test conformance
```

### Run Tests in Release Mode
```bash
cargo test --release
```

## Test Coverage

To check test coverage (requires `cargo-tarpaulin`):

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html
```

## Debugging Tests

### Print Output
```rust
#[test]
fn test_with_output() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    
    let result = jstime.run_script("console.log('debug'); 42", "test");
    println!("Result: {:?}", result);
    
    assert!(result.is_ok());
}
```

Run with: `cargo test test_with_output -- --nocapture`

### Use rust-lldb/rust-gdb
```bash
cargo test --no-run
rust-lldb ./target/debug/deps/test_builtins-<hash>
# Set breakpoints and run
```

## Adding New Tests

When adding a new API:

1. **Basic tests** in `test_builtins.rs`:
   ```rust
   #[test]
   fn test_your_api_exists() {
       let _setup_guard = common::setup();
       let options = jstime::Options::default();
       let mut jstime = jstime::JSTime::new(options);
       
       let result = jstime.run_script("typeof yourAPI", "test");
       assert_eq!(result.unwrap(), "function");
   }
   ```

2. **Feature tests** in new file `test_your_api.rs`:
   ```rust
   // Detailed functionality tests
   ```

3. **Conformance tests** in `test_conformance_your_api.rs`:
   ```rust
   // Standards compliance tests
   ```

4. Update [CONFORMANCE_TESTS.md](./CONFORMANCE_TESTS.md) with details

## Test Fixtures

When adding fixtures:

1. Place in appropriate subdirectory:
   - `fixtures/fs/` - File system test files
   - `fixtures/modules/` - Module test files

2. Keep fixtures minimal and focused

3. Document fixture purpose in test comments

## CI Testing

All tests run in CI on:
- Rust stable, beta, and nightly
- Multiple platforms (Linux, macOS, Windows)

Tests must pass on all configurations to merge.

## Test Performance

Keep tests fast:
- Use `run_script_no_event_loop` when event loop isn't needed
- Minimize external dependencies
- Use small test fixtures
- Consider `#[ignore]` for slow tests

Ignored tests can be run with:
```bash
cargo test -- --ignored
```

## Common Test Patterns

### Testing Multiple Cases

```rust
#[test]
fn test_multiple_inputs() {
    let _setup_guard = common::setup();
    
    let test_cases = vec![
        ("input1", "expected1"),
        ("input2", "expected2"),
    ];
    
    for (input, expected) in test_cases {
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(input, "test").unwrap();
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}
```

### Testing with Fixtures

```rust
#[test]
fn test_with_file() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    
    let script = format!(
        r#"
        import {{ readFile }} from 'node:fs/promises';
        await readFile('{}', 'utf-8');
        "#,
        "core/tests/fixtures/fs/test-readfile.txt"
    );
    
    let result = jstime.run_script(&script, "test");
    assert!(result.is_ok());
}
```

## Resources

- [CONFORMANCE_TESTS.md](./CONFORMANCE_TESTS.md) - Conformance testing details
- [DEVELOPMENT.md](../../DEVELOPMENT.md) - Development guide
- [Rust Testing](https://doc.rust-lang.org/book/ch11-00-testing.html) - Rust testing guide
- [Cargo Test](https://doc.rust-lang.org/cargo/commands/cargo-test.html) - Cargo test documentation

## Getting Help

- Look at existing tests for patterns
- Check test output carefully for error messages
- Run tests with `--nocapture` to see all output
- Ask in GitHub issues or discussions if stuck
