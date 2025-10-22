# jstime Architecture

This document provides an overview of the jstime architecture, explaining how the different components work together to provide a JavaScript runtime.

## Table of Contents

- [Overview](#overview)
- [High-Level Architecture](#high-level-architecture)
- [Core Components](#core-components)
- [Built-in APIs](#built-in-apis)
- [Module System](#module-system)
- [Event Loop](#event-loop)
- [V8 Integration](#v8-integration)
- [Memory Management](#memory-management)
- [Data Flow](#data-flow)

## Overview

jstime is a minimal JavaScript runtime built on top of the V8 JavaScript engine. It provides a focused set of APIs that enable JavaScript execution with support for modern features like ES modules, async/await, and standard web APIs.

### Design Philosophy

- **Minimal**: Only include essential APIs, avoiding feature bloat
- **Performant**: Optimize for execution speed and low memory usage
- **Standards-compliant**: Follow WHATWG and W3C specifications where applicable
- **Embeddable**: Designed to be embedded in Rust applications

### Technology Stack

- **Language**: Rust (2021 edition)
- **JavaScript Engine**: V8 (v140.2.0)
- **Build System**: Cargo
- **Testing**: Rust's built-in test framework

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        User Code                             │
│                    (JavaScript Files)                        │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ↓
┌─────────────────────────────────────────────────────────────┐
│                      jstime CLI                              │
│                     (cli/main.rs)                            │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  REPL  │  Script Execution  │  Module Loading        │   │
│  └──────────────────────────────────────────────────────┘   │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ↓
┌─────────────────────────────────────────────────────────────┐
│                   jstime_core Library                        │
│                    (core/src/lib.rs)                         │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                  JSTime Instance                      │   │
│  │  - V8 Isolate                                        │   │
│  │  - Global Context                                    │   │
│  │  - Isolate State                                     │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              Built-in APIs (builtins/)               │   │
│  │  console | timers | fetch | url | performance        │   │
│  │  base64 | events | structured_clone | fs | wasm     │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         Core Runtime Components                      │   │
│  │  Event Loop | Module System | Script Execution      │   │
│  └──────────────────────────────────────────────────────┘   │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ↓
┌─────────────────────────────────────────────────────────────┐
│                  V8 JavaScript Engine                        │
│  - JIT Compilation (TurboFan)                               │
│  - Garbage Collection                                       │
│  - ES Module System                                         │
│  - WebAssembly Support                                      │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### JSTime Instance (`lib.rs`)

The `JSTime` struct is the main entry point for embedding jstime in Rust applications.

**Responsibilities**:
- Initialize V8 engine
- Create and manage V8 isolate
- Set up global context with built-in APIs
- Execute JavaScript code
- Run the event loop

**Key Methods**:
- `new(options)`: Create a new JSTime instance
- `run_script(source, filename)`: Execute JavaScript code
- `import(filename)`: Load and execute an ES module
- `run_event_loop()`: Process pending async operations

**Lifecycle**:
```rust
// 1. Initialize V8
jstime::init(v8_flags);

// 2. Create instance
let mut jstime = JSTime::new(Options::default());

// 3. Execute code
jstime.run_script("console.log('Hello')", "script.js");

// 4. Event loop runs automatically
// 5. Instance dropped, resources cleaned up
```

### Isolate State (`isolate_state.rs`)

Manages per-isolate state using V8's isolate slots.

**Stored State**:
- Global context reference
- Module map for ES modules
- Event loop state

**Access Pattern**:
```rust
let state = IsolateState::get(&mut isolate);
let context = state.borrow().context();
```

### Script Execution (`script.rs`)

Handles JavaScript code compilation and execution.

**Features**:
- Script compilation with V8
- Error handling and reporting
- Source code management

### Module System (`module.rs`, `js_loading.rs`)

Implements ES module support following the ES6 specification.

**Components**:
- **Module Loader**: Resolves and loads modules from the file system
- **Module Graph**: Tracks dependencies and loading state
- **Import Meta Callback**: Provides `import.meta.url`

**Module Loading Flow**:
```
1. Parse import specifier
2. Resolve to absolute file path
3. Load file contents
4. Compile as V8 module
5. Link dependencies recursively
6. Evaluate module
7. Return exports
```

### Event Loop (`event_loop.rs`)

Manages asynchronous operations.

**Architecture**:
```
┌─────────────────────────────────────────────────────────┐
│                    Event Loop                           │
│  ┌────────────────────────────────────────────────┐    │
│  │           Task Queue                           │    │
│  │  - Timers (setTimeout, setInterval)            │    │
│  │  - Fetch Requests                              │    │
│  │  - File Operations                             │    │
│  └────────────────────────────────────────────────┘    │
│  ┌────────────────────────────────────────────────┐    │
│  │         Microtask Queue (V8)                   │    │
│  │  - Promise callbacks                           │    │
│  │  - queueMicrotask                              │    │
│  └────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

**Execution Model**:
1. Execute synchronous code
2. Process all microtasks (Promise callbacks)
3. Process next task from task queue
4. Repeat until queue is empty

**Timer Management**:
- Timers stored with execution time
- Sorted by next execution time
- Sleep until next timer or new task arrives

## Built-in APIs

Built-in APIs are implemented in the `core/src/builtins/` directory.

### Architecture Pattern

Each API follows a two-part structure:

**1. Rust Implementation (`*_impl.rs`)**:
- Exposes functions to V8
- Handles low-level operations
- Provides external references for V8

**2. JavaScript Polyfill (`*.js`)**:
- Wraps Rust functions
- Provides JavaScript-friendly interface
- Implements spec-compliant behavior

### Example: Console API

```
┌─────────────────────────────────────────────────────────┐
│              console.log("hello")                       │
└────────────────────┬────────────────────────────────────┘
                     │
                     ↓
┌─────────────────────────────────────────────────────────┐
│          console.js (JavaScript Polyfill)               │
│  - Format arguments                                     │
│  - Handle format specifiers                             │
│  - Call Rust implementation                             │
└────────────────────┬────────────────────────────────────┘
                     │
                     ↓
┌─────────────────────────────────────────────────────────┐
│         console_impl.rs (Rust Implementation)           │
│  - Convert V8 values to strings                         │
│  - Write to stdout/stderr                               │
│  - Return to JavaScript                                 │
└─────────────────────────────────────────────────────────┘
```

### API Categories

**Core APIs**:
- Console: Logging and debugging
- Timers: setTimeout, setInterval
- Performance: High-resolution timing

**Web APIs**:
- Fetch: HTTP client
- URL: URL parsing and manipulation
- Events: Event and EventTarget

**Data APIs**:
- Base64: Encoding and decoding
- Structured Clone: Deep object cloning

**System APIs**:
- File System: Node.js-compatible fs/promises API

**Advanced APIs**:
- WebAssembly: WASM module execution (via V8)

## Module System

### ES Modules

jstime implements the ES6 module specification:

**Features**:
- `import` and `export` statements
- Top-level `await`
- Dynamic `import()`
- `import.meta.url`

**Module Resolution**:
```
Import Specifier → Resolve Algorithm → Absolute Path → Load & Compile
```

**Resolution Rules**:
1. Relative imports: Resolve relative to importing file
2. Absolute paths: Use as-is
3. Bare specifiers: Not supported (Node.js-style resolution not implemented)

### JSON Modules

Support for importing JSON files:

```javascript
import data from './data.json';
```

**Implementation**:
- JSON files detected by `.json` extension
- Parsed and wrapped as ES module with default export
- Full JSON compatibility

## Event Loop

### Design

The event loop is implemented in Rust and integrates with V8's microtask queue.

**Task Types**:

1. **Macrotasks** (Managed by jstime):
   - Timers (setTimeout, setInterval)
   - Fetch requests
   - File system operations

2. **Microtasks** (Managed by V8):
   - Promise callbacks
   - queueMicrotask()

### Execution Order

```
1. Execute Script
   ↓
2. Process Microtasks (V8)
   ↓
3. Check for Ready Tasks
   ↓
4. Execute Next Task
   ↓
5. Go to step 2

Loop exits when no more tasks are pending
```

### Async Operations

Async operations follow this pattern:

```
JavaScript Call → Register Task → Continue Execution
                        ↓
                  Background Work
                        ↓
                Task Ready → Execute Callback → Resolve Promise
```

## V8 Integration

### Isolate

V8's isolate represents an independent instance of the V8 engine.

**jstime Usage**:
- One isolate per JSTime instance
- Isolate owns all V8 objects and contexts
- Thread-local state

### Context

A V8 context is an execution environment with its own global object.

**jstime Usage**:
- One global context per JSTime instance
- Built-ins registered on global object
- Context stored in isolate state

### Handles and Scopes

V8 uses handles for garbage-collected objects.

**Handle Types**:
- **Local Handle**: Valid only within a HandleScope
- **Global Handle**: Persists across scopes (must be explicitly freed)

**Scope Management**:
```rust
v8::scope!(let scope, &mut isolate);
let context = v8::Local::new(scope, global_context);
let mut scope = v8::ContextScope::new(scope, context_local);
// Work with V8 objects
// Handles automatically cleaned up when scope ends
```

### External References

Functions exposed to JavaScript must be registered as external references:

```rust
pub fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: my_function.map_fn_to(),
        },
    ]
}
```

This enables V8's snapshot feature (currently not used, but prepared for future optimization).

## Memory Management

### Rust Memory

- Managed by Rust's ownership system
- RAII pattern for resource cleanup
- No manual memory management needed

### V8 Memory

- Garbage collected by V8
- Handles automatically managed via scopes
- Global handles must be explicitly managed

### Interaction Points

**JavaScript → Rust**:
- Strings converted from V8 to Rust UTF-8
- Numbers converted to Rust primitives
- Objects accessed via V8 API

**Rust → JavaScript**:
- Rust strings converted to V8 strings
- Rust numbers converted to V8 numbers
- Complex types serialized or wrapped

## Data Flow

### Script Execution

```
User Code
   │
   ↓
[Parse & Compile] ← V8
   │
   ↓
[Execute] ← V8 Runtime
   │
   ├→ [Call Built-in] → Rust Code → Return Value
   │                        │
   │                        ↓
   │                   [Async Operation] → Event Loop
   │
   ↓
[Complete]
   │
   ↓
[Run Event Loop] → Process Tasks → Call Callbacks
   │
   ↓
[Finish]
```

### Module Loading

```
import statement
   │
   ↓
[Resolve Path] ← Module Loader
   │
   ↓
[Load File] ← File System
   │
   ↓
[Compile Module] ← V8
   │
   ↓
[Link Dependencies] ← Recursive
   │
   ↓
[Instantiate] ← V8
   │
   ↓
[Evaluate] ← V8
   │
   ↓
[Return Exports]
```

### Async Fetch Example

```
fetch(url)
   │
   ↓
[Create Request] ← JavaScript
   │
   ↓
[Call Rust fetch_impl] ← Binding
   │
   ↓
[Create Promise] ← V8
   │
   ↓
[Queue HTTP Request] ← Event Loop
   │
   └→ [Return Promise to JS]
   
[Background Thread] ← ureq
   │
   ↓
[HTTP Request/Response]
   │
   ↓
[Queue Callback] ← Event Loop
   │
   ↓
[Execute on Event Loop]
   │
   ↓
[Resolve Promise] ← V8
   │
   ↓
[Execute .then()] ← JavaScript
```

## Performance Considerations

### Optimizations

1. **V8 JIT Compilation**: TurboFan optimizes hot code paths
2. **String Handling**: Minimize conversions between Rust and V8
3. **Memory Pooling**: Reuse allocations where possible
4. **Lazy Initialization**: Initialize features only when used

### Bottlenecks

1. **String Conversions**: UTF-8 ↔ V8 string conversion overhead
2. **FFI Calls**: Crossing Rust/V8 boundary has cost
3. **Event Loop**: Waking and sleeping has overhead
4. **Module Loading**: File I/O and compilation time

### Measurement

Use the Performance API for measurements:

```javascript
const start = performance.now();
// ... operation ...
const end = performance.now();
console.log(`Took ${end - start}ms`);
```

## Future Architecture Considerations

### Potential Improvements

1. **Snapshot Support**: Pre-compile built-ins to V8 snapshot
2. **Worker Threads**: Multi-isolate support for parallelism
3. **Streaming APIs**: Support for streaming fetch responses
4. **Module Caching**: Cache compiled modules
5. **Native Modules**: Support for loading Rust modules from JS

### Extensibility

jstime is designed to be extended:

- Add new built-in APIs following existing patterns
- Embed jstime_core in other Rust applications
- Customize initialization and options
- Hook into event loop for custom tasks

## Debugging and Instrumentation

### Debugging Tools

- **Rust Debuggers**: lldb, gdb for Rust code
- **Console API**: For JavaScript debugging
- **V8 Flags**: Enable V8 tracing and profiling

### Instrumentation Points

- Script compilation and execution
- Module loading and linking
- Event loop task processing
- Built-in API calls
- Memory allocation and GC

## References

- [V8 Embedder's Guide](https://v8.dev/docs/embed)
- [Rust V8 Bindings](https://docs.rs/v8/)
- [ES6 Modules Spec](https://tc39.es/ecma262/#sec-modules)
- [WHATWG Standards](https://spec.whatwg.org/)
- [Event Loop Spec](https://html.spec.whatwg.org/multipage/webappapis.html#event-loops)

## Contributing

Understanding this architecture is the first step to contributing effectively:

1. **Start with tests**: Read tests to understand behavior
2. **Follow patterns**: Match existing code structure
3. **Read specs**: Implement according to standards
4. **Ask questions**: File issues or discussions for clarification

See [CONTRIBUTING.md](./CONTRIBUTING.md) for development information.
