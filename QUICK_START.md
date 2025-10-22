# Quick Start Guide

Get up and running with jstime development in 5 minutes! ⚡

## Prerequisites

- **Rust** (stable): Install from [rustup.rs](https://rustup.rs/)
- **Git**: For cloning the repository

That's it! V8 and other dependencies are managed by Cargo.

## Setup (3 steps)

```bash
# 1. Clone the repository
git clone https://github.com/jstime/jstime.git
cd jstime

# 2. Build the project
cargo build

# 3. Run tests to verify everything works
cargo test
```

If all tests pass, you're ready to go! 🎉

## Try It Out

### Run the REPL

```bash
cargo run
```

You'll see:
```
Welcome to jstime!

>>
```

Try some JavaScript:
```javascript
>> console.log("Hello, jstime!")
Hello, jstime!
undefined

>> 2 + 2
4

>> setTimeout(() => console.log("Async works!"), 100)
1
>> Async works!
```

Press `Ctrl+D` or `Ctrl+C` to exit.

### Run an Example

```bash
cargo run -- examples/console-demo.js
```

### Execute a Script

Create `test.js`:
```javascript
console.log("Hello from a file!");
```

Run it:
```bash
cargo run -- test.js
```

## Make Your First Change

Let's add a simple test to verify your setup:

### 1. Open a test file

Edit `core/tests/test_builtins.rs`

### 2. Add a test at the end

```rust
#[test]
fn test_my_first_contribution() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    
    let result = jstime.run_script("'Hello, contributor!'", "test");
    assert_eq!(result.unwrap(), "Hello, contributor!");
}
```

### 3. Run your test

```bash
cargo test test_my_first_contribution
```

You should see:
```
running 1 test
test tests::test_my_first_contribution ... ok
```

Success! You've made your first contribution to jstime! 🚀

## Development Workflow

```bash
# Make changes to code...

# Format your code
cargo fmt --all

# Check for issues
cargo clippy -- -D warnings

# Run tests
cargo test

# Run a specific test
cargo test test_console

# Run the REPL to test interactively
cargo run
```

## Common Commands

| Command | Purpose |
|---------|---------|
| `cargo build` | Build the project |
| `cargo test` | Run all tests |
| `cargo test test_name` | Run specific test |
| `cargo run` | Start the REPL |
| `cargo run -- file.js` | Execute a JavaScript file |
| `cargo fmt --all` | Format code |
| `cargo clippy -- -D warnings` | Check for issues |
| `cargo clean` | Clean build artifacts |

## Project Structure

Quick overview of key directories:

```
jstime/
├── cli/              # Command-line tool and REPL
│   └── main.rs      # CLI entry point
├── core/            # Core runtime library
│   ├── src/
│   │   ├── builtins/  # JavaScript APIs (console, fetch, etc.)
│   │   └── lib.rs     # Core API
│   └── tests/       # Integration tests
├── examples/        # Example scripts
└── docs/           # Documentation
```

## What to Work On

### For Beginners

Good first issues are tagged with `good first issue` on GitHub. Start with:

1. **Documentation**: Improve examples or fix typos
2. **Tests**: Add more test cases for existing features
3. **Examples**: Create new example scripts

### Intermediate

1. **Bug Fixes**: Fix reported issues
2. **API Improvements**: Enhance existing APIs
3. **Test Coverage**: Add conformance tests

### Advanced

1. **New APIs**: Add new built-in APIs
2. **Performance**: Optimize hot paths
3. **Architecture**: Improve core components

## Understanding the Codebase

### Adding a New Built-in API

Follow this pattern (detailed in [DEVELOPMENT.md](./DEVELOPMENT.md)):

1. Create `core/src/builtins/your_api_impl.rs` (Rust)
2. Create `core/src/builtins/your_api.js` (JavaScript)
3. Register in `core/src/builtins/mod.rs`
4. Write tests in `core/tests/test_your_api.rs`
5. Add examples in `examples/your-api-demo.js`
6. Update documentation

### Modifying Existing Code

1. Find the relevant file (check the structure above)
2. Make your changes
3. Run tests: `cargo test`
4. Format: `cargo fmt --all`
5. Lint: `cargo clippy -- -D warnings`

## Testing Your Changes

### Run All Tests

```bash
cargo test
```

Expected output (example):
```
running 307 tests
...
test result: ok. 307 passed; 0 failed; 0 ignored; 0 measured
```

### Run Specific Tests

```bash
# Test a specific file
cargo test --test test_console

# Test a specific function
cargo test test_console_log

# Show output
cargo test -- --nocapture
```

### Interactive Testing

Use the REPL to test changes:

```bash
cargo run

>> // Test your new feature here
```

## Getting Help

Stuck? Here's where to look:

1. **[DEVELOPMENT.md](./DEVELOPMENT.md)** - Detailed development guide
2. **[ARCHITECTURE.md](./ARCHITECTURE.md)** - How jstime works
3. **[core/src/builtins/README.md](./core/src/builtins/README.md)** - Adding APIs
4. **[core/tests/README.md](./core/tests/README.md)** - Writing tests
5. **[GitHub Discussions](https://github.com/jstime/jstime/discussions)** - Ask questions
6. **[GitHub Issues](https://github.com/jstime/jstime/issues)** - Report bugs

## Example: Adding a Simple Function

Let's walk through adding a simple `uppercase()` function to the global scope:

### Step 1: Add Rust Implementation

Edit `core/src/builtins/console_impl.rs` (we'll add it here for simplicity):

```rust
// Add this function
fn uppercase(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if let Some(arg) = args.get(0).to_string(scope) {
        let isolate: &v8::Isolate = scope;
        let text = arg.to_rust_string_lossy(isolate);
        let upper = text.to_uppercase();
        let result = v8::String::new(scope, &upper).unwrap();
        retval.set(result.into());
    }
}
```

### Step 2: Register the Function

In the same file, update `get_external_references()`:

```rust
pub fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: uppercase.map_fn_to(),
        },
    ]
}
```

Update `register_bindings()`:

```rust
pub fn register_bindings(scope: &mut v8::HandleScope, bindings: v8::Local<v8::Object>) {
    let key = v8::String::new(scope, "uppercase").unwrap();
    let func = v8::Function::new(scope, uppercase).unwrap();
    bindings.set(scope, key.into(), func.into());
}
```

### Step 3: Expose to JavaScript

Create `core/src/builtins/uppercase.js`:

```javascript
(function(bindings) {
  globalThis.uppercase = function(str) {
    return bindings.uppercase(str);
  };
})(globalThis.__jstime_bindings__);
```

### Step 4: Add to mod.rs

Edit `core/src/builtins/mod.rs`, add near the end of `Builtins::create()`:

```rust
builtin!("./uppercase.js");
```

### Step 5: Test It

```bash
cargo run

>> uppercase("hello")
HELLO
```

### Step 6: Add Tests

Edit `core/tests/test_builtins.rs`:

```rust
#[test]
fn test_uppercase() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    
    let result = jstime.run_script("uppercase('hello')", "test").unwrap();
    assert_eq!(result, "HELLO");
}
```

Run test:
```bash
cargo test test_uppercase
```

Congratulations! You've added a new feature! 🎊

## Next Steps

Now that you're set up:

1. **Explore the examples**: `ls examples/`
2. **Read the documentation**: Start with [DEVELOPMENT.md](./DEVELOPMENT.md)
3. **Look at existing code**: See how built-ins are implemented
4. **Pick an issue**: Check [GitHub Issues](https://github.com/jstime/jstime/issues)
5. **Ask questions**: Use [GitHub Discussions](https://github.com/jstime/jstime/discussions)

## Tips for Success

✅ **Do:**
- Start small - small PRs get merged faster
- Ask questions - we're here to help
- Read existing code - learn from examples
- Test thoroughly - write tests for your changes
- Follow conventions - match the existing code style

❌ **Don't:**
- Make large, unfocused changes
- Skip writing tests
- Forget to run `cargo fmt` and `cargo clippy`
- Be discouraged - everyone starts somewhere!

## Resources

- **[DEVELOPMENT.md](./DEVELOPMENT.md)** - Full development guide
- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - How jstime works
- **[CONTRIBUTING.md](./CONTRIBUTING.md)** - Contribution guidelines
- **[docs/FEATURES.md](./docs/FEATURES.md)** - API documentation
- **[examples/](./examples/)** - Example scripts

## Welcome!

Thank you for contributing to jstime! Your work helps make JavaScript runtimes more accessible and fun. If you have questions or need help, don't hesitate to ask in GitHub Discussions or Issues.

Happy coding! 🚀

---

**Need more detail?** Check out [DEVELOPMENT.md](./DEVELOPMENT.md) for comprehensive information.
