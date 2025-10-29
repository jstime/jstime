# Performance Optimizations

This document describes the performance optimizations implemented in jstime.

## Compiler Optimizations

### Release Profile Configuration
- **LTO (Link-Time Optimization)**: Enabled to allow cross-crate optimizations
- **codegen-units = 1**: Reduces parallelism during compilation but improves runtime performance by allowing better optimizations
- **opt-level = 3**: Maximum optimization level
- **strip = true**: Removes debug symbols to reduce binary size

These settings are configured in the workspace `Cargo.toml` and apply to release builds.

### V8 Flags
The runtime now enables performance-oriented V8 flags:
- `--turbofan`: Ensures the TurboFan optimizing compiler is enabled
- `--opt`: Enables general optimizations

These flags are automatically applied unless overridden by user-provided V8 flags.

## Runtime Optimizations

### Memory Allocation
1. **Pre-allocated Vectors**: Vectors are pre-allocated with `Vec::with_capacity()` where the size is known or can be estimated:
   - Event loop ready timers collection
   - HTTP response headers
   - External references
   - Fetch request headers

2. **String Caching**: Frequently used V8 string keys (like "body", "status", "statusText", "headers") are cached in `IsolateState` to avoid repeated allocation during fetch operations.

### Hash Map Performance
- Replaced `std::collections::HashMap` with `rustc_hash::FxHashMap` in the module map for faster lookups
- FxHashMap uses a faster (but non-cryptographic) hash function suitable for small keys

### Function Inlining
Added `#[inline]` hints to hot path functions in the event loop:
- `add_pending_timers()`
- `clear_marked_timers()`
- `collect_ready_timers()`
- `reschedule_interval()`
- `process_fetches()`

### Early Returns
Optimized event loop operations to check if collections are empty before processing:
- Timers to add
- Timers to clear
- Pending fetches

This avoids unnecessary allocations and iterations when there's no work to do.

### REPL Optimization
- Pre-build rustyline configuration once instead of recreating it in each loop iteration

## Performance Characteristics

### Benchmark Results
On a typical system, the optimized jstime runtime achieves:

- **Arithmetic operations**: ~20ms for 10M iterations
- **String concatenation**: ~0.2ms for 5K characters
- **Array operations**: ~1ms for 10K elements with reduce
- **Object creation**: ~2ms for 10K objects
- **Recursive fibonacci(20)**: ~0.3ms
- **JSON operations**: ~20ms for 1K serialize/parse cycles
- **Timer management**: ~0.6ms for 1K timer create/clear operations

### Binary Size
The optimized release binary is approximately 34MB with LTO and debug symbols stripped.

## Recommendations for Users

### For Production
Use the release build for best performance:
```bash
cargo build --release
./target/release/jstime script.js
```

### For Development
Use the debug build for faster compilation:
```bash
cargo build
./target/debug/jstime script.js
```

### Custom V8 Flags
You can pass custom V8 flags to fine-tune performance:
```bash
jstime --v8-options="--max-old-space-size=4096" script.js
```

### HTTP/Fetch Optimizations
1. **Connection Pooling**: Upgraded to ureq 3.1 and implemented Agent-based connection pooling for reusing HTTP connections across multiple fetch requests
2. **Status Code Handling**: Configured the HTTP agent to not treat HTTP status codes as errors, aligning with the Fetch API specification
3. **Improved Response Handling**: Updated to ureq 3.x API for better performance and reduced memory overhead
4. **Header Vector Pre-allocation**: Pre-allocate headers vector with capacity hint to reduce reallocations

### Module System Optimizations
1. **Path Caching**: Optimized module resolution to use `.cloned()` instead of `.unwrap().to_owned()` for better performance
2. **Fast Hash Maps**: Using FxHashMap for module map lookups with non-cryptographic hashing

## Performance Testing

### JavaScript Benchmarks

Run the JavaScript benchmark suite to measure runtime performance:

```bash
cargo build --release
./target/release/jstime benchmarks/js_benchmarks.js
```

The JavaScript benchmark suite tests:
- Core operations (arithmetic, strings, arrays, objects, functions)
- Built-in APIs (console, JSON, performance, base64, URL, crypto, events)
- Real-world patterns (JSON serialization, event dispatch, recursive algorithms)

Each benchmark includes warmup iterations and reports both total and per-iteration timing.

### Rust Benchmarks

Run comprehensive Rust-level benchmarks using Criterion:

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark suite
cargo bench script_execution
cargo bench json_operations

# Compare against baseline
cargo bench -- --save-baseline main
# Make changes, then:
cargo bench -- --baseline main
```

The Criterion benchmark suite provides:
- Statistical analysis (mean, median, std deviation)
- Outlier detection
- HTML reports with graphs (`target/criterion/report/index.html`)
- Performance regression detection

See `benchmarks/README.md` for detailed benchmark documentation.

## Future Optimization Opportunities

1. **Snapshot Support**: Enable V8 snapshot support to reduce startup time
2. **Parallel Module Loading**: Parallelize module compilation where possible
3. **JIT Warmup**: Consider adding warmup patterns for hot code paths
4. **Memory Pooling**: Implement object pooling for frequently allocated objects
5. **Native Modules**: Add support for native Rust modules for performance-critical operations
6. **SmallVec**: Use SmallVec for small collections to reduce heap allocations
7. **String Interning**: Consider interning frequently used strings beyond current cache
