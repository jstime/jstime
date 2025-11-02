# Cross-Runtime Compliance and Performance Tests

This directory contains compliance and performance tests that compare jstime with other JavaScript runtimes (Node.js, Deno, and Bun).

## Overview

The test suite is divided into two categories:

1. **Compliance Tests** - Verify that APIs work correctly and are compatible across runtimes
2. **Performance Tests** - Benchmark execution speed for various operations

## Running the Tests

### Quick Start

```bash
# From the repository root
./benchmarks/cross-runtime/run-tests.sh

# For detailed performance breakdown
./benchmarks/cross-runtime/run-tests.sh --verbose
```

The test runner will:
- Automatically detect available runtimes (jstime, node, deno, bun)
- Build jstime if needed
- Run all compliance and performance tests
- Generate a comparison report

### Command Line Options

- `--verbose` or `-v`: Show detailed breakdown for each performance test, including:
  - Individual elapsed time for each runtime
  - Number of iterations
  - Operations per millisecond
- `--help` or `-h`: Show usage information

### Requirements

- **Bash 3.2+**: Required to run the test script (compatible with the default bash on macOS)
- **jstime**: Built automatically if not found at `target/release/jstime`
- **Node.js**: Optional, will be used if available
- **Deno**: Optional, will be used if available
- **Bun**: Optional, will be used if available

The tests will run with whatever runtimes are available on your system.

## Test Structure

### Compliance Tests (`compliance/`)

Tests verify that standard JavaScript APIs work correctly:

- **test-console.js** - Console API (console.log, console.error, etc.)
- **test-timers.js** - Timers API (setTimeout, setInterval, clearTimeout, clearInterval)
- **test-url.js** - URL API (URL constructor, URLSearchParams)
- **test-crypto.js** - Crypto API (crypto.getRandomValues, crypto.randomUUID)
- **test-performance.js** - Performance API (performance.now, performance.timeOrigin)
- **test-base64.js** - Base64 encoding (btoa, atob)
- **test-json.js** - JSON operations (JSON.parse, JSON.stringify)
- **test-text-encoding.js** - Text Encoding API (TextEncoder, TextDecoder)
- **test-event.js** - Event and EventTarget API (Event, EventTarget, event dispatching)
- **test-streams.js** - Streams API (ReadableStream, WritableStream, TransformStream)
- **test-structured-clone.js** - Structured Clone API (structuredClone for deep cloning)
- **test-microtask.js** - Microtask API (queueMicrotask)

Each test outputs: `API_NAME: X passed, Y failed`

### Performance Tests (`performance/`)

Benchmarks measure execution speed for common operations:

- **bench-arithmetic.js** - Basic arithmetic operations (100K iterations)
- **bench-strings.js** - String concatenation (10K iterations)
- **bench-arrays.js** - Array map operations (1K iterations, 1K elements)
- **bench-objects.js** - Object creation and property access (100K iterations)
- **bench-json.js** - JSON stringify/parse round-trip (100K iterations)
- **bench-base64.js** - Base64 encode/decode round-trip (100K iterations)
- **bench-url.js** - URL parsing (100K iterations)
- **bench-crypto.js** - UUID generation (10K iterations)
- **bench-text-encoding.js** - Text encoding/decoding round-trip (10K iterations)
- **bench-structured-clone.js** - Structured cloning of complex objects (10K iterations)
- **bench-event.js** - Event creation and dispatching (100K iterations)

Each benchmark outputs JSON with timing results.

## Running Individual Tests

You can run individual tests manually with any runtime:

```bash
# Compliance test
./target/release/jstime benchmarks/cross-runtime/compliance/test-console.js
node benchmarks/cross-runtime/compliance/test-console.js
deno run benchmarks/cross-runtime/compliance/test-console.js
bun run benchmarks/cross-runtime/compliance/test-console.js

# Performance test
./target/release/jstime benchmarks/cross-runtime/performance/bench-arithmetic.js
node benchmarks/cross-runtime/performance/bench-arithmetic.js
deno run benchmarks/cross-runtime/performance/bench-arithmetic.js
bun run benchmarks/cross-runtime/performance/bench-arithmetic.js
```

## Sample Output

```
=== Cross-Runtime Test Suite ===
Available runtimes: jstime node deno

jstime
  Path:    /path/to/jstime/target/release/jstime
  Version: jstime 0.63.0
node
  Path:    /usr/local/bin/node
  Version: v20.19.5
deno
  Path:    /usr/local/bin/deno
  Version: 1.40.0

=== Compliance Tests ===

Running test-console...
  jstime    : 5 passed ✓
  node      : 5 passed ✓
  deno      : 5 passed ✓

...

=== Performance Tests ===

Running bench-arithmetic...
  jstime    : 45.123ms (2216.32 ops/ms)
  node      : 38.456ms (2600.42 ops/ms)
  deno      : 42.789ms (2337.29 ops/ms)

...

=== Summary ===

Compliance Test Results:
  jstime    : 12/12 passed ✓
  node      : 12/12 passed ✓
  deno      : 12/12 passed ✓

Performance Comparison:
  (Lower is better - showing elapsed time in milliseconds)

  arithmetic:          jstime:45.123ms node:38.456ms★ deno:42.789ms
  strings:             jstime:52.341ms★ node:55.678ms deno:53.912ms
  ...
```

