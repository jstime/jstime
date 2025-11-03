# jstime

> Another JavaScript Runtime

![jstime logo. Kinda looks like shrek](./logo.png)

## Using the binary

You can find the latest jstime binary on the [release page](https://github.com/jstime/jstime/releases)

Alternatively you can install with cargo

```bash
$ cargo install jstime
```

### As a repl

```bash
$ jstime

Welcome to jstime!

>>
```

### Run a script

```bash
$ cat hello-world.js
console.log("hello world");

$ jstime hello-world.js
hello world

```

### Performance: JIT Warmup

For benchmarking or performance-critical scripts, use the `--warmup` flag to allow V8's JIT compiler to optimize the code:

```bash
$ jstime --warmup 10 benchmark.js
```

This runs the script 10 times before the actual execution, allowing TurboFan to profile and optimize hot code paths.

## Embed it!

Check out the [README.md for jstime-core](./core/README.md) for
instructions on how to embed jstime in your rust application!

## Features

jstime provides a minimal and performant JavaScript runtime with essential APIs.

For detailed documentation on all supported features, see [Documentation](./docs/README.md).

**👉 Check out the [examples/](./examples/) directory for runnable code samples!**

### Quick Overview

- **Console API**: `console.log()`, `console.error()`, etc.
- **Timers**: `setTimeout()`, `setInterval()`, and clearing functions
- **Fetch API**: Modern HTTP client with `fetch()`, `Headers`, `Request`, `Response`
- **Streams API**: `ReadableStream`, `WritableStream`, `TransformStream` for streaming data processing
- **URL API**: `URL` and `URLSearchParams` for URL manipulation
- **Performance API**: High-resolution timing with `performance.now()`
- **Microtask API**: `queueMicrotask()` for fine-grained async control
- **Structured Clone API**: `structuredClone()` for deep cloning of complex objects
- **Text Encoding API**: `TextEncoder` and `TextDecoder` for UTF-8 encoding/decoding
- **Crypto API**: `crypto.getRandomValues()`, `crypto.randomUUID()`, `crypto.subtle.digest()` for cryptographic operations
- **Process API**: `process.env`, `process.argv`, `process.cwd()`, `process.exit()` for process information
- **File System API**: Complete Node.js-compatible `fs/promises` with `readFile()`, `writeFile()`, `mkdir()`, `stat()`, and more
- **WebAssembly**: Full WebAssembly support for running high-performance compiled modules
- **ES Modules**: Full support for `import`/`export` with top-level await and dynamic `import()`
- **Modern JavaScript**: All ES2015+ features via V8 engine

## Testing

jstime includes comprehensive test coverage including conformance tests for standard APIs:

- **Console API** - WHATWG Console Standard compliance
- **Fetch API** - WHATWG Fetch Standard compliance
- **Streams API** - WHATWG Streams Standard compliance
- **Performance API** - W3C High Resolution Time compliance
- **URL API** - WHATWG URL Standard compliance
- **Timers API** - WHATWG HTML Standard timers compliance
- **Structured Clone API** - HTML Standard structured cloning compliance

Run all tests with:
```bash
cargo test
```

For more details on conformance testing, see [core/tests/CONFORMANCE_TESTS.md](./core/tests/CONFORMANCE_TESTS.md).
