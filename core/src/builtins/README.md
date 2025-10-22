# Built-in APIs

This directory contains the implementation of JavaScript APIs provided by jstime. Each API consists of two files that work together:

- **`*_impl.rs`** - Rust implementation with V8 bindings
- **`*.js`** - JavaScript polyfill/wrapper for spec-compliant behavior

## Architecture Pattern

Each built-in API follows a consistent pattern:

### Rust Implementation (`*_impl.rs`)

The Rust file provides:

1. **External References**: Functions that can be called from JavaScript
2. **Bindings Registration**: Registers Rust functions with V8
3. **Implementation**: The actual functionality

```rust
// Example structure
pub fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: my_function.map_fn_to(),
        },
    ]
}

pub fn register_bindings(scope: &mut v8::HandleScope, bindings: v8::Local<v8::Object>) {
    let key = v8::String::new(scope, "myFunction").unwrap();
    let func = v8::Function::new(scope, my_function).unwrap();
    bindings.set(scope, key.into(), func.into());
}

fn my_function(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    // Implementation
}
```

### JavaScript Polyfill (`*.js`)

The JavaScript file:

1. Wraps Rust functions to provide spec-compliant APIs
2. Handles argument processing and validation
3. Implements any JavaScript-side logic needed for standards compliance

```javascript
// Example structure
(function(bindings) {
  globalThis.myAPI = function(...args) {
    // Process arguments
    // Call Rust implementation
    return bindings.myFunction(...args);
  };
})(globalThis.__jstime_bindings__);
```

## Available APIs

### Core APIs

