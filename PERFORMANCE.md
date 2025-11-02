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

### V8 Snapshots

**Implemented**: V8 snapshot support is now enabled to significantly reduce startup time.

The runtime uses V8 snapshots to pre-compile all built-in APIs into a binary blob:
- All built-in JavaScript polyfills (console, timers, fetch, URL, etc.) are compiled once at build time
- The snapshot blob (~1-2MB) is embedded in the jstime binary
- At runtime, the snapshot is loaded instead of re-compiling JavaScript source
- This eliminates the cost of parsing and compiling ~50KB of built-in JavaScript code on every startup

**Performance Impact**:
- Startup time reduced by 30-50% depending on system
- Particularly beneficial for short-lived scripts and CLI tools
- No runtime performance impact - only affects initialization

To benchmark the improvement:
```bash
cargo bench startup
```

See `benchmarks/README.md` for detailed benchmark instructions.

### Memory Allocation
1. **Pre-allocated Vectors**: Vectors are pre-allocated with `Vec::with_capacity()` where the size is known or can be estimated:
   - Event loop ready timers collection
   - HTTP response headers
   - External references
   - Fetch request headers

2. **SmallVec Optimization**: Small, stack-allocated vectors reduce heap allocations for collections that are typically small:
   - **Event Loop Collections**: Timer collections (ready callbacks, ready times, pending additions, cleared timers) use SmallVec with inline capacity of 8, avoiding heap allocations for typical workloads
   - **Fetch Requests**: Pending fetch request collection uses SmallVec[4] to handle common concurrent fetch scenarios without heap allocation
   - **URL Parsing**: Host:port splitting uses SmallVec[2] for zero-allocation parsing in the common case
   - **External References**: Module external references (79 items) use SmallVec to keep the fixed-size collection on stack during initialization
   - **Performance Impact**: Eliminates heap allocations for small collections, reducing memory pressure and improving cache locality in hot paths
   - **Implementation**: Uses `smallvec` crate with appropriate inline capacities based on typical usage patterns

3. **Object Pooling**: A generic object pooling mechanism reduces allocation overhead by reusing objects:
   - **Pool Structure**: `Pool<T>` in `core/src/pool.rs` provides thread-local object recycling
   - **Pooled Types**: Header vectors (`Vec<(String, String)>`) for fetch operations
   - **Per-Isolate Lifecycle**: Pools are stored in `IsolateState` and managed per V8 isolate
   - **Capacity Limits**: Pools have configurable maximum capacity (200 objects) to prevent unbounded growth
   - **Zero-Cost Abstraction**: `PooledVec<T>` provides RAII-style automatic return-to-pool via Drop
   - **Performance Impact**: Reduces allocations in fetch hot paths, particularly beneficial for applications making many HTTP requests
   - **Implementation**: Event loop methods use pooled vectors instead of allocating fresh ones on each fetch operation

4. **Comprehensive String Caching**: A comprehensive string caching mechanism significantly reduces UTF-8 ↔ V8 string conversion overhead.
   - **Cache Structure**: `StringCache` in `IsolateState` caches 40+ frequently used string literals
   - **Lazy Initialization**: Strings are cached on first use (zero overhead for unused strings)
   - **Categories Covered**:
     - Fetch-related: "status", "statusText", "headers"
     - Common properties: "name", "type", "value", "length", "done", "message", "stack"
     - Crypto: "algorithm", "hash", "extractable", "usages", etc.
     - Events: Internal property names like "__listeners__", "__currentTarget__", "__target__", "__stopPropagation__", "__stopImmediatePropagation__", "__defaultPrevented__", and standard property names like "cancelable", "type" (Note: User-provided event type values like "click" are not cached as they vary widely)
     - File system: "isFile", "isDirectory", "size", etc.
     - Modules: "url" for import.meta
   - **Performance Impact**: Eliminates repeated string allocations in hot paths like error formatting, fetch operations, event dispatching, and module loading
   - **Implementation**: Uses V8's `Global<String>` handles with helper macro `get_or_create_cached_string!`
   - **Event API Optimization**: String caching in the event implementation delivers 1.7x performance improvement (from 141ms to 81ms for 100K event dispatches)

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
- **Crypto UUID generation**: ~5.8ms for 10K iterations (~577 ns/op)
- **Crypto getRandomValues (16 bytes)**: ~5.4ms for 10K iterations (~535 ns/op, 84% of theoretical max)
- **Event dispatch**: ~81ms for 100K dispatches (1.7x faster after string caching optimization)

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

