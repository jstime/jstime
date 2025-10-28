# Built-in APIs

This directory contains the implementation of JavaScript APIs provided by jstime, organized by standards body.

Each API consists of two files:
- **`*_impl.rs`** - Rust implementation with V8 bindings
- **`*.js`** - JavaScript polyfill for spec-compliant behavior

## Directory Structure

```
builtins/
├── whatwg/       # WHATWG Standards
├── w3c/          # W3C Standards
└── node/         # Node.js Compatible APIs
```

## Available APIs

### WHATWG Standards (`whatwg/`)

| API | Files | Specification |
|-----|-------|---------------|
| **Fetch** | `fetch_impl.rs`, `fetch.js` | [WHATWG Fetch](https://fetch.spec.whatwg.org/) |
| **URL** | `url_impl.rs`, `url.js` | [WHATWG URL](https://url.spec.whatwg.org/) |
| **Events** | `event_impl.rs`, `event.js` | [WHATWG DOM](https://dom.spec.whatwg.org/#events) |
| **Console** | `console_impl.rs`, `console.js` | [WHATWG Console](https://console.spec.whatwg.org/) |
| **Base64** | `base64_impl.rs`, `base64.js` | [WHATWG HTML](https://html.spec.whatwg.org/multipage/webappapis.html#atob) |
| **Timers** | `timers_impl.rs`, `timers.js` | [WHATWG HTML](https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#timers) |
| **Microtask** | `queue_microtask_impl.rs`, `queue_microtask.js` | [WHATWG HTML](https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#microtask-queuing) |
| **Structured Clone** | `structured_clone_impl.rs`, `structured_clone.js` | [WHATWG HTML](https://html.spec.whatwg.org/multipage/structured-data.html#structured-cloning) |

### W3C Standards (`w3c/`)

| API | Files | Specification |
|-----|-------|---------------|
| **Performance** | `performance_impl.rs`, `performance.js` | [W3C HR Time](https://w3c.github.io/hr-time/) |

### Node.js Compatible (`node/`)

| API | Files | Specification |
|-----|-------|---------------|
| **File System** | `fs_impl.rs`, `fs.js` | [Node.js fs/promises](https://nodejs.org/api/fs.html#promises-api) |

## Adding a New API

See [CONTRIBUTING.md](../../../CONTRIBUTING.md#adding-built-in-apis) for instructions.

**Quick steps:**
1. Create `your_api_impl.rs` (Rust) and `your_api.js` (JavaScript) in the appropriate directory (whatwg/, w3c/, or node/)
2. Register in `mod.rs`
3. Write tests in `core/tests/`
4. Update the appropriate documentation file:
   - Web APIs: `docs/apis/web-apis.md`
   - Text Encoding/Crypto: `docs/apis/encoding-crypto.md`
   - System APIs: `docs/apis/system.md`
   - Module System: `docs/apis/modules.md`
   - Runtime features: `docs/runtime.md`
   - Update `docs/README.md` if adding a new API category
