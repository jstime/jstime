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

The module caching feature is automatically enabled and transparent to users.
You can observe its benefits by:

1. Running an application with many modules
2. Creating multiple JSTime instances that import the same modules
3. Observing that file I/O is minimized after the first read

See `core/tests/test_module_cache.rs` for concrete examples of how the caching works.
