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

## Embed it!

Check out the [README.md for jstime-core](./core/README.md) for
instructions on how to embed jstime in your rust application!

## Features

jstime provides a minimal and performant JavaScript runtime with essential APIs.

For detailed documentation on all supported features, see [FEATURES.md](./docs/FEATURES.md).

**👉 Check out the [examples/](./examples/) directory for runnable code samples!**

### Quick Overview

- **Console API**: `console.log()`, `console.error()`, etc.
- **Timers**: `setTimeout()`, `setInterval()`, and clearing functions
- **Fetch API**: Modern HTTP client with `fetch()`, `Headers`, `Request`, `Response`
- **URL API**: `URL` and `URLSearchParams` for URL manipulation
- **Performance API**: High-resolution timing with `performance.now()`
- **Microtask API**: `queueMicrotask()` for fine-grained async control
- **Structured Clone API**: `structuredClone()` for deep cloning of complex objects
- **File System API**: Complete Node.js-compatible `fs/promises` with `readFile()`, `writeFile()`, `mkdir()`, `stat()`, and more
- **WebAssembly**: Full WebAssembly support for running high-performance compiled modules
- **ES Modules**: Full support for `import`/`export` with top-level await
- **Modern JavaScript**: All ES2015+ features via V8 engine

## Testing

jstime includes comprehensive test coverage including conformance tests for standard APIs:

- **Console API** - WHATWG Console Standard compliance
- **Fetch API** - WHATWG Fetch Standard compliance  
- **Performance API** - W3C High Resolution Time compliance
- **URL API** - WHATWG URL Standard compliance
- **Timers API** - WHATWG HTML Standard timers compliance
- **Structured Clone API** - HTML Standard structured cloning compliance

Run all tests with:
```bash
cargo test
```

For more details on conformance testing, see [core/tests/CONFORMANCE_TESTS.md](./core/tests/CONFORMANCE_TESTS.md).

## Current Project Team Members

For information about the governance of the jstime project, see
[GOVERNANCE.md](./GOVERNANCE.md).

## Contributing

Interested in contributing? We'd love to have you! ❤️

**👉 New to the project? Start with the [Quick Start Guide](./QUICK_START.md)** - Get up and running in 5 minutes!

### Resources for Contributors

- **[QUICK_START.md](./QUICK_START.md)** - 5-minute setup guide for new contributors
- **[DEVELOPMENT.md](./DEVELOPMENT.md)** - Complete development workflow and guidelines
- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - Learn how jstime is built
- **[CONTRIBUTING.md](./CONTRIBUTING.md)** - Contribution guidelines and code of conduct
- **[CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md)** - Community standards

We welcome contributions of all kinds: bug fixes, features, documentation, examples, and more!

## Project Team

### Chair

* [MylesBorins](https://github.com/MylesBorins) - **Myles Borins** (he/him)

### Collaborators

* [bengl](https://github.com/bengl) - **Bryan English** (he/him)
* [bdougie](https://github.com/bdougie) - **Brian Douglas**
* [codebytere](https://github.com/codebytere) - **Shelley Vohr**
* [devsnek](https://github.com/devsnek) - **Gus Caplan**
* [EstebanBorai](https://github.com/EstebanBorai) - **Esteban Borai**
* [jalafel](https://github.com/jalafel) - **Jess Tran**
* [MylesBorins](https://github.com/MylesBorins) - **Myles Borins** (he/him)
* [solumos](https://github.com/solumos) - **Tom Hadley**

