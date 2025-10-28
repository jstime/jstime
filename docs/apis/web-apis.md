# Web APIs

This document describes the web-standard APIs implemented in jstime, following WHATWG and W3C specifications.

## Table of Contents

- [Console API](#console-api)
- [Event and EventTarget](#event-and-eventtarget)
- [Timers](#timers)
- [Fetch API](#fetch-api)
- [Streams API](#streams-api)
- [URL API](#url-api)
- [Performance API](#performance-api)
- [Microtask API](#microtask-api)
- [Structured Clone API](#structured-clone-api)
- [Base64 Encoding](#base64-encoding)

## Console API

jstime implements the [WHATWG Console Standard](https://console.spec.whatwg.org/#console-namespace), providing familiar logging methods for debugging and output.

**📁 Example:** See [examples/console-demo.js](../../examples/console-demo.js) for a complete demonstration.

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

**📁 Example:** See [examples/events-demo.js](../../examples/events-demo.js) for a complete demonstration.

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

**📁 Example:** See [examples/timers-demo.js](../../examples/timers-demo.js) for a complete demonstration.

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

jstime implements the [WHATWG Fetch API](https://fetch.spec.whatwg.org/), providing a modern interface for making HTTP requests.

**📁 Example:** See [examples/fetch-demo.mjs](../../examples/fetch-demo.mjs) for a complete demonstration.

### Supported APIs

- `fetch(url, options)` - Make HTTP requests
- `Headers` - Manage HTTP headers
- `Request` - Represent HTTP requests
- `Response` - Represent HTTP responses

### Headers

The `Headers` class provides methods to work with HTTP headers:

```javascript
const headers = new Headers();
headers.append('Content-Type', 'application/json');
headers.set('Authorization', 'Bearer token');
headers.get('content-type'); // 'application/json' (case-insensitive)
headers.has('authorization'); // true
headers.delete('authorization');

// Iterate over headers
for (const [key, value] of headers) {
  console.log(key, value);
}
```

### Request

The `Request` class represents an HTTP request:

```javascript
const request = new Request('https://example.com/api', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({ data: 'value' })
});

console.log(request.url);    // 'https://example.com/api'
console.log(request.method); // 'POST'
```

### Response

The `Response` class represents an HTTP response:

```javascript
const response = new Response('{"key":"value"}', {
  status: 200,
  statusText: 'OK',
  headers: { 'Content-Type': 'application/json' }
});

console.log(response.ok);     // true (status 200-299)
console.log(response.status); // 200

// Parse response body
const text = await response.text();
const json = await response.json();
```

### Supported HTTP Methods

- GET
- POST
- PUT
- DELETE
- HEAD
- PATCH

### Response Body Methods

- `text()`: Returns the body as a text string
- `json()`: Parses the body as JSON

### Examples

#### Simple GET Request

```javascript
fetch('https://api.example.com/data')
  .then(response => response.json())
  .then(data => console.log(data))
  .catch(error => console.error('Error:', error));
```

#### POST Request with JSON Body

```javascript
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
```

#### Working with Headers

```javascript
const headers = new Headers();
headers.append('Authorization', 'Bearer token123');
headers.set('Content-Type', 'application/json');

fetch('https://api.example.com/protected', {
  method: 'GET',
  headers: headers
})
  .then(response => response.json())
  .then(data => console.log(data));
```

#### Check Response Status

```javascript
fetch('https://api.example.com/data')
  .then(response => {
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return response.json();
  })
  .then(data => console.log(data));
```

### Implementation Architecture

The Fetch API implementation consists of three parts:

1. **JavaScript Layer** (`core/src/builtins/whatwg/fetch.js`)
   - Implements the Headers, Request, Response classes
   - Provides the fetch() function
   - Handles promise creation and data marshalling

2. **Rust Bindings** (`core/src/builtins/whatwg/fetch_impl.rs`)
   - `fetch_send()`: Native binding that creates a promise
   - Queues fetch requests for execution

3. **Event Loop Integration** (`core/src/event_loop.rs`)
   - Processes pending fetch requests
   - Uses `ureq` HTTP client to execute requests
   - Resolves promises with response data

### Promise Handling

Fetch requests are asynchronous and return promises:

1. `fetch()` is called from JavaScript
2. A Promise is created using V8's PromiseResolver
3. The fetch request is queued in `pending_fetches`
4. The event loop processes the request
5. The HTTP request is executed using `ureq`
6. The promise is resolved with the response data

### Limitations

- No streaming support (entire response body is loaded into memory)
- Limited to basic fetch options (no advanced features like credentials, cache control)
- Network access depends on the environment

## Streams API

jstime implements the [WHATWG Streams API](https://streams.spec.whatwg.org/), providing a standard way to handle streaming data. Streams are essential for processing large files, handling network responses, and transforming data incrementally.

**📁 Example:** See [examples/streams-demo.js](../../examples/streams-demo.js) for a complete demonstration.

### Supported APIs

- `ReadableStream` - For reading data chunks sequentially
- `WritableStream` - For writing data chunks sequentially
- `TransformStream` - For transforming data as it passes through
- `ReadableStreamDefaultReader` - Default reader for readable streams
- `WritableStreamDefaultWriter` - Default writer for writable streams

### Why Streams Matter

- **Memory Efficiency**: Process large files without loading everything into memory
- **Fetch Integration**: `Response.body` returns a ReadableStream for efficient data handling
- **Data Transformation**: Transform data incrementally as it flows through pipelines
- **Standard API**: Compatible with the WHATWG Streams specification

### ReadableStream

A ReadableStream represents a source of streaming data that you can read from.

```javascript
// Create a readable stream with data chunks
const readable = new ReadableStream({
  start(controller) {
    controller.enqueue("chunk1");
    controller.enqueue("chunk2");
    controller.close();
  }
});

// Read from the stream
const reader = readable.getReader();
reader.read().then(result => {
  console.log(result.value); // "chunk1"
  console.log(result.done);  // false
});
```

### WritableStream

A WritableStream represents a destination for streaming data that you can write to.

```javascript
// Create a writable stream
const writable = new WritableStream({
  write(chunk) {
    console.log("Writing:", chunk);
  },
  close() {
    console.log("Stream closed");
  }
});

// Write to the stream
const writer = writable.getWriter();
writer.write("Hello");
writer.write("World");
writer.close();
```

### TransformStream

A TransformStream consists of a readable and writable side, and can transform data as it passes through.

```javascript
// Create a transform stream to uppercase text
const transform = new TransformStream({
  transform(chunk, controller) {
    controller.enqueue(chunk.toUpperCase());
  }
});

const writer = transform.writable.getWriter();
const reader = transform.readable.getReader();

writer.write("hello");
writer.close();

reader.read().then(result => {
  console.log(result.value); // "HELLO"
});
```

### Integration with Fetch API

The Fetch API's `Response.body` property returns a ReadableStream, allowing you to process response data incrementally:

```javascript
const response = await fetch('https://api.example.com/data');
const reader = response.body.getReader();

// Read chunks as they arrive
while (true) {
  const {value, done} = await reader.read();
  if (done) break;
  console.log("Received chunk:", value);
}
```

### Stream Pipelines

You can chain streams together to create data processing pipelines:

```javascript
// Transform pipeline: source -> transform -> destination
const source = new ReadableStream({
  start(controller) {
    controller.enqueue("data");
    controller.close();
  }
});

const transform = new TransformStream({
  transform(chunk, controller) {
    controller.enqueue(`[${chunk}]`);
  }
});

const writer = transform.writable.getWriter();
const reader = transform.readable.getReader();

// Process data through the pipeline
writer.write("chunk1");
writer.write("chunk2");
writer.close();
```

## URL API

jstime implements the [WHATWG URL Standard](https://url.spec.whatwg.org/), providing tools for parsing and manipulating URLs.

**📁 Example:** See [examples/url-demo.js](../../examples/url-demo.js) for a complete demonstration.

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

**📁 Example:** See [examples/performance-demo.js](../../examples/performance-demo.js) for a complete demonstration.

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

**📁 Example:** See [examples/structured-clone-demo.mjs](../../examples/structured-clone-demo.mjs) for a complete demonstration.

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

## Base64 Encoding

jstime implements the [WHATWG HTML Standard base64 utilities](https://html.spec.whatwg.org/multipage/webappapis.html#atob), providing functions for encoding and decoding base64 strings.

### Supported Functions

- `btoa(data)` - Encode a string to base64
- `atob(data)` - Decode a base64 string

### Examples

```javascript
// Encode to base64
const encoded = btoa('Hello, World!');
console.log(encoded); // 'SGVsbG8sIFdvcmxkIQ=='

// Decode from base64
const decoded = atob('SGVsbG8sIFdvcmxkIQ==');
console.log(decoded); // 'Hello, World!'

// Works with binary data (Latin-1 encoded)
const binary = btoa('\x00\x01\x02\x03');
console.log(binary); // 'AAECAw=='

const decodedBinary = atob('AAECAw==');
console.log(decodedBinary); // '\x00\x01\x02\x03'
```

### Notes

- `btoa()` only accepts strings with characters in the Latin-1 range (0-255)
- For UTF-8 strings, use TextEncoder/TextDecoder in combination with base64
- These functions match the behavior of the browser APIs