## HTTP/Fetch Optimizations
1. **Connection Pooling**: Upgraded to ureq 3.1 and implemented Agent-based connection pooling for reusing HTTP connections across multiple fetch requests
2. **Status Code Handling**: Configured the HTTP agent to not treat HTTP status codes as errors, aligning with the Fetch API specification
3. **Improved Response Handling**: Updated to ureq 3.x API for better performance and reduced memory overhead
4. **Header Vector Pre-allocation**: Pre-allocate headers vector with capacity hint to reduce reallocations

### Crypto Optimizations
1. **Fast UUID Formatting**: Optimized `crypto.randomUUID()` with unrolled loop and manual hex formatting
   - Pre-allocates 36-byte buffer to avoid allocations
   - Uses lookup table for hex digit conversion
   - Eliminates branch predictions with fully unrolled loop
   - Result: **12.8% faster** (577 ns/op vs 650 ns/op baseline)
2. **Optimized getRandomValues**: Micro-optimizations for small arrays
   - Fixed byte_offset bug to correctly handle typed arrays with offsets
   - Eliminated duplicate byte_length() V8 API calls
   - Added early return for zero-length arrays
   - Performance at 84-99% of theoretical maximum (limited by CSRNG speed)
3. **Inlined Hot Paths**: Added `#[inline]` to frequently called crypto functions
   - `crypto_get_random_values`
   - `crypto_random_uuid`
   - `crypto_subtle_digest`
4. **Performance Characteristics**:
   - Small arrays (16 bytes): ~1867 ops/ms (84% of theoretical max, 535 ns/op)
   - Medium arrays (1KB): ~317 ops/ms (96% of theoretical max, 3.2 µs/op)
   - Large arrays (64KB): ~5.86 ops/ms (99% of theoretical max, 170 µs/op)
   - The "slow" performance for large arrays is actually optimal - it's limited by the cryptographically secure random number generator itself (~2.6 GB/s throughput)

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

## JIT Warmup

**Implemented**: JIT warmup support allows V8's TurboFan compiler to optimize code before actual execution.

The runtime now supports optional warmup iterations:
- Controlled via the `--warmup` CLI flag or `Options::with_warmup()` API
- Executes the script/module multiple times before the actual run
- Allows TurboFan JIT compiler to profile and optimize hot code paths
- Particularly useful for benchmarking and performance-critical scripts

**Usage Examples**:

Command line:
```bash
# Run with 10 warmup iterations
jstime --warmup 10 script.js

# Or use = syntax
jstime --warmup=10 script.js
```

API usage:
```rust
use jstime_core as jstime;

let options = jstime::Options::default()
    .with_warmup(10);
let mut js = jstime::JSTime::new(options);
js.run_script("/* your code */", "script.js")?;
```

**Performance Impact**:
- Warmup incurs upfront cost but can significantly improve execution time for compute-intensive code
- Best used for benchmarking, repeated executions, or performance-critical scripts
- Default is 0 (no warmup) to optimize for startup time

**Recommendations**:
- Use 5-10 iterations for benchmarking
- Use 0 (default) for one-time script execution
- Adjust based on script complexity and execution patterns

## Parallel Module Loading

**Implemented**: Parallel module loading optimizes the module import process by parallelizing file I/O operations.

The runtime now implements intelligent parallel prefetching of module dependencies:
- **Import Discovery**: Parses JavaScript source to discover import/export statements
- **Parallel File I/O**: Reads discovered module files in parallel using threads
- **Smart Caching**: Files are cached in the global SOURCE_CACHE before V8 compilation begins
- **Breadth-First Traversal**: Efficiently discovers and loads the entire module dependency graph
- **Deduplication**: Shared dependencies are loaded only once

**Implementation Details**:
1. When a module is imported, the system first scans its source code to extract all import specifiers
2. All discovered modules are read from disk in parallel using threads
3. The process continues recursively, building and caching the entire dependency graph
4. V8 module compilation, instantiation, and evaluation remain sequential (as required)
5. When V8's module resolution callback is invoked, files are already cached, eliminating I/O bottlenecks

**Performance Impact**:
- Reduces module loading time for projects with many dependencies
- Particularly beneficial for applications with deep or wide module dependency graphs
- No overhead for single-module scripts
- Maintains correctness and determinism of module evaluation order

**Example**:
For a module graph with 10 independent modules, parallel loading can reduce total I/O time from 10× sequential reads to approximately 1× parallel read batch (assuming sufficient parallelism).

## Future Optimization Opportunities

1. **Native Modules**: Add support for native Rust modules for performance-critical operations
2. **Extended String Caching**: Further expand string caching to additional builtins (text encoding, streams, etc.) as usage patterns emerge
3. **Additional Pooling Opportunities**: As profiling identifies new bottlenecks, expand pooling to other frequently allocated types
