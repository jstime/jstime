# Module System

This document describes the module system support in jstime, including ES Modules and WebAssembly.

## Table of Contents

- [ES Modules](#es-modules)
- [JSON Modules](#json-modules)
- [WebAssembly](#webassembly)


jstime provides full support for [WebAssembly](https://webassembly.org/) through the V8 engine, allowing you to run high-performance compiled modules alongside JavaScript code.

**📁 Example:** See [examples/webassembly-demo.js](../../examples/webassembly-demo.js) for a complete demonstration.

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

**📁 Example:** See [examples/json-import-example.js](../../examples/json-import-example.js) for JSON import demonstration.

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

## JSON Modules

jstime supports importing JSON files as ES modules, following the [JSON modules proposal](https://github.com/tc39/proposal-json-modules). This allows you to import JSON data directly into your JavaScript code.

**📁 Example:** See [examples/json-import-example.js](../../examples/json-import-example.js) for a complete demonstration.

### Features

- Import JSON files using standard `import` syntax
- JSON data is parsed and available as the default export
- Type-safe: imported values are standard JavaScript objects/arrays
- Automatic file resolution with `.json` extension

### Examples

**data.json**
```json
{
  "name": "jstime",
  "version": "0.60.0",
  "features": ["ES Modules", "WebAssembly", "Fetch API"]
}
```

**app.js**
```javascript
// Import JSON data as the default export
import data from './data.json';

console.log(data.name);        // "jstime"
console.log(data.version);     // "0.60.0"
console.log(data.features[0]); // "ES Modules"
```

**config-example.js**
```javascript
// Import JSON configuration
import config from './config.json';

// Use the configuration data
console.log(`App: ${config.app.name}`);
console.log(`Environment: ${config.environment}`);
console.log(`API URL: ${config.api.url}`);

// JSON data is a regular JavaScript object
const features = config.features.map(f => f.toUpperCase());
console.log('Features:', features);
```

**array-example.js**
```javascript
// Import a JSON array
import users from './users.json';

// Work with the imported array
users.forEach(user => {
  console.log(`${user.name}: ${user.email}`);
});

// Filter and transform
const admins = users.filter(u => u.role === 'admin');
console.log(`Found ${admins.length} administrators`);
```

### Usage Notes

- JSON modules are read-only: the imported data is a constant
- The JSON file must be valid JSON (trailing commas are not allowed)
- JSON modules use the default export pattern
- The imported data is deeply frozen (immutable)

### Running JSON Module Examples

```bash
# Run a module that imports JSON
$ jstime app.js

# The runtime automatically handles JSON module imports
```

