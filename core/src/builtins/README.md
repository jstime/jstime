# Built-in APIs

This directory contains the implementation of JavaScript APIs provided by jstime. Each API consists of two files:

- **`*_impl.rs`** - Rust implementation with V8 bindings
- **`*.js`** - JavaScript polyfill for spec-compliant behavior

## Available APIs

### WHATWG/W3C Standards

| API | Files | Specification |
|-----|-------|---------------|
| **Console** | `console_impl.rs`, `console.js` | [WHATWG Console](https://console.spec.whatwg.org/) |
| **Timers** | `timers_impl.rs`, `timers.js` | [WHATWG HTML](https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#timers) |
| **Fetch** | `fetch_impl.rs`, `fetch.js` | [WHATWG Fetch](https://fetch.spec.whatwg.org/) |
| **URL** | `url_impl.rs`, `url.js` | [WHATWG URL](https://url.spec.whatwg.org/) |
| **Events** | `event_impl.rs`, `event.js` | [WHATWG DOM](https://dom.spec.whatwg.org/#events) |
| **Base64** | `base64_impl.rs`, `base64.js` | [WHATWG HTML](https://html.spec.whatwg.org/multipage/webappapis.html#atob) |
| **Structured Clone** | `structured_clone_impl.rs`, `structured_clone.js` | [WHATWG HTML](https://html.spec.whatwg.org/multipage/structured-data.html#structured-cloning) |
| **Performance** | `performance_impl.rs`, `performance.js` | [W3C HR Time](https://w3c.github.io/hr-time/) |
| **Microtask** | `queue_microtask_impl.rs`, `queue_microtask.js` | [WHATWG HTML](https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#microtask-queuing) |

### Node.js Compatible

| API | Files | Specification |
|-----|-------|---------------|
| **File System** | `fs_impl.rs`, `fs.js` | [Node.js fs/promises](https://nodejs.org/api/fs.html#promises-api) |

## Adding a New API

See [CONTRIBUTING.md](../../../CONTRIBUTING.md#adding-built-in-apis) for instructions.

**Quick steps:**
1. Create `your_api_impl.rs` (Rust) and `your_api.js` (JavaScript)
2. Register in `mod.rs`
3. Write tests in `core/tests/`
4. Update `docs/FEATURES.md`

## File Organization

Each pair of files (`*.js` and `*_impl.rs`) implements a complete API following web standards.
