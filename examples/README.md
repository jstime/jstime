# jstime Examples

This directory contains example scripts demonstrating various features of jstime. Each example is designed to be standalone and showcases a specific API or feature.

## Running Examples

To run any example, use the `jstime` binary:

```bash
jstime examples/console-demo.js
```

For ES module examples (`.mjs` files), you can run them the same way:

```bash
jstime examples/fetch-demo.mjs
```

## Available Examples

### Console API

**File:** `console-demo.js`

Demonstrates the Console API for logging and debugging:
- Basic `console.log()`, `console.info()`, `console.debug()`
- Warning and error messages with `console.warn()` and `console.error()`
- Logging objects, arrays, and nested structures
- Format specifiers (`%s`, `%d`, `%f`)
- Handling special values (null, undefined, NaN, Infinity)

**To run:**
```bash
jstime examples/console-demo.js
```

**See also:** [Console API documentation](../docs/apis/web-apis.md#console-api)

---

### Timers

**File:** `timers-demo.js`

Demonstrates the Timers API for scheduling code execution:
- Using `setTimeout()` to delay execution
- Using `setInterval()` for repeated execution
- Passing arguments to timer callbacks
- Clearing timers with `clearTimeout()` and `clearInterval()`
- Running multiple concurrent timers
- Nested timers

**To run:**
```bash
jstime examples/timers-demo.js
```

**See also:** [Timers documentation](../docs/apis/web-apis.md#timers)

---

### Fetch API

**File:** `fetch-demo.mjs` (ES Module)

Demonstrates the Fetch API for making HTTP requests:
- Simple GET requests
- Working with Headers
- Using Request objects
- Checking response status
- Parsing JSON and text responses
- Error handling

**Note:** This example requires network access and may fail in restricted environments.

**To run:**
```bash
jstime examples/fetch-demo.mjs
```

**See also:** [Fetch API documentation](../docs/apis/web-apis.md#fetch-api)

---

### URL API

**File:** `url-demo.js`

Demonstrates the URL and URLSearchParams APIs:
- Parsing URLs and accessing components
- Constructing URLs with base paths
- Modifying URL components
- Working with query parameters using URLSearchParams
- Iterating over parameters
- URL encoding

**To run:**
```bash
jstime examples/url-demo.js
```

**See also:** [URL API documentation](../docs/apis/web-apis.md#url-api)

---

### Performance API

**File:** `performance-demo.js`

Demonstrates the Performance API for high-resolution timing:
- Using `performance.now()` for timing measurements
- Measuring function execution time
- Comparing different operations
- High-resolution timing (microsecond precision)
- Benchmarking different implementations
- Using `performance.timeOrigin`

**To run:**
```bash
jstime examples/performance-demo.js
```

**See also:** [Performance API documentation](../docs/apis/web-apis.md#performance-api)

---

### Event and EventTarget

**File:** `events-demo.js`

Demonstrates the Event and EventTarget APIs:
- Creating events with `new Event()`
- Creating event targets with `new EventTarget()`
- Adding and removing event listeners
- Multiple listeners for the same event
- Event properties (type, target, timestamp)
- Preventing default actions and stopping propagation
- Custom event types
- Event emitter pattern

**To run:**
```bash
jstime examples/events-demo.js
```

**See also:** [Event and EventTarget documentation](../docs/apis/web-apis.md#event-and-eventtarget)

---

### Structured Clone

**File:** `structured-clone-demo.mjs` (ES Module)

Demonstrates the Structured Clone API for deep cloning:
- Basic cloning of objects and arrays
- Deep cloning nested structures
- Cloning special types (Date, RegExp, Map, Set)
- Cloning ArrayBuffer and typed arrays
- Handling circular references
- Comparison with JSON serialization
- Error cases (functions, symbols)

**To run:**
```bash
jstime examples/structured-clone-demo.mjs
```

**See also:** [Structured Clone documentation](../docs/apis/web-apis.md#structured-clone-api)

---

### Text Encoding API

**File:** `text-encoding-demo.js`

Demonstrates the Text Encoding API for converting between strings and UTF-8 bytes:
- Using `TextEncoder` to encode strings to UTF-8
- Using `TextDecoder` to decode UTF-8 bytes to strings
- Encoding multi-byte characters (€, 世, 界, 😀)
- Working with different languages and scripts
- Using `encodeInto()` for efficient encoding into existing buffers
- Handling buffer overflow scenarios
- Round-trip encoding/decoding
- Understanding UTF-8 byte sizes

**To run:**
```bash
jstime examples/text-encoding-demo.js
```

**See also:** [Text Encoding API documentation](../docs/apis/encoding-crypto.md#text-encoding-api)

---

### File System API

**File:** `fs-demo.js` (ES Module)

Demonstrates the basic Node.js-compatible File System API:
- Reading files as text (UTF-8)
- Reading files as buffers
- Reading files with options object
- Listing directory contents
- Error handling

**To run:**
```bash
jstime examples/fs-demo.js
```

**Advanced example:** `fs-complete-demo.js` - Shows all filesystem operations including writing, copying, renaming, and more.

**See also:** [File System API documentation](../docs/apis/system.md#file-system-api)

---

### WebAssembly

**File:** `webassembly-demo.js`

Demonstrates WebAssembly support in jstime:
- Validating WebAssembly bytecode
- Creating and instantiating WebAssembly modules
- Calling exported WebAssembly functions
- Working with WebAssembly Memory
- Working with WebAssembly Tables
- Error handling with WebAssembly

**To run:**
```bash
jstime examples/webassembly-demo.js
```

**See also:** [WebAssembly documentation](../docs/apis/modules.md#webassembly)

---

### JSON Imports

**File:** `json-import-example.js` (ES Module)

Demonstrates importing JSON files as ES modules:
- Using `import` statements with JSON files
- Accessing imported JSON data

**To run:**
```bash
jstime examples/json-import-example.js
```

**See also:** [ES Modules documentation](../docs/apis/modules.md#es-modules)

---

## Examples by Category

### Core APIs
- [Console API](console-demo.js) - Logging and debugging
- [Timers](timers-demo.js) - setTimeout, setInterval
- [Performance](performance-demo.js) - High-resolution timing
- [Events](events-demo.js) - Event and EventTarget

### Web APIs
- [Fetch API](fetch-demo.mjs) - HTTP requests
- [URL API](url-demo.js) - URL parsing and manipulation

### Data APIs
- [Structured Clone](structured-clone-demo.mjs) - Deep cloning objects
- [Text Encoding](text-encoding-demo.js) - UTF-8 encoding and decoding

### File System
- [Basic File System](fs-demo.js) - Reading files and directories
- [Complete File System](fs-complete-demo.js) - All filesystem operations

### Advanced Features
- [WebAssembly](webassembly-demo.js) - Running compiled modules
- [JSON Imports](json-import-example.js) - ES module JSON imports

## Creating Your Own Examples

Feel free to create your own examples to explore jstime's features! 

**Tips:**
- Use `.js` extension for regular scripts
- Use `.mjs` extension for ES modules (enables top-level `await` and `import`/`export`)
- All examples should be standalone and runnable with just `jstime examples/your-example.js`
- Add clear comments explaining what each section demonstrates
- Follow the structure of existing examples for consistency

## Additional Resources

- [Main README](../README.md) - Getting started with jstime
- [Features Documentation](../docs/FEATURES.md) - Complete API reference
- [Web APIs](../docs/apis/web-apis.md) - Web standard APIs
- [System APIs](../docs/apis/system.md) - Process and file system APIs
- [Module System](../docs/apis/modules.md) - ES Modules and WebAssembly
- [Contributing Guide](../CONTRIBUTING.md) - How to contribute
