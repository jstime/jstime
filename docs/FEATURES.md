# jstime Features

jstime is a minimal and performant JavaScript runtime built on top of V8. This document provides an overview of the various features and APIs supported by jstime.

## Table of Contents

- [JavaScript Language Support](#javascript-language-support)
- [Console API](#console-api)
- [Timers](#timers)
- [Fetch API](#fetch-api)
- [URL API](#url-api)
- [Performance API](#performance-api)
- [Microtask API](#microtask-api)
- [Structured Clone API](#structured-clone-api)
- [Base64 Encoding](#base64-encoding)
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

- WASI (WebAssembly System Interface) support for system APIs
- Additional Web APIs
- Node.js compatibility layer

## Additional Resources

- [README.md](./README.md) - Getting started guide
- [FETCH_API.md](./FETCH_API.md) - Detailed Fetch API documentation
- [CONTRIBUTING.md](./CONTRIBUTING.md) - Contribution guidelines
- [GitHub Repository](https://github.com/jstime/jstime) - Source code and issues
