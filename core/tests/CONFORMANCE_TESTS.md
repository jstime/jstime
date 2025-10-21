# Conformance Testing

This directory contains conformance tests for jstime's implementation of standard web APIs. These tests verify that jstime's APIs follow the behavior specified in their respective WHATWG and W3C specifications.

## Test Coverage

### Base64 Encoding
- **File**: `test_conformance_base64.rs`
- **Specification**: [WHATWG HTML Standard - Base64 utilities](https://html.spec.whatwg.org/multipage/webappapis.html#atob)
- **Tests**: 29 tests
- **Coverage**:
  - Global object properties (existence of `btoa` and `atob`)
  - Function type checks
  - Empty string handling
  - ASCII text encoding/decoding
  - Round-trip encoding/decoding
  - Special characters and null bytes
  - Latin-1 boundary testing (character code 255)
  - Error handling for characters outside Latin-1 range (Unicode, emoji, CJK)
  - Error handling for missing arguments
  - Error handling for invalid base64 input
  - Base64 padding handling (single, double, none)
  - Whitespace handling in base64 strings
  - Type coercion to string
  - Full Latin-1 character range (0-255)

### Console API
- **File**: `test_conformance_console.rs`
- **Specification**: [WHATWG Console Standard](https://console.spec.whatwg.org/)
- **Tests**: 13 tests
- **Coverage**:
  - Global object properties (existence, enumerability, configurability, writability)
  - Console methods (log, error, warn, info, debug, etc.)
  - Counting and timing methods (count, countReset, time, timeEnd, timeLog)
  - Grouping methods (group, groupCollapsed, groupEnd)
  - Assertion method (assert)

### Fetch API
- **File**: `test_conformance_fetch.rs`
- **Specification**: [WHATWG Fetch Standard](https://fetch.spec.whatwg.org/)
- **Tests**: 32 tests
- **Coverage**:
  - `fetch()` global function
  - `Headers` class (append, delete, get, has, set, iteration)
  - `Request` class (URL, method, headers, request cloning)
  - `Response` class (status, ok, headers, text(), json())
  - Promise-based API behavior

### Performance API
- **File**: `test_conformance_performance.rs`
- **Specification**: [W3C High Resolution Time](https://w3c.github.io/hr-time/) and [Performance Timeline](https://w3c.github.io/performance-timeline/)
- **Tests**: 19 tests
- **Coverage**:
  - `performance` global object
  - `performance.now()` method (returns high-resolution timestamp)
  - `performance.timeOrigin` property
  - Monotonic time guarantees
  - JSON serialization via `toJSON()`

### URL API
- **File**: `test_conformance_url.rs`
- **Specification**: [WHATWG URL Standard](https://url.spec.whatwg.org/)
- **Tests**: 26 tests
- **Coverage**:
  - `URL` constructor (absolute URLs, relative URLs with base)
  - URL properties (protocol, hostname, pathname, search, hash, origin, etc.)
  - URL property setters
  - `URLSearchParams` class (get, set, append, delete, has, getAll)
  - URLSearchParams iteration
  - Live binding between URL and searchParams

### WebAssembly
- **File**: `test_conformance_webassembly.rs`
- **Specification**: [WebAssembly JavaScript Interface](https://webassembly.github.io/spec/js-api/)
- **Tests**: 28 tests
- **Coverage**:
  - `WebAssembly` namespace
  - `WebAssembly.Module` constructor and static methods (imports, exports, customSections)
  - `WebAssembly.Instance` constructor and exports property
  - `WebAssembly.Memory` constructor, buffer property, and grow method
  - `WebAssembly.Table` constructor, length property, and methods (get, set, grow)
  - `WebAssembly.compile()` asynchronous compilation
  - `WebAssembly.instantiate()` asynchronous instantiation
  - `WebAssembly.validate()` bytecode validation
  - Error constructors (CompileError, LinkError, RuntimeError)
  - Error hierarchy and inheritance

### Timers API
- **File**: `test_conformance_timers.rs`
- **Specification**: [WHATWG HTML Standard - Timers](https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html)
- **Tests**: 17 tests
- **Coverage**:
  - `setTimeout()` and `clearTimeout()` functions
  - `setInterval()` and `clearInterval()` functions
  - Timer execution and cancellation
  - Argument passing to timer callbacks
  - Timer ordering and execution

## Running Conformance Tests

To run all conformance tests:

```bash
cargo test --test test_conformance_base64 --test test_conformance_console --test test_conformance_fetch --test test_conformance_performance --test test_conformance_timers --test test_conformance_url --test test_conformance_webassembly
```

To run a specific conformance test suite:

```bash
# Base64 conformance tests
cargo test --test test_conformance_base64

# Console conformance tests
cargo test --test test_conformance_console

# Fetch conformance tests
cargo test --test test_conformance_fetch

# Performance conformance tests
cargo test --test test_conformance_performance

# Timers conformance tests
cargo test --test test_conformance_timers

# URL conformance tests
cargo test --test test_conformance_url

# WebAssembly conformance tests
cargo test --test test_conformance_webassembly
```

## Test Philosophy

These conformance tests focus on:

1. **Specification Compliance**: Tests verify behavior defined in official specifications
2. **API Availability**: Ensuring required constructors, functions, and methods exist
3. **Correct Behavior**: Validating that APIs behave as specified
4. **Edge Cases**: Testing boundary conditions and special cases
5. **Interoperability**: Ensuring jstime can run standard JavaScript code

## Limitations

These tests verify the core functionality and specification compliance. They do not:

- Test every possible edge case covered by Web Platform Tests (WPT)
- Include tests that require network access or external resources
- Test browser-specific features not applicable to a standalone runtime
- Cover features not yet implemented in jstime

## Adding New Conformance Tests

When adding new conformance tests:

1. Create a new test file `test_conformance_<api>.rs` in the `tests/` directory
2. Include a module-level comment with the specification URL
3. Group related tests in a module
4. Use descriptive test names that indicate what's being tested
5. Follow the existing test structure and patterns
6. Update this README with the new test suite information

## Total Coverage

- **Total Test Files**: 7
- **Total Tests**: 164
- **APIs Covered**: Base64 Encoding, Console, Fetch, Performance, Timers, URL, WebAssembly

All tests pass ✓
