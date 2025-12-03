// Compliance test for WebAssembly API
// Tests WebAssembly object, Module, Instance, Memory, and Table

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

// Test WebAssembly object exists
test('WebAssembly exists', () => {
  if (typeof WebAssembly !== 'object') throw new Error('WebAssembly is not an object');
});

// Test WebAssembly.validate exists and works
test('WebAssembly.validate exists', () => {
  if (typeof WebAssembly.validate !== 'function') throw new Error('validate is not a function');
});

// Test WebAssembly.validate returns true for valid module
test('WebAssembly.validate returns true', () => {
  const isValid = WebAssembly.validate(wasmCode);
  if (isValid !== true) throw new Error(`expected true, got ${isValid}`);
});

// Test WebAssembly.validate returns false for invalid module
test('WebAssembly.validate returns false for invalid', () => {
  const invalidCode = new Uint8Array([0x00, 0x61, 0x73, 0x6d, 0xFF, 0xFF, 0xFF, 0xFF]);
  const isValid = WebAssembly.validate(invalidCode);
  if (isValid !== false) throw new Error(`expected false, got ${isValid}`);
});

// Test WebAssembly.Module exists
test('WebAssembly.Module exists', () => {
  if (typeof WebAssembly.Module !== 'function') throw new Error('Module is not a function');
});

// Test WebAssembly.Module can compile
test('WebAssembly.Module can compile', () => {
  const module = new WebAssembly.Module(wasmCode);
  if (!(module instanceof WebAssembly.Module)) throw new Error('not a Module instance');
});

// Test WebAssembly.Module.exports
test('WebAssembly.Module.exports', () => {
  const module = new WebAssembly.Module(wasmCode);
  const exports = WebAssembly.Module.exports(module);
  if (!Array.isArray(exports)) throw new Error('exports is not an array');
  if (exports.length !== 1) throw new Error(`expected 1 export, got ${exports.length}`);
  if (exports[0].name !== 'add') throw new Error(`expected 'add', got '${exports[0].name}'`);
});

// Test WebAssembly.Instance exists
test('WebAssembly.Instance exists', () => {
  if (typeof WebAssembly.Instance !== 'function') throw new Error('Instance is not a function');
});

// Test WebAssembly.Instance can be created
test('WebAssembly.Instance can be created', () => {
  const module = new WebAssembly.Module(wasmCode);
  const instance = new WebAssembly.Instance(module);
  if (!(instance instanceof WebAssembly.Instance)) throw new Error('not an Instance');
});

// Test WebAssembly.Instance exports
test('WebAssembly.Instance.exports', () => {
  const module = new WebAssembly.Module(wasmCode);
  const instance = new WebAssembly.Instance(module);
  if (typeof instance.exports !== 'object') throw new Error('exports is not an object');
  if (typeof instance.exports.add !== 'function') throw new Error('add is not a function');
});

// Test exported function works correctly
test('WASM exported function works', () => {
  const module = new WebAssembly.Module(wasmCode);
  const instance = new WebAssembly.Instance(module);
  const result = instance.exports.add(2, 3);
  if (result !== 5) throw new Error(`expected 5, got ${result}`);
});

// Test WebAssembly.Memory exists
test('WebAssembly.Memory exists', () => {
  if (typeof WebAssembly.Memory !== 'function') throw new Error('Memory is not a function');
});

// Test WebAssembly.Memory can be created
test('WebAssembly.Memory can be created', () => {
  const memory = new WebAssembly.Memory({ initial: 1 });
  if (!(memory instanceof WebAssembly.Memory)) throw new Error('not a Memory instance');
});

// Test WebAssembly.Memory has buffer
test('WebAssembly.Memory.buffer', () => {
  const memory = new WebAssembly.Memory({ initial: 1 });
  if (!(memory.buffer instanceof ArrayBuffer)) throw new Error('buffer is not ArrayBuffer');
  // 1 page = 64KB
  if (memory.buffer.byteLength !== 65536) {
    throw new Error(`expected 65536 bytes, got ${memory.buffer.byteLength}`);
  }
});

// Test WebAssembly.Memory.grow
test('WebAssembly.Memory.grow', () => {
  const memory = new WebAssembly.Memory({ initial: 1, maximum: 2 });
  const oldPages = memory.grow(1);
  if (oldPages !== 1) throw new Error(`expected 1, got ${oldPages}`);
  if (memory.buffer.byteLength !== 131072) {
    throw new Error(`expected 131072 bytes, got ${memory.buffer.byteLength}`);
  }
});

// Test WebAssembly.Table exists
test('WebAssembly.Table exists', () => {
  if (typeof WebAssembly.Table !== 'function') throw new Error('Table is not a function');
});

// Test WebAssembly.Table can be created
test('WebAssembly.Table can be created', () => {
  const table = new WebAssembly.Table({ initial: 1, element: 'anyfunc' });
  if (!(table instanceof WebAssembly.Table)) throw new Error('not a Table instance');
});

// Test WebAssembly.Table.length
test('WebAssembly.Table.length', () => {
  const table = new WebAssembly.Table({ initial: 2, element: 'anyfunc' });
  if (table.length !== 2) throw new Error(`expected 2, got ${table.length}`);
});

// Test WebAssembly.Table.get
test('WebAssembly.Table.get', () => {
  const table = new WebAssembly.Table({ initial: 1, element: 'anyfunc' });
  const value = table.get(0);
  if (value !== null) throw new Error(`expected null, got ${value}`);
});

// Test WebAssembly.compile exists (async)
test('WebAssembly.compile exists', () => {
  if (typeof WebAssembly.compile !== 'function') throw new Error('compile is not a function');
});

// Test WebAssembly.instantiate exists (async)
test('WebAssembly.instantiate exists', () => {
  if (typeof WebAssembly.instantiate !== 'function') throw new Error('instantiate is not a function');
});

// Report results
console.log(`WebAssembly API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
