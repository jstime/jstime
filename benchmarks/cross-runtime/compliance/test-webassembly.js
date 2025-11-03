// Compliance test for WebAssembly API
// Tests WebAssembly module compilation, validation, and instantiation

let passed = 0;
let failed = 0;

function test(name, fn) {
  try {
    fn();
    passed++;
  } catch (e) {
    console.error(`FAIL: ${name} - ${e.message}`);
    failed++;
  }
}

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

// Test WebAssembly object
test('WebAssembly object exists', () => {
  if (typeof WebAssembly !== 'object') throw new Error('WebAssembly is not an object');
});

// Test WebAssembly.validate
test('WebAssembly.validate exists', () => {
  if (typeof WebAssembly.validate !== 'function') throw new Error('WebAssembly.validate is not a function');
});

test('WebAssembly.validate works', () => {
  const result = WebAssembly.validate(wasmCode);
  if (result !== true) throw new Error('WebAssembly.validate returned false for valid module');
});

// Test WebAssembly.Module
test('WebAssembly.Module exists', () => {
  if (typeof WebAssembly.Module !== 'function') throw new Error('WebAssembly.Module is not a function');
});

test('WebAssembly.Module compilation', () => {
  const module = new WebAssembly.Module(wasmCode);
  if (!(module instanceof WebAssembly.Module)) throw new Error('Failed to create WebAssembly.Module');
});

// Test WebAssembly.Instance
test('WebAssembly.Instance exists', () => {
  if (typeof WebAssembly.Instance !== 'function') throw new Error('WebAssembly.Instance is not a function');
});

test('WebAssembly.Instance instantiation', () => {
  const module = new WebAssembly.Module(wasmCode);
  const instance = new WebAssembly.Instance(module);
  if (!(instance instanceof WebAssembly.Instance)) throw new Error('Failed to create WebAssembly.Instance');
});

test('WebAssembly function execution', () => {
  const module = new WebAssembly.Module(wasmCode);
  const instance = new WebAssembly.Instance(module);
  const result = instance.exports.add(5, 3);
  if (result !== 8) throw new Error(`Expected 5+3=8, got ${result}`);
});

// Test WebAssembly.Memory
test('WebAssembly.Memory exists', () => {
  if (typeof WebAssembly.Memory !== 'function') throw new Error('WebAssembly.Memory is not a function');
});

test('WebAssembly.Memory creation', () => {
  const memory = new WebAssembly.Memory({ initial: 1 });
  if (!(memory instanceof WebAssembly.Memory)) throw new Error('Failed to create WebAssembly.Memory');
  if (!(memory.buffer instanceof ArrayBuffer)) throw new Error('Memory.buffer is not an ArrayBuffer');
});

// Test WebAssembly.Table
test('WebAssembly.Table exists', () => {
  if (typeof WebAssembly.Table !== 'function') throw new Error('WebAssembly.Table is not a function');
});

test('WebAssembly.Table creation', () => {
  const table = new WebAssembly.Table({ initial: 1, element: 'anyfunc' });
  if (!(table instanceof WebAssembly.Table)) throw new Error('Failed to create WebAssembly.Table');
});

// Report results
console.log(`WebAssembly API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
