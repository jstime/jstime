# Cross-Runtime Compliance and Performance Tests

This directory contains compliance and performance tests that compare jstime with other JavaScript runtimes (Node.js, Deno, and Bun).

## Overview

The test suite is divided into two categories:

1. **Compliance Tests** - Verify that APIs work correctly and are compatible across runtimes
2. **Performance Tests** - Benchmark execution speed for various operations

**Runtime Compatibility:** Tests gracefully handle APIs that are not available in certain runtimes. For example:
- `process` API is not available in Deno by default
- `node:fs/promises` module is not available in Deno
- Such tests will show as "SKIPPED (API not available)" rather than failing

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

- `--api <apis>`: Run tests only for specific APIs (comma-separated)
  - Use `all` to run all tests (default behavior when flag is omitted)
  - Available APIs: console, timers, url, crypto, performance, base64, json, text-encoding, event, streams, structured-clone, microtask, arithmetic, strings, arrays, objects
  - Example: `--api crypto,url,json` runs only crypto, url, and json tests
  - Useful for focused testing during development or debugging specific APIs
  - Can be combined with `--verbose` flag
- `--verbose` or `-v`: Show detailed breakdown for each performance test
  - Shows individual sub-test results (e.g., for strings: concatenation, template literals, repeat, split/join)
  - Displays elapsed time and operations per millisecond for each sub-test
  - During execution: Shows sub-tests immediately after each benchmark
  - In summary: Shows complete breakdown for all runtimes
  - Helps identify which specific operations need performance improvements
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
- **test-fetch.js** - Fetch API (Headers, Request, Response classes)
- **test-webassembly.js** - WebAssembly API (Module, Instance, Memory, Table)
- **test-fs.mjs** - File System API (node:fs/promises - readFile, writeFile, mkdir, etc.)
- **test-process.js** - Process API (process.env, process.argv, process.cwd, process.exit)

Each test outputs: `API_NAME: X passed, Y failed`

### Performance Tests (`performance/`)

Benchmarks measure execution speed for common operations. Each benchmark includes multiple sub-tests to provide granular performance metrics:

- **bench-arithmetic.js** - Arithmetic operations (100K iterations)
  - Sub-tests: addition, subtraction, multiplication, division, mixed operations
- **bench-strings.js** - String operations (10K iterations)
  - Sub-tests: concatenation, template literals, repeat, split/join
- **bench-arrays.js** - Array operations (1K iterations, 1K elements)
  - Sub-tests: creation, map, filter, reduce, push/pop
- **bench-objects.js** - Object operations (100K iterations)
  - Sub-tests: creation, property access, property assignment, spread
- **bench-json.js** - JSON operations (100K iterations)
  - Sub-tests: stringify (small), parse (small), roundtrip (small)
- **bench-base64.js** - Base64 operations (100K iterations)
  - Sub-tests: encode, decode, roundtrip
- **bench-url.js** - URL operations (100K iterations)
  - Sub-tests: parse, searchparams, property access
- **bench-crypto.js** - Crypto operations (10K iterations)
  - Sub-tests: randomUUID, getRandomValues (small), getRandomValues (large)
- **bench-text-encoding.js** - Text encoding operations (10K iterations)
  - Sub-tests: encode, decode, roundtrip
- **bench-structured-clone.js** - Structured cloning (10K iterations)
  - Sub-tests: simple, complex, array
- **bench-event.js** - Event operations (100K iterations)
  - Sub-tests: creation, addEventListener, dispatch
- **bench-console.js** - Console API operations (100K iterations)
  - Sub-tests: log, error, warn, info
- **bench-performance.js** - Performance API operations (1M iterations)
  - Sub-tests: now, timeOrigin, measurements
- **bench-timers.js** - Timers API operations (10K iterations)
  - Sub-tests: setTimeout_cancel, setInterval_cancel, setTimeout_execute
- **bench-streams.js** - Streams API operations (1K iterations)
  - Sub-tests: readable_creation, writable_creation, transform_creation, readable_read, writable_write
- **bench-fetch.js** - Fetch API operations (10K iterations)
  - Sub-tests: headers_creation, headers_from_object, request_creation, response_creation
