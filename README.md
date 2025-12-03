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
- **Event API**: `Event` and `EventTarget` for event handling
- **Microtask API**: `queueMicrotask()` for fine-grained async control
- **Structured Clone API**: `structuredClone()` for deep cloning of complex objects
- **Base64 API**: `atob()` and `btoa()` for base64 encoding/decoding
- **Text Encoding API**: `TextEncoder` and `TextDecoder` for UTF-8 encoding/decoding
- **Crypto API**: `crypto.getRandomValues()`, `crypto.randomUUID()`, `crypto.subtle.*` (digest, sign, verify, encrypt, decrypt)
- **Process API**: `process.env`, `process.argv`, `process.cwd()`, `process.exit()`, `process.stdout`, `process.stderr`, `process.stdin`
- **File System API**: Complete Node.js-compatible `fs/promises` with `readFile()`, `writeFile()`, `mkdir()`, `stat()`, and more
- **Buffer API**: Node.js-compatible `Buffer` for binary data operations
- **UDP Sockets API**: Node.js-compatible `dgram` module for UDP networking
- **WebAssembly**: Full WebAssembly support for running high-performance compiled modules
- **ES Modules**: Full support for `import`/`export` with top-level await, dynamic `import()`, and **Node.js-compatible `node_modules` resolution**
- **Modern JavaScript**: All ES2015+ features via V8 engine

## Testing

jstime includes comprehensive test coverage including conformance tests for standard APIs:

- **Console API** - WHATWG Console Standard compliance
- **Event API** - WHATWG DOM Events compliance
- **Timers API** - WHATWG HTML Standard timers compliance
- **Fetch API** - WHATWG Fetch Standard compliance
- **Streams API** - WHATWG Streams Standard compliance
- **URL API** - WHATWG URL Standard compliance
- **Performance API** - W3C High Resolution Time compliance
- **Structured Clone API** - HTML Standard structured cloning compliance
- **Base64 API** - HTML Standard base64 encoding compliance
- **Text Encoding API** - WHATWG Encoding Standard compliance
- **Crypto API** - W3C Web Cryptography API compliance
- **JSON Modules** - ECMAScript JSON modules compliance
- **WebAssembly** - WebAssembly specification compliance

Run all tests with:
```bash
cargo test
```

For more details on conformance testing, see [core/tests/CONFORMANCE_TESTS.md](./core/tests/CONFORMANCE_TESTS.md).
