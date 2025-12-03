# jstime Performance Benchmarks

This directory contains performance benchmarks for jstime.

## Cross-Runtime Comparison Tests

**New!** Compare jstime with other JavaScript runtimes (Node.js, Deno, Bun).

### Quick Start

```bash
# Run all tests
./benchmarks/cross-runtime/run-tests.sh

# Run specific APIs only
./benchmarks/cross-runtime/run-tests.sh --api crypto,url,json

# Detailed output with sub-test breakdown
./benchmarks/cross-runtime/run-tests.sh --verbose
```

This test suite:
- **Auto-detects** available runtimes (jstime, node, deno, bun)
- **Selective testing**: Run specific APIs with `--api` flag
- **Compliance tests**: Verifies API compatibility across runtimes
- **Performance benchmarks**: Compares execution speed
- **Graceful handling**: Works with whatever runtimes are installed

See [cross-runtime/README.md](./cross-runtime/README.md) for detailed documentation.

### Sample Output

```
=== Cross-Runtime Test Suite ===
Available runtimes: jstime node

Compliance Test Results:
  jstime    : 14/14 passed ✓
  node      : 14/14 passed ✓

Performance Comparison:
  arithmetic:          jstime:0.979ms★ node:2.562ms
  json:                jstime:53.275ms★ node:89.902ms
  ...
```

## JavaScript Benchmarks

### Running JS Benchmarks

```bash
# From repository root
cargo build --release
./target/release/jstime benchmarks/js_benchmarks.js
```

The JavaScript benchmark suite (`js_benchmarks.js`) tests:
- Arithmetic operations
- String operations (concatenation, templates)
- Array operations (creation, map, filter, reduce)
- Object operations (creation, property access, spread)
- Function calls (regular and arrow functions, recursion)
- JSON operations (stringify, parse, round-trip)
- Console API
- Performance API (`performance.now()`)
- Base64 encoding/decoding (`btoa`, `atob`)
- URL parsing and URLSearchParams
- Crypto operations (randomUUID, getRandomValues)
- Event system (Event, EventTarget, dispatch)

Each benchmark includes:
- Warmup iterations to allow JIT optimization
- Multiple iterations for statistical significance
- Total time and average time per iteration

## Rust Benchmarks

### Running Rust Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark group
cargo bench startup              # Startup and initialization benchmarks
cargo bench script_execution
cargo bench json_operations
cargo bench performance_api

# Generate detailed report
cargo bench -- --verbose
```

The Rust benchmark suite uses [Criterion.rs](https://github.com/bheisler/criterion.rs) and tests:
- **Startup** - JSTime instance creation with and without snapshots
- Script execution (arithmetic, strings, arrays, objects, functions)
- Console API
- JSON operations
- Performance API
- Base64 operations
- URL operations
- Crypto operations
- Event operations

### Startup Benchmarks

The startup benchmarks measure the performance impact of V8 snapshots:

- `new_instance_with_snapshot` - Creating a JSTime instance with V8 snapshot (default)
- `new_instance_without_snapshot` - Creating a JSTime instance without snapshot (for comparison)
- `new_instance_and_hello_world` - Cold start with minimal script execution
- `new_instance_with_builtins` - Cold start with builtin API usage

V8 snapshots significantly improve startup time by pre-compiling all built-in APIs (console, fetch, URL, crypto, etc.) into a binary blob that's loaded at initialization time instead of being compiled from JavaScript source.

### Benchmark Results Location

Criterion saves detailed benchmark results in:
- `target/criterion/` - HTML reports and raw data
- Open `target/criterion/report/index.html` in a browser for visualized results

## Performance Comparisons

To compare performance before and after changes:

1. Run benchmarks on the baseline:
   ```bash
   git checkout main
   cargo bench --bench runtime_bench -- --save-baseline main
   ```

2. Make your changes and run benchmarks again:
   ```bash
   git checkout feature-branch
   cargo bench --bench runtime_bench -- --baseline main
   ```

Criterion will show performance differences between the two runs.

### Comparing Snapshot Performance

To measure the impact of snapshots:

1. Create a baseline with snapshots (current implementation):
   ```bash
   cargo bench startup -- --save-baseline with-snapshots
   ```

2. To test without snapshots, temporarily modify `cli/main.rs` to use `Options::new(None)`:
   ```bash
   # Modify code to disable snapshots
   cargo bench startup -- --baseline with-snapshots
   ```

This will show the performance improvement from V8 snapshots.

## Tips for Accurate Benchmarking

1. **Use release builds**: Always benchmark with `--release`
2. **Minimize system load**: Close other applications during benchmarking
3. **Multiple runs**: Run benchmarks multiple times and look for consistency
4. **Warm up the system**: The first run may be slower due to cold caches
5. **Isolate changes**: Test one optimization at a time to measure its impact

## Adding New Benchmarks

### Adding JS Benchmarks

Edit `js_benchmarks.js` and add a new benchmark call:

```javascript
benchmark('My New Test', () => {
  // Your test code here
  return result;
}, iterations);
```

### Adding Rust Benchmarks

Edit `core/benches/runtime_bench.rs` and add a new benchmark function:

```rust
fn bench_my_feature(c: &mut Criterion) {
    let mut group = c.benchmark_group("my_feature");
    
    group.bench_function("test_name", |b| {
        b.iter_batched(
            || setup(),
            |mut js| {
                js.run_script(black_box("/* your js code */"), "bench.js")
            },
            criterion::BatchSize::SmallInput,
        )
    });
    
    group.finish();
}

// Add to criterion_group! at the bottom:
criterion_group!(
    benches,
    bench_startup,
    bench_script_execution,
    // ... other benchmarks ...
    bench_my_feature  // Add your new function here
);
```

## Continuous Integration

Benchmarks can be integrated into CI to detect performance regressions:

1. Run benchmarks on every PR
2. Compare against main branch baseline
3. Fail if performance degrades beyond threshold (e.g., >10% slower)

See `.github/workflows/` for CI configuration examples.
