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
- [File System API](#file-system-api)
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

## File System API

jstime provides a comprehensive Node.js-compatible file system API through the `node:fs/promises` module. This provides promise-based access to essential file operations.

### Supported APIs

**Primary (Essential):**
- `readFile(path, options?)` - Read the entire contents of a file
- `writeFile(path, data, options?)` - Write data to a file
- `appendFile(path, data, options?)` - Append data to a file
- `readdir(path, options?)` - Read the contents of a directory
- `mkdir(path, options?)` - Create a directory
- `rmdir(path, options?)` - Remove a directory
- `unlink(path)` - Delete a file
- `rename(oldPath, newPath)` - Rename a file or directory
- `copyFile(src, dest, mode?)` - Copy a file
- `stat(path, options?)` - Get file statistics
- `access(path, mode?)` - Test file accessibility
- `constants` - File system constants (F_OK, R_OK, W_OK, X_OK)

**Secondary (Additional):**
- `rm(path, options?)` - Remove files and directories (modern alternative)
- `truncate(path, len?)` - Truncate a file to a specified length
- `realpath(path, options?)` - Resolve path to an absolute path
- `chmod(path, mode)` - Change file permissions (Unix-like systems)
- `mkdtemp(prefix, options?)` - Create a unique temporary directory
- `readlink(path, options?)` - Read the target of a symbolic link
- `symlink(target, path, type?)` - Create a symbolic link
- `lstat(path, options?)` - Get file statistics without following symlinks
- `chown(path, uid, gid)` - Change file ownership (Unix-like systems)
- `utimes(path, atime, mtime)` - Change file access and modification times

### Usage

```javascript
import { readFile, writeFile, appendFile, mkdir, rm, stat, mkdtemp, symlink } from 'node:fs/promises';
// or
import * as fs from 'node:fs/promises';
```

### Reading Files

#### Read file as text

```javascript
import { readFile } from 'node:fs/promises';

// Simple string encoding
const text = await readFile('./README.md', 'utf-8');
console.log(text);

// Using options object
const content = await readFile('./file.txt', { encoding: 'utf-8' });
console.log(content);
```

#### Read file as buffer

```javascript
import { readFile } from 'node:fs/promises';

// Returns Uint8Array when no encoding is specified
const buffer = await readFile('./image.png');
console.log(buffer instanceof Uint8Array); // true
console.log(buffer.length); // file size in bytes
```

### Writing Files

```javascript
import { writeFile } from 'node:fs/promises';

// Write text
await writeFile('./output.txt', 'Hello, World!', 'utf-8');

// Write buffer
const buffer = new Uint8Array([72, 101, 108, 108, 111]);
await writeFile('./output.bin', buffer);
```

### Appending to Files

```javascript
import { appendFile } from 'node:fs/promises';

// Append text to a file
await appendFile('./log.txt', 'New log entry\n', 'utf-8');

// Append buffer
const buffer = new Uint8Array([72, 101, 108, 108, 111]);
await appendFile('./data.bin', buffer);

// Creates file if it doesn't exist
await appendFile('./new-file.txt', 'First line\n');
```

### Directory Operations

#### Creating directories

```javascript
import { mkdir } from 'node:fs/promises';

// Create single directory
await mkdir('./new-dir');

// Create nested directories (recursive)
await mkdir('./path/to/nested/dir', { recursive: true });
```

#### Listing directories

```javascript
import { readdir } from 'node:fs/promises';

// List directory contents
const files = await readdir('./src');
console.log('Files:', files); // Array of file/directory names

// Process files
for (const file of files) {
  console.log(file);
}
```

#### Removing directories

```javascript
import { rmdir } from 'node:fs/promises';

// Remove empty directory
await rmdir('./empty-dir');

// Remove directory and all contents (recursive)
await rmdir('./dir-with-files', { recursive: true });
```

### File Operations

#### Deleting files

```javascript
import { unlink, rm } from 'node:fs/promises';

// Delete a file with unlink
await unlink('./unwanted-file.txt');

// Or use modern rm() - works for files and directories
await rm('./unwanted-file.txt');

// Remove directory and all contents
await rm('./directory', { recursive: true });
```

#### Renaming files

```javascript
import { rename } from 'node:fs/promises';

await rename('./old-name.txt', './new-name.txt');
```

#### Copying files

```javascript
import { copyFile } from 'node:fs/promises';

await copyFile('./source.txt', './destination.txt');
```

#### Truncating files

```javascript
import { truncate } from 'node:fs/promises';

// Truncate file to 100 bytes
await truncate('./file.txt', 100);

// Truncate file to 0 bytes (empty the file)
await truncate('./file.txt');
```

### File Information

#### Getting file statistics

```javascript
import { stat } from 'node:fs/promises';

const stats = await stat('./file.txt');
console.log('Size:', stats.size);
console.log('Is file:', stats.isFile);
console.log('Is directory:', stats.isDirectory);
console.log('Is symlink:', stats.isSymbolicLink);
console.log('Modified time (ms):', stats.mtimeMs);
```

#### Resolving absolute paths

```javascript
import { realpath } from 'node:fs/promises';

// Resolve relative path to absolute path
const absolutePath = await realpath('./some/relative/path.txt');
console.log('Absolute path:', absolutePath);
```

#### Changing file permissions

```javascript
import { chmod } from 'node:fs/promises';

// Set file to read/write for owner, read-only for others
await chmod('./file.txt', 0o644);

// Set file to executable for owner
await chmod('./script.sh', 0o755);
```

**Note:** `chmod()` is only available on Unix-like systems (Linux, macOS).

#### Working with symbolic links

```javascript
import { symlink, readlink, lstat } from 'node:fs/promises';

// Create a symbolic link
await symlink('./target.txt', './link.txt');

// Read the link target
const target = await readlink('./link.txt');
console.log('Link points to:', target);

// Get stats without following the link
const stats = await lstat('./link.txt');
console.log('Is symlink:', stats.isSymbolicLink); // true
```

#### Creating temporary directories

```javascript
import { mkdtemp, writeFile, rmdir } from 'node:fs/promises';

// Create a unique temporary directory
const tmpDir = await mkdtemp('/tmp/myapp-');
console.log('Temp dir:', tmpDir); // e.g., /tmp/myapp-4a5b6c

// Use the directory
await writeFile(`${tmpDir}/data.txt`, 'temporary data');

// Clean up
await rmdir(tmpDir, { recursive: true });
```

#### Changing file ownership

```javascript
import { chown } from 'node:fs/promises';

// Change file ownership (Unix-like systems, requires permissions)
await chown('./file.txt', 1000, 1000);
```

**Note:** `chown()` is only available on Unix-like systems and typically requires root privileges.

#### Changing file timestamps

```javascript
import { utimes } from 'node:fs/promises';

// Set access and modification times
const now = Date.now();
const yesterday = now - 86400000; // 24 hours ago

await utimes('./file.txt', yesterday, yesterday);

// Or use Date objects
await utimes('./file.txt', new Date(), new Date());
```

#### Testing file accessibility

```javascript
import { access, constants } from 'node:fs/promises';

// Check if file exists
try {
  await access('./file.txt', constants.F_OK);
  console.log('File exists');
} catch (e) {
  console.log('File does not exist');
}

// Constants available
console.log(constants.F_OK); // 0 - File exists
console.log(constants.R_OK); // 4 - File is readable
console.log(constants.W_OK); // 2 - File is writable
console.log(constants.X_OK); // 1 - File is executable
```

### Error Handling

All file system operations can throw errors if the file or directory doesn't exist, or if there are permission issues:

```javascript
import { readFile, writeFile, mkdir } from 'node:fs/promises';

try {
  const data = await readFile('./nonexistent.txt', 'utf-8');
} catch (error) {
  console.error('Failed to read file:', error.message);
}

try {
  await writeFile('/root/protected.txt', 'data');
} catch (error) {
  console.error('Permission denied:', error.message);
}
```

### API Reference

#### `readFile(path, options?)`

Reads the entire contents of a file.

**Parameters:**
- `path` (string | Buffer | URL): The path to the file
- `options` (object | string, optional):
  - `encoding` (string): If specified, returns a string. Defaults to null (returns Buffer)
  - `flag` (string): File system flag. Defaults to 'r'

**Returns:** Promise<string | Uint8Array>

**Supported encodings:** 'utf-8', 'utf8'

#### `writeFile(path, data, options?)`

Writes data to a file, replacing the file if it already exists.

**Parameters:**
- `path` (string | Buffer | URL): The path to the file
- `data` (string | Uint8Array): Data to write
- `options` (object | string, optional):
  - `encoding` (string): Character encoding. Defaults to 'utf8' for strings
  - `flag` (string): File system flag. Defaults to 'w'

**Returns:** Promise<void>

#### `appendFile(path, data, options?)`

Appends data to a file, creating the file if it doesn't exist.

**Parameters:**
- `path` (string | Buffer | URL): The path to the file
- `data` (string | Uint8Array): Data to append
- `options` (object | string, optional):
  - `encoding` (string): Character encoding. Defaults to 'utf8' for strings
  - `flag` (string): File system flag. Defaults to 'a'

**Returns:** Promise<void>

#### `readdir(path, options?)`

Reads the contents of a directory.

**Parameters:**
- `path` (string | Buffer | URL): The path to the directory
- `options` (object | string, optional):
  - `encoding` (string): Character encoding for file names. Defaults to 'utf8'
  - `withFileTypes` (boolean): Not yet supported. Defaults to false

**Returns:** Promise<string[]>

Returns an array of filenames in the directory (excluding '.' and '..').

#### `mkdir(path, options?)`

Creates a directory.

**Parameters:**
- `path` (string | Buffer | URL): The path to create
- `options` (object, optional):
  - `recursive` (boolean): Create parent directories if needed. Defaults to false

**Returns:** Promise<void>

#### `rmdir(path, options?)`

Removes a directory.

**Parameters:**
- `path` (string | Buffer | URL): The path to remove
- `options` (object, optional):
  - `recursive` (boolean): Remove directory and all contents. Defaults to false

**Returns:** Promise<void>

#### `unlink(path)`

Deletes a file.

**Parameters:**
- `path` (string | Buffer | URL): The path to the file

**Returns:** Promise<void>

#### `rename(oldPath, newPath)`

Renames a file or directory.

**Parameters:**
- `oldPath` (string | Buffer | URL): The old path
- `newPath` (string | Buffer | URL): The new path

**Returns:** Promise<void>

#### `copyFile(src, dest, mode?)`

Copies a file.

**Parameters:**
- `src` (string | Buffer | URL): Source path
- `dest` (string | Buffer | URL): Destination path
- `mode` (number, optional): Copy mode flags

**Returns:** Promise<void>

#### `stat(path, options?)`

Gets file statistics.

**Parameters:**
- `path` (string | Buffer | URL): The path to stat
- `options` (object, optional): Options

**Returns:** Promise<Stats>

Returns a Stats object with properties:
- `isFile` (boolean): True if the path is a file
- `isDirectory` (boolean): True if the path is a directory
- `isSymbolicLink` (boolean): True if the path is a symbolic link
- `size` (number): File size in bytes
- `mtimeMs` (number): Last modified time in milliseconds since Unix epoch

#### `access(path, mode?)`

Tests file accessibility.

**Parameters:**
- `path` (string | Buffer | URL): The path to test
- `mode` (number, optional): Accessibility mode to check

**Returns:** Promise<void>

Throws an error if the file is not accessible.

#### `rm(path, options?)`

Removes files and directories (modern alternative to `unlink`/`rmdir`).

**Parameters:**
- `path` (string | Buffer | URL): The path to remove
- `options` (object, optional):
  - `recursive` (boolean): Remove directory and all contents. Defaults to false

**Returns:** Promise<void>

#### `truncate(path, len?)`

Truncates a file to a specified length.

**Parameters:**
- `path` (string | Buffer | URL): The path to the file
- `len` (number, optional): Target length in bytes. Defaults to 0

**Returns:** Promise<void>

#### `realpath(path, options?)`

Resolves a path to an absolute path, resolving symbolic links.

**Parameters:**
- `path` (string | Buffer | URL): The path to resolve
- `options` (object, optional): Options

**Returns:** Promise<string>

Returns the resolved absolute path.

#### `chmod(path, mode)`

Changes file permissions (Unix-like systems only).

**Parameters:**
- `path` (string | Buffer | URL): The path to the file
- `mode` (number): File mode (permissions) as octal number (e.g., 0o644)

**Returns:** Promise<void>

**Note:** Not supported on Windows. Will throw an error on non-Unix platforms.

#### `mkdtemp(prefix, options?)`

Creates a unique temporary directory.

**Parameters:**
- `prefix` (string): Directory name prefix
- `options` (object | string, optional):
  - `encoding` (string): Character encoding. Defaults to 'utf8'

**Returns:** Promise<string>

Returns the path to the created temporary directory.

#### `readlink(path, options?)`

Reads the target of a symbolic link.

**Parameters:**
- `path` (string | Buffer | URL): Path to the symbolic link
- `options` (object | string, optional):
  - `encoding` (string): Character encoding. Defaults to 'utf8'

**Returns:** Promise<string>

Returns the target path that the symbolic link points to.

#### `symlink(target, path, type?)`

Creates a symbolic link.

**Parameters:**
- `target` (string | Buffer | URL): Target path to link to
- `path` (string | Buffer | URL): Path of the symbolic link to create
- `type` (string, optional): Type of symlink ('file', 'dir', 'junction') - Windows only

**Returns:** Promise<void>

**Note:** On Windows, requires administrator privileges or Developer Mode.

#### `lstat(path, options?)`

Gets file statistics without following symbolic links.

**Parameters:**
- `path` (string | Buffer | URL): The path to stat
- `options` (object, optional): Options

**Returns:** Promise<Stats>

Returns a Stats object. Unlike `stat()`, if the path is a symbolic link, the stats are for the link itself, not the target.

#### `chown(path, uid, gid)`

Changes file ownership (Unix-like systems only).

**Parameters:**
- `path` (string | Buffer | URL): The path to the file
- `uid` (number): User ID
- `gid` (number): Group ID

**Returns:** Promise<void>

**Note:** Only supported on Unix-like systems. Requires appropriate permissions.

#### `utimes(path, atime, mtime)`

Changes file access and modification times.

**Parameters:**
- `path` (string | Buffer | URL): The path to the file
- `atime` (number | Date): Access time (milliseconds since epoch or Date object)
- `mtime` (number | Date): Modification time (milliseconds since epoch or Date object)

**Returns:** Promise<void>

#### `constants`

File system constants for use with `access()`:

- `F_OK` (0): File exists
- `R_OK` (4): File is readable  
- `W_OK` (2): File is writable
- `X_OK` (1): File is executable

### Example: Complete File Processing

```javascript
import { 
  readFile, 
  writeFile, 
  appendFile,
  readdir, 
  mkdir, 
  stat, 
  copyFile 
} from 'node:fs/promises';

// Create output directory
await mkdir('./output', { recursive: true });

// Create a summary log file
await writeFile('./output/summary.txt', 'Processing Summary\n==================\n\n');

// Read all JavaScript files in a directory
const files = await readdir('./src');
const jsFiles = files.filter(f => f.endsWith('.js'));

console.log(`Found ${jsFiles.length} JavaScript files`);

// Process each file
for (const file of jsFiles) {
  const inputPath = `./src/${file}`;
  const outputPath = `./output/${file}`;
  
  // Get file stats
  const stats = await stat(inputPath);
  console.log(`${file}: ${stats.size} bytes`);
  
  // Append to summary log
  await appendFile('./output/summary.txt', `${file}: ${stats.size} bytes\n`);
  
  // Read and transform content
  const content = await readFile(inputPath, 'utf-8');
  const transformed = content.toUpperCase();
  
  // Write to output
  await writeFile(outputPath, transformed, 'utf-8');
  console.log(`Processed ${file}`);
}

console.log('Processing complete!');
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