- **bench-webassembly.js** - WebAssembly API operations (1K iterations)
  - Sub-tests: validate, module_sync, instantiate_sync, memory_creation, table_creation
- **bench-fs.mjs** - File System API operations (1K iterations)
  - Sub-tests: write_small, read_small, stat, append, readdir
- **bench-process.js** - Process API operations (100K iterations)
  - Sub-tests: env_access, argv_access, cwd_call, stdout_access, env_keys

Each benchmark outputs JSON with timing results including aggregate totals and individual sub-test metrics.

## Running Selective Tests

You can run tests for specific APIs using the `--api` flag:

```bash
# Test only crypto and url APIs
./benchmarks/cross-runtime/run-tests.sh --api crypto,url

# Test a single API
./benchmarks/cross-runtime/run-tests.sh --api json

# Combine with verbose mode for detailed output
./benchmarks/cross-runtime/run-tests.sh --api crypto,base64 --verbose

# Test multiple related APIs
./benchmarks/cross-runtime/run-tests.sh --api arithmetic,strings,arrays,objects
```

This is useful for:
- Focused testing during development
- Debugging specific API implementations
- Faster iteration when working on particular features
- CI/CD pipelines that only need to validate certain APIs

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

With `--verbose` flag, the output shows individual sub-test results for each benchmark:

```
=== Performance Tests ===

Running bench-arithmetic...
  jstime    : 5.428ms (total)
      addition            :    1.760ms (  56809.99 ops/ms)
      subtraction         :    0.832ms ( 120223.07 ops/ms)
      multiplication      :    0.838ms ( 119312.55 ops/ms)
      division            :    0.853ms ( 117195.99 ops/ms)
      mixed               :    1.144ms (  87390.66 ops/ms)
  node      : 5.576ms (total)
      addition            :    1.199ms (  83384.27 ops/ms)
      subtraction         :    1.036ms (  96521.18 ops/ms)
      multiplication      :    0.865ms ( 115567.79 ops/ms)
      division            :    1.233ms (  81101.22 ops/ms)
      mixed               :    1.243ms (  80478.04 ops/ms)

Running bench-strings...
  jstime    : 9.740ms (total)
      concatenation       :    6.302ms (   1586.86 ops/ms)
      template_literals   :    0.675ms (  14819.10 ops/ms)
      repeat              :    0.665ms (  15042.23 ops/ms)
      split_join          :    2.099ms (   4764.45 ops/ms)
  ...

=== Summary ===

Performance Comparison:
  (Lower time is better - showing detailed breakdown)

  arithmetic:          jstime:5.428ms★ node:5.576ms
    jstime:   
        addition            :    1.760ms (  56809.99 ops/ms)
        subtraction         :    0.832ms ( 120223.07 ops/ms)
        multiplication      :    0.838ms ( 119312.55 ops/ms)
        division            :    0.853ms ( 117195.99 ops/ms)
        mixed               :    1.144ms (  87390.66 ops/ms)
    node:     
        addition            :    1.199ms (  83384.27 ops/ms)
        subtraction         :    1.036ms (  96521.18 ops/ms)
        multiplication      :    0.865ms ( 115567.79 ops/ms)
        division            :    1.233ms (  81101.22 ops/ms)
        mixed               :    1.243ms (  80478.04 ops/ms)

  strings:             jstime:9.740ms★ node:11.987ms
    jstime:   
        concatenation       :    6.302ms (   1586.86 ops/ms)
        template_literals   :    0.675ms (  14819.10 ops/ms)
        repeat              :    0.665ms (  15042.23 ops/ms)
        split_join          :    2.099ms (   4764.45 ops/ms)
    node:     
        concatenation       :    7.387ms (   1353.73 ops/ms)
        template_literals   :    0.560ms (  17859.44 ops/ms)
        repeat              :    1.120ms (   8931.96 ops/ms)
        split_join          :    2.921ms (   3423.73 ops/ms)
  ...

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
- **Skipped**: API is not available in the runtime (e.g., `process` in Deno, `node:fs/promises` in Deno)
- **Error**: Test couldn't run (e.g., syntax error, unexpected runtime error)

### Performance Tests

- **Lower times are better** (faster execution)
- The fastest runtime for each test is marked with ★
- Tests that are skipped due to unavailable APIs will show "SKIPPED (API not available)"
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
