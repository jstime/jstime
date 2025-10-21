# Conformance Testing

This directory contains conformance tests for jstime's implementation of standard web APIs. These tests verify that jstime's APIs follow the behavior specified in their respective WHATWG and W3C specifications.

## Test Coverage

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
cargo test --test test_conformance_console --test test_conformance_fetch --test test_conformance_performance --test test_conformance_timers --test test_conformance_url
```

To run a specific conformance test suite:

```bash
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

- **Total Test Files**: 5
- **Total Tests**: 107
- **APIs Covered**: Console, Fetch, Performance, Timers, URL

All tests pass ✓
