# Module Caching Example

This example demonstrates jstime's module caching feature.

## Features

jstime implements a two-level caching strategy:

1. **Per-Isolate Module Cache**: Each JSTime instance caches compiled modules
2. **Global Source Code Cache**: Source code is cached in memory across all instances

## Benefits

- **Startup Performance**: Eliminates repeated file I/O for commonly imported modules
- **Memory Efficiency**: Source code is shared across multiple JSTime instances
- **Scalability**: Better performance for applications with many modules

## Example

Run the example:
```bash
cargo run --example module-cache-demo
```

This will create multiple JSTime instances and demonstrate how the source code cache
improves performance by avoiding repeated file reads.
