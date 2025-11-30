# jstime Documentation

jstime is a minimal and performant JavaScript runtime built on top of V8. This document provides an overview of the various features and APIs supported by jstime.

**🚀 Want to see these features in action?** Check out the [examples/](../examples/) directory for runnable code samples demonstrating each API!

## Features

jstime's feature documentation is organized into the following categories:

### Core Runtime

- **[JavaScript Runtime](runtime.md)** - JavaScript language support, REPL, script execution, and limitations

### Web Standard APIs

- **[Web APIs](apis/web-apis.md)** - WHATWG/W3C standard web APIs
  - Console API - Logging and debugging
  - Event and EventTarget - Event handling
  - Timers - setTimeout, setInterval
  - Fetch API - HTTP requests
  - Streams API - Streaming data processing
  - URL API - URL parsing and manipulation
  - Performance API - High-resolution timing
  - Microtask API - Microtask queuing
  - Structured Clone API - Deep cloning
  - Base64 Encoding - Base64 encoding/decoding

- **[Text Encoding & Cryptography](apis/encoding-crypto.md)** - Encoding and cryptographic APIs
  - Text Encoding API - UTF-8 encoding/decoding
  - Web Cryptography API - Secure random values and hashing

### System Integration

- **[System APIs](apis/system.md)** - Operating system and file system interaction
  - Process API - Environment variables, command-line arguments, working directory
  - File System API - Node.js-compatible fs/promises API
  - UDP/Datagram Sockets API - Node.js-compatible dgram module for UDP networking

### Module System

- **[Module System](apis/modules.md)** - JavaScript modules and WebAssembly
  - ES Modules - import/export, top-level await
  - WebAssembly - High-performance compiled modules

### Debugging & Error Handling

- **[Error Handling and Debugging](apis/error-handling.md)** - Enhanced error messages and debugging
  - Color-coded error output
  - Helpful hints for common errors
  - Stack traces with source locations
  - Source map support (planned)

## Quick Feature Overview

### JavaScript Language Support

jstime uses V8 as its JavaScript engine, providing full support for modern JavaScript features including:

- **ES2015+ (ES6+)**: All modern JavaScript syntax and features
- **Async/Await**: Asynchronous programming with async functions
- **Top-level await**: Use await at the top level of ES modules
- **Promises**: Native Promise support
- **Classes**: ES6 class syntax
- **Arrow functions**: Concise function expressions
- **Template literals**: String interpolation
- **Destructuring**: Object and array destructuring
- **Spread operator**: Spread syntax for arrays and objects
- **And more**: All standard JavaScript features supported by V8

### Essential APIs

| API Category | Key Features |
|--------------|--------------|
| **Console** | `console.log()`, `console.error()`, format specifiers |
| **Timers** | `setTimeout()`, `setInterval()`, and clearing functions |
| **Fetch** | Modern HTTP client with `fetch()`, `Headers`, `Request`, `Response` |
| **Streams** | `ReadableStream`, `WritableStream`, `TransformStream` |
| **URL** | `URL` and `URLSearchParams` for URL manipulation |
| **Performance** | High-resolution timing with `performance.now()` |
| **Events** | `Event` and `EventTarget` for event handling |
| **Text Encoding** | `TextEncoder` and `TextDecoder` for UTF-8 |
| **Cryptography** | `crypto.getRandomValues()`, `crypto.randomUUID()`, `crypto.subtle.digest()` |
| **Process** | `process.env`, `process.argv`, `process.cwd()`, `process.exit()` |
| **File System** | Complete Node.js-compatible `fs/promises` API |
| **UDP Sockets** | Node.js-compatible `dgram` module for UDP networking |
| **WebAssembly** | Full WebAssembly support |
| **ES Modules** | Full support for `import`/`export` with top-level await |

### Examples

Each API has example code in the [examples/](../examples/) directory:

- [console-demo.js](../examples/console-demo.js) - Console API examples
- [timers-demo.js](../examples/timers-demo.js) - Timer examples
- [fetch-demo.mjs](../examples/fetch-demo.mjs) - Fetch API examples
- [streams-demo.js](../examples/streams-demo.js) - Streams API examples
- [url-demo.js](../examples/url-demo.js) - URL API examples
- [performance-demo.js](../examples/performance-demo.js) - Performance API examples
- [events-demo.js](../examples/events-demo.js) - Event handling examples
- [structured-clone-demo.mjs](../examples/structured-clone-demo.mjs) - Structured clone examples
- [text-encoding-demo.js](../examples/text-encoding-demo.js) - Text encoding examples
- [error-handling-demo.js](../examples/error-handling-demo.js) - Error handling and debugging examples
- [fs-demo.js](../examples/fs-demo.js) - File system basic examples
- [fs-complete-demo.js](../examples/fs-complete-demo.js) - File system complete API examples
- [webassembly-demo.js](../examples/webassembly-demo.js) - WebAssembly examples
- [json-import-example.js](../examples/json-import-example.js) - JSON module import examples

## Testing

jstime includes comprehensive test coverage including conformance tests for standard APIs. For details on conformance testing, see [core/tests/CONFORMANCE_TESTS.md](../core/tests/CONFORMANCE_TESTS.md).

Run all tests with:
```bash
cargo test
```

## Additional Resources

- [README.md](../README.md) - Getting started guide
- [ARCHITECTURE.md](../ARCHITECTURE.md) - Detailed architecture documentation
- [PERFORMANCE.md](../PERFORMANCE.md) - Performance optimization details
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution guidelines
- [GitHub Repository](https://github.com/jstime/jstime) - Source code and issues