### Sample Output (Verbose Mode)

With `--verbose` flag, the output includes detailed breakdown for each performance test:

```
=== Performance Tests ===

Running bench-arithmetic...
  jstime    : 0.944ms
    Iterations: 100000, Ops/ms: 105976.71
  node      : 2.193ms
    Iterations: 100000, Ops/ms: 45605.42

...

=== Summary ===

Performance Comparison:
  (Lower time is better - showing detailed breakdown)

  arithmetic:          jstime:0.944ms★ node:2.193ms
    jstime:   elapsed=0.944ms, iterations=100000, ops_per_ms=105976.71
    node:     elapsed=2.193ms, iterations=100000, ops_per_ms=45605.42

  strings:             jstime:6.758ms★ node:8.635ms
    jstime:   elapsed=6.758ms, iterations=10000, ops_per_ms=1479.77
    node:     elapsed=8.635ms, iterations=10000, ops_per_ms=1158.08
  ...
```

## Adding New Tests

### Adding a Compliance Test

1. Create a new file in `compliance/` directory (e.g., `test-myapi.js`)
2. Use this template:

```javascript
let passed = 0;
let failed = 0;

function test(name, fn) {
  try {
    fn();
    passed++;
  } catch (e) {
    console.error(`FAIL: ${name} - ${e.message}`);
    failed++;
  }
}

test('my API exists', () => {
  if (typeof myAPI !== 'object') throw new Error('myAPI not found');
});

// Add more tests...

console.log(`My API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
```

3. Add the test filename to the `COMPLIANCE_TESTS` array in `run-tests.sh`

### Adding a Performance Test

1. Create a new file in `performance/` directory (e.g., `bench-mytest.js`)
2. Use this template:

```javascript
const ITERATIONS = 10000;

const start = performance.now();

for (let i = 0; i < ITERATIONS; i++) {
  // Your test code here
}

const end = performance.now();
const elapsed = end - start;

console.log(JSON.stringify({
  test: 'mytest',
  iterations: ITERATIONS,
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
}));
```

3. Add the test filename to the `PERFORMANCE_TESTS` array in `run-tests.sh`

## Interpreting Results

### Compliance Tests

- **Passed**: All API features work correctly
- **Failed**: One or more API features don't work or behave incorrectly
- **Error**: Test couldn't run (e.g., API not implemented, syntax error)

### Performance Tests

- **Lower times are better** (faster execution)
- The fastest runtime for each test is marked with ★
- Results can vary based on:
  - System load
  - Hardware specifications
  - Runtime optimizations (JIT warmup)
  - Build configuration (debug vs release)

**Note**: Always use release builds for meaningful performance comparisons:
```bash
cargo build --release
```

## Troubleshooting

### "This script requires bash to run"

The test runner script requires Bash 3.2 or higher (which is available on all modern systems, including macOS). If you get this error:
```bash
# Make sure to run with bash explicitly
bash benchmarks/cross-runtime/run-tests.sh

# Or make the script executable and run directly
chmod +x benchmarks/cross-runtime/run-tests.sh
./benchmarks/cross-runtime/run-tests.sh
```

The script is compatible with Bash 3.2+, so it works with the default bash on macOS without needing to install a newer version.

### "No JavaScript runtimes found"

Install at least one runtime:
- Node.js: https://nodejs.org/
- Deno: https://deno.land/
- Bun: https://bun.sh/

Or build jstime:
```bash
cargo build --release
```

### Test timeouts

Tests have a 30-second timeout. If a test times out:
1. Check system resources
2. Reduce iteration counts in performance tests
3. Check for infinite loops in test code

### Permission errors (Deno)

Some Deno tests may need additional permissions:
```bash
deno run --allow-net --allow-read test-file.js
```

The test runner includes `--allow-net` for tests that might need it.

## CI Integration

You can integrate these tests into your CI pipeline:

```yaml
# Example GitHub Actions workflow
- name: Run cross-runtime tests
  run: |
    cargo build --release
    bash ./benchmarks/cross-runtime/run-tests.sh
```

The test runner exits with code 0 on success and non-zero on failure.

## Notes

- Tests are designed to be runtime-agnostic and use only standard APIs
- Performance results are relative and depend on many factors
- Some APIs may not be available in all runtimes (tests will fail gracefully)
- Results are most meaningful when comparing the same test across runtimes on the same system
