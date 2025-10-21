# jstime Features

jstime is a minimal and performant JavaScript runtime built on top of V8. This document provides an overview of the various features and APIs supported by jstime.

## Table of Contents

- [JavaScript Language Support](#javascript-language-support)
- [Console API](#console-api)
- [Event and EventTarget](#event-and-eventtarget)
- [Timers](#timers)
- [Fetch API](#fetch-api)
- [URL API](#url-api)
- [Performance API](#performance-api)
- [Microtask API](#microtask-api)
- [Structured Clone API](#structured-clone-api)
- [Base64 Encoding](#base64-encoding)
- [WebAssembly](#webassembly)
- [ES Modules](#es-modules)
- [REPL](#repl)

## JavaScript Language Support

jstime uses V8 as its JavaScript engine, providing full support for modern JavaScript features including:

- **ES2015+ (ES6+)**: All modern JavaScript syntax and features
- **Async/Await**: Asynchronous programming with async functions and await expressions
- **Top-level await**: Use await at the top level of ES modules
- **Promises**: Native Promise support for asynchronous operations
- **Classes**: ES6 class syntax
- **Arrow functions**: Concise function expressions
- **Template literals**: String interpolation and multi-line strings
- **Destructuring**: Object and array destructuring
- **Spread operator**: Spread syntax for arrays and objects
- **And more**: All standard JavaScript features supported by V8

### Example

```javascript
// Modern JavaScript features work out of the box
const greet = (name) => `Hello, ${name}!`;
console.log(greet('World'));

// Classes
class Person {
  constructor(name) {
    this.name = name;
  }
  
  greet() {
    return `Hello, I'm ${this.name}`;
  }
}

const person = new Person('Alice');
console.log(person.greet());

// Async/await
async function fetchData() {
  const result = await Promise.resolve('data');
  return result;
}
```

## Console API

jstime implements the [WHATWG Console Standard](https://console.spec.whatwg.org/#console-namespace), providing familiar logging methods for debugging and output.

### Supported Methods

- `console.log()` - Output informational messages
- `console.info()` - Output informational messages (same as log)
- `console.debug()` - Output debug messages
- `console.error()` - Output error messages to stderr
- `console.warn()` - Output warning messages to stderr

### Format Specifiers

The console API supports format specifiers for string formatting:

- `%s` - String
- `%d` or `%i` - Integer
- `%f` - Floating point number

### Examples

```javascript
// Basic logging
console.log('Hello, World!');
console.error('An error occurred');
console.warn('This is a warning');

// Multiple arguments
console.log('Name:', 'Alice', 'Age:', 30);

// Format specifiers
console.log('Hello %s', 'World');
console.log('Integer: %d', 42);
console.log('Float: %f', 3.14159);
```

## Event and EventTarget

jstime implements the [DOM Standard Event and EventTarget interfaces](https://dom.spec.whatwg.org/#events), providing a standard way to handle events in JavaScript.

### Supported APIs

- `Event` - Represents an event that occurs
- `EventTarget` - Base class for objects that can receive events and have listeners for them

### Event Class

The `Event` class represents an event that takes place in the DOM or any other event-driven context.

#### Constructor

```javascript
new Event(type, eventInitDict)
```

- `type` (string) - The type of event (e.g., 'click', 'load', 'custom')
- `eventInitDict` (optional object) - Configuration object with:
  - `bubbles` (boolean, default: false) - Whether the event bubbles
  - `cancelable` (boolean, default: false) - Whether the event can be cancelled
  - `composed` (boolean, default: false) - Whether the event will trigger listeners outside of a shadow root

#### Properties

- `type` (read-only) - The type of the event
- `target` (read-only) - The object to which the event was originally dispatched
- `currentTarget` (read-only) - The object whose event listener is currently being processed
- `eventPhase` (read-only) - The current phase of event flow (0: NONE, 1: CAPTURING_PHASE, 2: AT_TARGET, 3: BUBBLING_PHASE)
- `bubbles` (read-only) - Whether the event bubbles
- `cancelable` (read-only) - Whether the event can be cancelled
- `defaultPrevented` (read-only) - Whether preventDefault() was called
- `composed` (read-only) - Whether the event will trigger listeners outside of a shadow root
- `isTrusted` (read-only) - Whether the event was initiated by the browser (always false for user-created events)
- `timeStamp` (read-only) - The time when the event was created (in milliseconds)

#### Methods

- `preventDefault()` - Cancels the event if it is cancelable, preventing the default action
- `stopPropagation()` - Prevents further propagation of the event
- `stopImmediatePropagation()` - Prevents other listeners of the same event from being called

#### Constants

- `Event.NONE` (0)
- `Event.CAPTURING_PHASE` (1)
- `Event.AT_TARGET` (2)
- `Event.BUBBLING_PHASE` (3)

### EventTarget Class

The `EventTarget` class is an interface implemented by objects that can receive events and have listeners for them.

#### Constructor

```javascript
new EventTarget()
```

#### Methods

- `addEventListener(type, listener, options)` - Registers an event listener
  - `type` (string) - The event type to listen for
  - `listener` (function or object) - The callback function or object with a `handleEvent` method
  - `options` (optional) - Options object (currently ignored but accepted for compatibility)

- `removeEventListener(type, listener, options)` - Removes an event listener
  - `type` (string) - The event type
  - `listener` (function or object) - The listener to remove
  - `options` (optional) - Options object (currently ignored but accepted for compatibility)

- `dispatchEvent(event)` - Dispatches an event to this EventTarget
  - `event` (Event) - The event to dispatch
  - Returns `true` if the event was not cancelled, `false` otherwise

### Examples

#### Basic Event Usage

```javascript
// Create an event target
const button = new EventTarget();

// Create an event
const clickEvent = new Event('click');

// Add an event listener
button.addEventListener('click', (e) => {
  console.log('Button was clicked!');
  console.log('Event type:', e.type);
});

// Dispatch the event
button.dispatchEvent(clickEvent);
```

#### Event with Options

```javascript
const target = new EventTarget();

// Create a cancelable event
const event = new Event('submit', {
  bubbles: true,
  cancelable: true
});

target.addEventListener('submit', (e) => {
  console.log('Preventing default action');
  e.preventDefault();
});

const notCancelled = target.dispatchEvent(event);
console.log('Event was cancelled:', !notCancelled);
```

#### Multiple Listeners

```javascript
const target = new EventTarget();

target.addEventListener('custom', () => {
  console.log('Handler 1');
});

target.addEventListener('custom', () => {
  console.log('Handler 2');
});

target.addEventListener('custom', () => {
  console.log('Handler 3');
});

// All three handlers will be called in order
target.dispatchEvent(new Event('custom'));
```

#### Stopping Propagation

```javascript
const target = new EventTarget();

target.addEventListener('test', (e) => {
  console.log('Handler 1');
  e.stopImmediatePropagation();
});

target.addEventListener('test', () => {
  console.log('Handler 2'); // This won't be called
});

target.dispatchEvent(new Event('test'));
```

#### Removing Event Listeners

```javascript
const target = new EventTarget();

const handler = () => {
  console.log('Event fired');
};

target.addEventListener('custom', handler);
target.dispatchEvent(new Event('custom')); // Logs: "Event fired"

target.removeEventListener('custom', handler);
target.dispatchEvent(new Event('custom')); // No output
```

#### Event Target and Current Target

```javascript
const target = new EventTarget();

target.addEventListener('test', (e) => {
  console.log('target === currentTarget:', e.target === e.currentTarget); // true
  console.log('Event dispatched on:', e.target);
});

target.dispatchEvent(new Event('test'));
```

#### Custom Event Types

```javascript
const emitter = new EventTarget();

// Listen for custom events
emitter.addEventListener('data-received', (e) => {
  console.log('Data received event');
});

emitter.addEventListener('connection-error', (e) => {
  console.log('Connection error event');
});

// Dispatch custom events
emitter.dispatchEvent(new Event('data-received'));
emitter.dispatchEvent(new Event('connection-error'));
```

## Timers

jstime implements the [HTML Standard Timer APIs](https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#timers), allowing you to schedule code execution.

### Supported Functions

- `setTimeout(callback, delay, ...args)` - Execute a function after a specified delay
- `setInterval(callback, delay, ...args)` - Execute a function repeatedly at specified intervals
- `clearTimeout(id)` - Cancel a timeout created with setTimeout
- `clearInterval(id)` - Cancel an interval created with setInterval

### Examples

```javascript
// Execute after 1 second
setTimeout(() => {
  console.log('This runs after 1 second');
}, 1000);

// Execute every 500ms
const intervalId = setInterval(() => {
  console.log('This runs every 500ms');
}, 500);

// Stop the interval after 3 seconds
setTimeout(() => {
  clearInterval(intervalId);
  console.log('Interval stopped');
}, 3000);

// Pass arguments to callbacks
setTimeout((name, age) => {
  console.log(`Name: ${name}, Age: ${age}`);
}, 1000, 'Alice', 30);
```

## Fetch API

jstime implements the [WHATWG Fetch API](https://fetch.spec.whatwg.org/), providing a modern interface for making HTTP requests. For detailed documentation, see [FETCH_API.md](./FETCH_API.md).

### Supported APIs

- `fetch(url, options)` - Make HTTP requests
- `Headers` - Manage HTTP headers
- `Request` - Represent HTTP requests
- `Response` - Represent HTTP responses

### Supported HTTP Methods

- GET
- POST
- PUT
- DELETE
- HEAD
- PATCH

### Examples

```javascript
// Simple GET request
fetch('https://api.example.com/data')
  .then(response => response.json())
  .then(data => console.log(data))
  .catch(error => console.error('Error:', error));

// POST request with JSON body
fetch('https://api.example.com/users', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    name: 'Alice',
    email: 'alice@example.com'
  })
})
  .then(response => response.json())
  .then(data => console.log('Created:', data))
  .catch(error => console.error('Error:', error));

// Working with Headers
const headers = new Headers();
headers.append('Authorization', 'Bearer token123');
headers.set('Content-Type', 'application/json');

fetch('https://api.example.com/protected', {
  method: 'GET',
  headers: headers
})
  .then(response => response.json())
  .then(data => console.log(data));

// Check response status
fetch('https://api.example.com/data')
  .then(response => {
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return response.json();
  })
  .then(data => console.log(data));
```

## URL API

jstime implements the [WHATWG URL Standard](https://url.spec.whatwg.org/), providing tools for parsing and manipulating URLs.

### Supported APIs

- `URL` - Parse and construct URLs
- `URLSearchParams` - Work with URL query strings

### URL Properties

The `URL` class provides the following properties:

- `href` - The complete URL
- `origin` - The origin of the URL
- `protocol` - The protocol scheme (e.g., 'https:')
- `username` - The username
- `password` - The password
- `host` - The host with port (e.g., 'example.com:8080')
- `hostname` - The hostname (e.g., 'example.com')
- `port` - The port number
- `pathname` - The path (e.g., '/path/to/resource')
- `search` - The query string (e.g., '?key=value')
- `hash` - The fragment identifier (e.g., '#section')

### Examples

```javascript
// Parsing a URL
const url = new URL('https://user:pass@example.com:8080/path?query=value#hash');
console.log(url.protocol);  // 'https:'
console.log(url.hostname);  // 'example.com'
console.log(url.port);      // '8080'
console.log(url.pathname);  // '/path'
console.log(url.search);    // '?query=value'
console.log(url.hash);      // '#hash'

// Constructing a URL with a base
const relativeUrl = new URL('/api/users', 'https://example.com');
console.log(relativeUrl.href);  // 'https://example.com/api/users'

// Modifying URL components
url.pathname = '/new/path';
url.search = '?newQuery=newValue';
console.log(url.href);

// Working with URLSearchParams
const params = new URLSearchParams('foo=1&bar=2');
console.log(params.get('foo'));  // '1'
params.append('baz', '3');
params.set('foo', '10');
console.log(params.toString());  // 'foo=10&bar=2&baz=3'

// Iterating over parameters
for (const [key, value] of params) {
  console.log(`${key}: ${value}`);
}

// Using URLSearchParams with URL
const url2 = new URL('https://example.com/search');
url2.searchParams.append('q', 'javascript');
url2.searchParams.append('limit', '10');
console.log(url2.href);  // 'https://example.com/search?q=javascript&limit=10'
```

## Performance API

jstime implements the [High Resolution Time API](https://w3c.github.io/hr-time/), providing access to high-resolution timestamps.

### Supported APIs

- `performance.now()` - Get a high-resolution timestamp in milliseconds
- `performance.timeOrigin` - Get the time origin as a timestamp

### Examples

```javascript
// Measure execution time
const start = performance.now();

// Some operation
for (let i = 0; i < 1000000; i++) {
  // Do work
}

const end = performance.now();
console.log(`Operation took ${end - start} milliseconds`);

// Get the time origin
console.log('Time origin:', performance.timeOrigin);
```

## Microtask API

jstime implements the [HTML Standard Microtask Queuing](https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#microtask-queuing) API, allowing you to queue microtasks.

### Supported Functions

- `queueMicrotask(callback)` - Queue a microtask to be executed

### Examples

```javascript
// Queue a microtask
console.log('1');

queueMicrotask(() => {
  console.log('3');
});

console.log('2');

// Output:
// 1
// 2
// 3

// Microtasks run before the next task
setTimeout(() => {
  console.log('Timeout');
}, 0);

queueMicrotask(() => {
  console.log('Microtask');
});

// Output:
// Microtask
// Timeout

// Nested microtasks
queueMicrotask(() => {
  console.log('First microtask');
  queueMicrotask(() => {
    console.log('Nested microtask');
  });
});

queueMicrotask(() => {
  console.log('Second microtask');
});

// Output:
// First microtask
// Second microtask
// Nested microtask
```

## Structured Clone API

jstime implements the [HTML Standard Structured Clone Algorithm](https://html.spec.whatwg.org/multipage/structured-data.html#structured-cloning), allowing deep cloning of JavaScript values including complex types that JSON cannot handle.

### Supported Function

- `structuredClone(value)` - Creates a deep clone of a value

### Supported Types

The structured clone algorithm can clone:

- **Primitive types**: strings, numbers, booleans, null, undefined, BigInt
- **Objects**: Plain objects with deep cloning of nested structures
- **Arrays**: Including nested arrays and objects
- **Date objects**: Preserves the date and time
- **RegExp**: Regular expressions with flags
- **Map**: Map objects with all entries
- **Set**: Set objects with all values
- **ArrayBuffer**: Binary data buffers
- **Typed Arrays**: Uint8Array, Int32Array, Float64Array, etc.
- **Boolean, Number, String objects**: Wrapper objects
- **Circular references**: Objects that reference themselves

### Unsupported Types

The following types cannot be cloned and will throw an error:

- **Functions**: Regular functions and arrow functions
- **Symbols**: Symbol values
- **Error objects**: Error, TypeError, etc.
- **DOM nodes**: Not applicable in jstime
- **Host objects**: Objects provided by the host environment

### Examples

```javascript
// Clone a simple object
const obj = { a: 1, b: 'hello', c: true };
const cloned = structuredClone(obj);
console.log(cloned); // { a: 1, b: 'hello', c: true }
console.log(obj !== cloned); // true (different objects)

// Clone nested objects
const nested = {
  user: {
    name: 'Alice',
    preferences: {
      theme: 'dark',
      language: 'en'
    }
  }
};
const clonedNested = structuredClone(nested);
console.log(clonedNested.user.preferences.theme); // 'dark'
console.log(nested.user !== clonedNested.user); // true

// Clone arrays
const arr = [1, 2, { x: 3 }];
const clonedArr = structuredClone(arr);
console.log(clonedArr); // [1, 2, { x: 3 }]

// Clone Date objects
const date = new Date('2024-01-01');
const clonedDate = structuredClone(date);
console.log(clonedDate.toISOString()); // '2024-01-01T00:00:00.000Z'

// Clone Map
const map = new Map([['key1', 'value1'], ['key2', 'value2']]);
const clonedMap = structuredClone(map);
console.log(clonedMap.get('key1')); // 'value1'

// Clone Set
const set = new Set([1, 2, 3]);
const clonedSet = structuredClone(set);
console.log(clonedSet.has(2)); // true

// Handle circular references
const circular = { name: 'circular' };
circular.self = circular;
const clonedCircular = structuredClone(circular);
console.log(clonedCircular.self === clonedCircular); // true

// Clone complex nested structures
const complex = {
  num: 42,
  str: "hello",
  date: new Date(),
  arr: [1, 2, { nested: true }],
  map: new Map([["key", "value"]]),
  set: new Set([1, 2, 3]),
  regexp: /test/gi
};
const clonedComplex = structuredClone(complex);
console.log(clonedComplex.map.get("key")); // 'value'

// Error: Cannot clone functions
try {
  structuredClone(() => {});
} catch (e) {
  console.error('Cannot clone function');
}

// Error: Cannot clone symbols
try {
  structuredClone(Symbol('test'));
} catch (e) {
  console.error('Cannot clone symbol');
}
```

### Use Cases

Structured clone is useful for:

- **Deep copying objects**: Create independent copies of complex data structures
- **Message passing**: Clone data when sending messages between workers (when available)
- **State management**: Create snapshots of application state
- **Data persistence**: Clone objects before serialization
- **Testing**: Create test fixtures from original data without mutation

### Comparison with JSON

Unlike `JSON.parse(JSON.stringify(obj))`, structured clone:

- ✅ Preserves Date objects (not converted to strings)
- ✅ Handles Map and Set
- ✅ Handles ArrayBuffer and typed arrays
- ✅ Handles RegExp with flags
- ✅ Handles circular references
- ✅ Handles undefined values
- ✅ More efficient for complex structures

```javascript
// JSON method loses Date objects
const obj1 = { date: new Date() };
const jsonClone = JSON.parse(JSON.stringify(obj1));
console.log(typeof jsonClone.date); // 'string' ❌

// Structured clone preserves Date objects
const structuredClone1 = structuredClone(obj1);
console.log(structuredClone1.date instanceof Date); // true ✅

// JSON method fails with circular references
const circular = { name: 'test' };
circular.self = circular;
try {
  JSON.parse(JSON.stringify(circular)); // Throws error ❌
} catch (e) {
  console.error('JSON cannot handle circular references');
}

// Structured clone handles circular references
const clonedCircular = structuredClone(circular); // Works ✅
console.log(clonedCircular.self === clonedCircular); // true
```

## WebAssembly

jstime provides full support for [WebAssembly](https://webassembly.org/) through the V8 engine, allowing you to run high-performance compiled modules alongside JavaScript code.

### Supported APIs

- `WebAssembly.Module` - Compile WebAssembly bytecode
- `WebAssembly.Instance` - Instantiate WebAssembly modules
- `WebAssembly.Memory` - Manage WebAssembly linear memory
- `WebAssembly.Table` - Manage WebAssembly tables
- `WebAssembly.compile()` - Asynchronously compile a module
- `WebAssembly.instantiate()` - Asynchronously compile and instantiate a module
- `WebAssembly.validate()` - Validate WebAssembly bytecode
- `WebAssembly.CompileError` - Error thrown during compilation
- `WebAssembly.LinkError` - Error thrown during instantiation
- `WebAssembly.RuntimeError` - Error thrown during execution

### Examples

#### Basic Module Instantiation

```javascript
// Create a simple WebAssembly module that adds two numbers
const wasmCode = new Uint8Array([
  0x00, 0x61, 0x73, 0x6d, // WASM_BINARY_MAGIC
  0x01, 0x00, 0x00, 0x00, // WASM_BINARY_VERSION
  // Type section
  0x01, 0x07, 0x01,       // section code, section size, num types
  0x60, 0x02, 0x7f, 0x7f, // func type: (i32, i32) -> ...
  0x01, 0x7f,             // ... -> i32
  // Function section
  0x03, 0x02, 0x01, 0x00, // section code, section size, num functions, func 0 type
  // Export section
  0x07, 0x07, 0x01,       // section code, section size, num exports
  0x03, 0x61, 0x64, 0x64, // field_len, field_str "add"
  0x00, 0x00,             // export kind (func), export func index
  // Code section
  0x0a, 0x09, 0x01,       // section code, section size, num functions
  0x07, 0x00,             // body size, local decl count
  0x20, 0x00,             // local.get 0
  0x20, 0x01,             // local.get 1
  0x6a,                   // i32.add
  0x0b                    // end
]);

// Compile and instantiate the module
const wasmModule = new WebAssembly.Module(wasmCode);
const wasmInstance = new WebAssembly.Instance(wasmModule);

// Call the exported function
const result = wasmInstance.exports.add(5, 7);
console.log(result); // 12
```

#### Validating WebAssembly Code

```javascript
// Valid WebAssembly module header
const validWasm = new Uint8Array([
  0x00, 0x61, 0x73, 0x6d, // WASM_BINARY_MAGIC
  0x01, 0x00, 0x00, 0x00, // WASM_BINARY_VERSION
]);

console.log(WebAssembly.validate(validWasm)); // true

// Invalid WebAssembly bytecode
const invalidWasm = new Uint8Array([0x00, 0x01, 0x02, 0x03]);
console.log(WebAssembly.validate(invalidWasm)); // false
```

#### Using WebAssembly Memory

```javascript
// Create a WebAssembly memory with 1 page (64KB)
const memory = new WebAssembly.Memory({ initial: 1 });

// Access the underlying ArrayBuffer
const buffer = memory.buffer;
console.log(buffer.byteLength); // 65536

// Grow the memory by 2 pages
const oldSize = memory.grow(2);
console.log(oldSize); // 1
console.log(memory.buffer.byteLength); // 196608 (3 pages)
```

#### Using WebAssembly Tables

```javascript
// Create a table that can hold function references
const table = new WebAssembly.Table({ 
  initial: 2, 
  element: 'anyfunc' 
});

console.log(table.length); // 2

// Grow the table
table.grow(3);
console.log(table.length); // 5
```

#### Async Compilation

```javascript
// Compile WebAssembly asynchronously
const wasmCode = new Uint8Array([
  0x00, 0x61, 0x73, 0x6d,
  0x01, 0x00, 0x00, 0x00,
]);

WebAssembly.compile(wasmCode)
  .then(module => {
    console.log('Module compiled successfully');
    return WebAssembly.instantiate(module);
  })
  .then(instance => {
    console.log('Instance created');
  })
  .catch(error => {
    console.error('Error:', error);
  });
```

#### Async Instantiation

```javascript
// Compile and instantiate in one step
const wasmCode = new Uint8Array([
  0x00, 0x61, 0x73, 0x6d,
  0x01, 0x00, 0x00, 0x00,
]);

WebAssembly.instantiate(wasmCode)
  .then(result => {
    console.log('Module:', result.module);
    console.log('Instance:', result.instance);
  })
  .catch(error => {
    console.error('Error:', error);
  });
```

### Features

WebAssembly in jstime supports:

- ✅ **Full WebAssembly Core Specification**: All standard WebAssembly features
- ✅ **Synchronous APIs**: Direct module compilation and instantiation
- ✅ **Asynchronous APIs**: Promise-based compilation and instantiation
- ✅ **Memory Management**: Linear memory allocation and growth
- ✅ **Table Management**: Function reference tables
- ✅ **Import/Export**: Module imports and exports
- ✅ **Error Handling**: Proper error types for compilation and runtime errors
- ✅ **Validation**: Bytecode validation before compilation

### Use Cases

WebAssembly is ideal for:

- **Performance-critical code**: CPU-intensive operations like image processing, cryptography, or data compression
- **Porting existing code**: Running C, C++, Rust, or other compiled languages in JavaScript
- **Game engines**: High-performance game logic and physics
- **Scientific computing**: Complex mathematical calculations
- **Media processing**: Audio/video encoding and decoding

### Compiling to WebAssembly

You can compile code from various languages to WebAssembly:

**Rust:**
```bash
# Install the wasm32-unknown-unknown target
rustup target add wasm32-unknown-unknown

# Compile to WebAssembly
cargo build --target wasm32-unknown-unknown --release
```

**C/C++ (using Emscripten):**
```bash
# Compile C/C++ to WebAssembly
emcc mycode.c -o mycode.wasm
```

**AssemblyScript:**
```bash
# Compile TypeScript-like code to WebAssembly
asc module.ts -o module.wasm
```

Then you can load and run the compiled `.wasm` files in jstime.

## ES Modules

jstime supports ES modules, allowing you to organize your code using `import` and `export` statements.

### Features

- Standard `import` and `export` syntax
- Top-level `await` support
- Module resolution from the file system
- `import.meta.url` support for getting the current module's URL

### Examples

**math.js**
```javascript
export function add(a, b) {
  return a + b;
}

export function multiply(a, b) {
  return a * b;
}

export const PI = 3.14159;
```

**main.js**
```javascript
import { add, multiply, PI } from './math.js';

console.log(add(2, 3));        // 5
console.log(multiply(4, 5));   // 20
console.log(PI);               // 3.14159
```

**app.js (with top-level await)**
```javascript
// Top-level await is supported
const data = await fetch('https://api.example.com/data')
  .then(response => response.json());

console.log(data);

// You can use await at the top level
await new Promise(resolve => setTimeout(resolve, 1000));
console.log('1 second has passed');
```

**module-info.js (using import.meta.url)**
```javascript
// Get the current module's URL
console.log('Current module URL:', import.meta.url);
// Outputs: file:///path/to/module-info.js

// You can use it with the URL constructor
const moduleUrl = new URL(import.meta.url);
console.log('Protocol:', moduleUrl.protocol);  // 'file:'
console.log('Pathname:', moduleUrl.pathname);  // '/path/to/module-info.js'

// Resolve relative paths from the current module
const dataPath = new URL('./data.json', import.meta.url);
console.log('Data file URL:', dataPath.href);
```

### Running Modules

```bash
# Run a module
$ jstime main.js

# The runtime automatically handles module imports
```

## REPL

jstime provides an interactive REPL (Read-Eval-Print Loop) for experimenting with JavaScript code.

### Features

- Interactive JavaScript shell
- Command history (saved to `~/.jstime_repl_history`)
- Tab completion for globals, built-in objects, and properties
- Multi-line input support
- Access to all jstime APIs

### Starting the REPL

```bash
# Start the REPL
$ jstime

Welcome to jstime v<version>!

>>
```

### REPL Examples

```javascript
>> 2 + 2
4

>> const name = 'Alice'
Alice

>> console.log(`Hello, ${name}!`)
Hello, Alice!
undefined

>> setTimeout(() => console.log('Delayed'), 1000)
1
Delayed

>> fetch('https://api.github.com')
  .then(r => r.json())
  .then(d => console.log(d))
Promise { <pending> }
>> // Result appears after promise resolves
```

### Tab Completion

The REPL supports tab completion:

- Type `cons` and press Tab → suggests `console`
- Type `console.` and press Tab → shows console methods
- Completion works for JavaScript built-ins and jstime APIs

### Exiting the REPL

Press `Ctrl+C` or `Ctrl+D` to exit the REPL.

## Running Scripts

jstime can execute JavaScript files directly:

```bash
# Run a JavaScript file
$ jstime script.js

# Run a module
$ jstime module.mjs
```

## Limitations and Future Work

While jstime provides a solid foundation for JavaScript execution, there are some limitations:

- **No file system API**: Currently no built-in API for reading/writing files
- **No process API**: No access to environment variables or process information
- **Limited streaming**: Fetch API doesn't support streaming response bodies
- **No WebSocket support**: WebSocket API not yet implemented

Future enhancements being considered:

- **WASI (WebAssembly System Interface)**: Support for WASI to enable WebAssembly modules to access file system and other system APIs
- **Additional Web APIs**: More browser APIs as they become relevant
- **Node.js compatibility layer**: Compatibility APIs for running Node.js code

## Additional Resources

- [README.md](./README.md) - Getting started guide
- [FETCH_API.md](./FETCH_API.md) - Detailed Fetch API documentation
- [CONTRIBUTING.md](./CONTRIBUTING.md) - Contribution guidelines
- [GitHub Repository](https://github.com/jstime/jstime) - Source code and issues