| API | Files | Spec | Description |
|-----|-------|------|-------------|
| **Console** | `console_impl.rs`, `console.js` | [WHATWG Console](https://console.spec.whatwg.org/) | Logging and debugging |
| **Timers** | `timers_impl.rs`, `timers.js` | [HTML Timers](https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#timers) | setTimeout, setInterval |
| **Performance** | `performance_impl.rs`, `performance.js` | [HR Time](https://w3c.github.io/hr-time/) | High-resolution timing |
| **Microtask** | `queue_microtask_impl.rs`, `queue_microtask.js` | [HTML Microtask](https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#microtask-queuing) | queueMicrotask |

### Web APIs

| API | Files | Spec | Description |
|-----|-------|------|-------------|
| **Fetch** | `fetch_impl.rs`, `fetch.js` | [WHATWG Fetch](https://fetch.spec.whatwg.org/) | HTTP client |
| **URL** | `url_impl.rs`, `url.js` | [WHATWG URL](https://url.spec.whatwg.org/) | URL parsing and manipulation |
| **Events** | `event_impl.rs`, `event.js` | [DOM Events](https://dom.spec.whatwg.org/#events) | Event and EventTarget |

### Data APIs

| API | Files | Spec | Description |
|-----|-------|------|-------------|
| **Base64** | `base64_impl.rs`, `base64.js` | [HTML Base64](https://html.spec.whatwg.org/multipage/webappapis.html#atob) | atob, btoa |
| **Structured Clone** | `structured_clone_impl.rs`, `structured_clone.js` | [HTML Structured Clone](https://html.spec.whatwg.org/multipage/structured-data.html#structured-cloning) | Deep object cloning |

### System APIs

| API | Files | Spec | Description |
|-----|-------|------|-------------|
| **File System** | `fs_impl.rs`, `fs.js` | [Node.js fs/promises](https://nodejs.org/api/fs.html#promises-api) | File system operations |

## Adding a New API

See [DEVELOPMENT.md](../../../DEVELOPMENT.md#adding-new-built-in-apis) for step-by-step instructions.

### Quick Checklist

1. ✅ Create `your_api_impl.rs` with Rust implementation
2. ✅ Create `your_api.js` with JavaScript polyfill
3. ✅ Add to `mod.rs`:
   - Import module
   - Add to `get_external_references()`
   - Add to `Builtins::create()`
4. ✅ Write tests in `core/tests/test_your_api.rs`
5. ✅ Add conformance tests in `core/tests/test_conformance_your_api.rs`
6. ✅ Update `docs/FEATURES.md` with API documentation
7. ✅ Create example in `examples/your-api-demo.js`
8. ✅ Ensure all tests pass: `cargo test`
9. ✅ Format code: `cargo fmt --all`
10. ✅ Check lints: `cargo clippy -- -D warnings`

## Implementation Guidelines

### Rust Side

**Do:**
- ✅ Handle errors gracefully
- ✅ Convert V8 values carefully
- ✅ Follow Rust naming conventions
- ✅ Add doc comments for public functions
- ✅ Use descriptive error messages

**Don't:**
- ❌ Panic in library code (use Result instead)
- ❌ Hold V8 handles across function boundaries
- ❌ Forget to add external references
- ❌ Make unnecessary allocations

### JavaScript Side

**Do:**
- ✅ Follow the relevant specification
- ✅ Validate arguments according to spec
- ✅ Handle edge cases
- ✅ Throw appropriate error types
- ✅ Use modern JavaScript syntax

**Don't:**
- ❌ Add unnecessary polyfill code
- ❌ Diverge from standards
- ❌ Use global state unnecessarily
- ❌ Over-engineer simple APIs

## Testing

Each API should have:

1. **Basic Tests** (`test_builtins.rs`)
   - Verify API exists
   - Test basic functionality
   - Check error cases

2. **Conformance Tests** (`test_conformance_*.rs`)
   - Spec compliance verification
   - Edge cases from specification
   - Standards-mandated behavior

3. **Examples** (`examples/*.js`)
   - Real-world usage examples
   - Document common patterns

## Performance Considerations

- **Minimize string conversions** between Rust and V8
- **Avoid unnecessary allocations** in hot paths
- **Use V8's built-in types** when possible
- **Cache expensive computations** when safe
- **Profile** before optimizing

## Dependencies

Built-in APIs can use these crates:

- **v8**: V8 JavaScript engine bindings (required)
- **lazy_static**: Lazy static initialization
- **url**: URL parsing (for URL API)
- **ureq**: HTTP client (for Fetch API)
- **urlencoding**: URL encoding utilities
- **rustc-hash**: Fast hashing
- **filetime**: File time operations (for FS API)

Add new dependencies sparingly and justify their inclusion.

## Module Registration (`mod.rs`)

The `mod.rs` file coordinates all built-in APIs:

```rust
// 1. Import modules
mod base64_impl;
mod console_impl;
// ...

// 2. Collect external references
pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    let mut refs = Vec::with_capacity(45);
    refs.extend(base64_impl::get_external_references());
    refs.extend(console_impl::get_external_references());
    // ...
    refs
}

// 3. Initialize built-ins
impl Builtins {
    pub(crate) fn create(scope: &mut v8::PinScope) {
        let bindings = v8::Object::new(scope);

        // Register Rust bindings
        base64_impl::register_bindings(scope, bindings);
        console_impl::register_bindings(scope, bindings);
        // ...

        // Load JavaScript polyfills
        builtin!("./base64.js");
        builtin!("./console.js");
        // ...
    }
}
```

## File Organization

Files in this directory:

```
builtins/
├── mod.rs                      # Module coordination
│
├── base64.js                   # JavaScript polyfills
├── console.js
├── event.js
├── fetch.js
├── fs.js
├── performance.js
├── queue_microtask.js
├── structured_clone.js
├── timers.js
├── url.js
│
├── base64_impl.rs             # Rust implementations
├── console_impl.rs
├── event_impl.rs
├── fetch_impl.rs
├── fs_impl.rs
├── performance_impl.rs
├── queue_microtask_impl.rs
├── structured_clone_impl.rs
├── timers_impl.rs
└── url_impl.rs
```

Each pair of files (`*.js` and `*_impl.rs`) implements a complete API.

## Resources

- [DEVELOPMENT.md](../../../DEVELOPMENT.md) - Development guide
- [ARCHITECTURE.md](../../../ARCHITECTURE.md) - Architecture overview
- [docs/FEATURES.md](../../../docs/FEATURES.md) - User-facing API documentation
- [V8 Embedder's Guide](https://v8.dev/docs/embed) - V8 embedding documentation
- [Rust V8 Bindings](https://docs.rs/v8/) - rust-v8 API documentation

## Getting Help

- Check existing implementations for patterns
- Read the specification for the API you're implementing
- Ask questions in GitHub issues or discussions
- Tag maintainers in pull requests for guidance
