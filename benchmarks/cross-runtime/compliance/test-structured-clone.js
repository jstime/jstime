// Compliance test for Structured Clone API
// Tests structuredClone for deep cloning of JavaScript values

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

// Test structuredClone exists
test('structuredClone exists', () => {
  if (typeof structuredClone !== 'function') throw new Error('structuredClone is not a function');
});

// Test cloning primitive values
test('structuredClone clones primitives', () => {
  if (structuredClone(42) !== 42) throw new Error('failed to clone number');
  if (structuredClone('hello') !== 'hello') throw new Error('failed to clone string');
  if (structuredClone(true) !== true) throw new Error('failed to clone boolean');
  if (structuredClone(null) !== null) throw new Error('failed to clone null');
});

// Test cloning objects
test('structuredClone clones objects', () => {
  const obj = { a: 1, b: 'hello', c: true };
  const cloned = structuredClone(obj);
  if (cloned === obj) throw new Error('cloned object is same reference');
  if (cloned.a !== 1 || cloned.b !== 'hello' || cloned.c !== true) {
    throw new Error('cloned object has incorrect values');
  }
});

// Test cloning arrays
test('structuredClone clones arrays', () => {
  const arr = [1, 2, 3];
  const cloned = structuredClone(arr);
  if (cloned === arr) throw new Error('cloned array is same reference');
  if (cloned.length !== 3 || cloned[0] !== 1 || cloned[1] !== 2 || cloned[2] !== 3) {
    throw new Error('cloned array has incorrect values');
  }
});

// Test cloning nested objects
test('structuredClone clones nested objects', () => {
  const obj = { a: { b: { c: 42 } } };
  const cloned = structuredClone(obj);
  if (cloned === obj) throw new Error('cloned object is same reference');
  if (cloned.a === obj.a) throw new Error('nested object is same reference');
  if (cloned.a.b.c !== 42) throw new Error('nested value is incorrect');
});

// Test cloning Date objects
test('structuredClone clones Date objects', () => {
  const date = new Date('2024-01-01T00:00:00.000Z');
  const cloned = structuredClone(date);
  if (cloned === date) throw new Error('cloned Date is same reference');
  if (!(cloned instanceof Date)) throw new Error('cloned value is not a Date');
  if (cloned.getTime() !== date.getTime()) throw new Error('Date time is incorrect');
});

// Test cloning RegExp objects
test('structuredClone clones RegExp objects', () => {
  const regex = /test/gi;
  const cloned = structuredClone(regex);
  if (cloned === regex) throw new Error('cloned RegExp is same reference');
  if (!(cloned instanceof RegExp)) throw new Error('cloned value is not a RegExp');
  if (cloned.source !== 'test') throw new Error('RegExp source is incorrect');
  if (cloned.flags !== 'gi') throw new Error('RegExp flags are incorrect');
});

// Test cloning Map objects
test('structuredClone clones Map objects', () => {
  const map = new Map([['key1', 'value1'], ['key2', 'value2']]);
  const cloned = structuredClone(map);
  if (cloned === map) throw new Error('cloned Map is same reference');
  if (!(cloned instanceof Map)) throw new Error('cloned value is not a Map');
  if (cloned.get('key1') !== 'value1') throw new Error('Map value is incorrect');
  if (cloned.size !== 2) throw new Error('Map size is incorrect');
});

// Test cloning Set objects
test('structuredClone clones Set objects', () => {
  const set = new Set([1, 2, 3]);
  const cloned = structuredClone(set);
  if (cloned === set) throw new Error('cloned Set is same reference');
  if (!(cloned instanceof Set)) throw new Error('cloned value is not a Set');
  if (!cloned.has(1) || !cloned.has(2) || !cloned.has(3)) throw new Error('Set values are incorrect');
  if (cloned.size !== 3) throw new Error('Set size is incorrect');
});

// Test cloning TypedArrays
test('structuredClone clones TypedArrays', () => {
  const arr = new Uint8Array([1, 2, 3]);
  const cloned = structuredClone(arr);
  if (cloned === arr) throw new Error('cloned TypedArray is same reference');
  if (!(cloned instanceof Uint8Array)) throw new Error('cloned value is not a Uint8Array');
  if (cloned.length !== 3 || cloned[0] !== 1 || cloned[1] !== 2 || cloned[2] !== 3) {
    throw new Error('TypedArray values are incorrect');
  }
});

// Test circular references
test('structuredClone handles circular references', () => {
  const obj = { name: 'circular' };
  obj.self = obj;
  const cloned = structuredClone(obj);
  if (cloned === obj) throw new Error('cloned object is same reference');
  if (cloned.self !== cloned) throw new Error('circular reference not preserved');
});

// Report results
console.log(`Structured Clone API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
