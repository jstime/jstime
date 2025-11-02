// Performance benchmark: WebAssembly API
// Measures WebAssembly module compilation and instantiation

const ITERATIONS = 1000;

// Simple WASM module that exports an add function
// (module (func $add (param $a i32) (param $b i32) (result i32) local.get $a local.get $b i32.add) (export "add" (func $add)))
const wasmCode = new Uint8Array([
  0x00, 0x61, 0x73, 0x6d, // WASM_BINARY_MAGIC
  0x01, 0x00, 0x00, 0x00, // WASM_BINARY_VERSION
  0x01, 0x07, 0x01, 0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f, // Type section
  0x03, 0x02, 0x01, 0x00, // Function section
  0x07, 0x07, 0x01, 0x03, 0x61, 0x64, 0x64, 0x00, 0x00, // Export section
  0x0a, 0x09, 0x01, 0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b // Code section
]);

// Empty WASM module for basic tests
const emptyWasmCode = new Uint8Array([
  0x00, 0x61, 0x73, 0x6d, // WASM_BINARY_MAGIC
  0x01, 0x00, 0x00, 0x00, // WASM_BINARY_VERSION
]);

const results = [];
let totalElapsed = 0;

// Test 1: Module validation
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  WebAssembly.validate(wasmCode);
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'validate',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: Module compilation (synchronous)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  new WebAssembly.Module(wasmCode);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'module_sync',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: Module instantiation
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const module = new WebAssembly.Module(wasmCode);
  new WebAssembly.Instance(module);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'instantiate_sync',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: Memory creation
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  new WebAssembly.Memory({ initial: 1 });
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'memory_creation',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 5: Table creation
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  new WebAssembly.Table({ initial: 1, element: 'anyfunc' });
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'table_creation',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'webassembly',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
