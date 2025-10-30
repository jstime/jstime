// Compliance test for JSON operations
// Tests JSON.parse and JSON.stringify

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

// Test JSON.parse exists
test('JSON.parse exists', () => {
  if (typeof JSON.parse !== 'function') throw new Error('JSON.parse is not a function');
});

// Test JSON.stringify exists
test('JSON.stringify exists', () => {
  if (typeof JSON.stringify !== 'function') throw new Error('JSON.stringify is not a function');
});

// Test JSON.parse with simple object
test('JSON.parse simple object', () => {
  const obj = JSON.parse('{"a":1,"b":"test","c":[1,2,3]}');
  if (obj.a !== 1 || obj.b !== 'test' || obj.c[1] !== 2) {
    throw new Error('JSON.parse failed to parse simple object');
  }
});

// Test JSON.stringify with simple object
test('JSON.stringify simple object', () => {
  const str = JSON.stringify({ a: 1, b: 'test', c: [1, 2, 3] });
  if (str !== '{"a":1,"b":"test","c":[1,2,3]}') {
    throw new Error(`JSON.stringify failed: got ${str}`);
  }
});

// Test round-trip
test('JSON round-trip', () => {
  const original = { 
    string: 'hello',
    number: 42,
    array: [1, 2, 3],
    nested: { key: 'value' },
    bool: true,
    null: null
  };
  const json = JSON.stringify(original);
  const parsed = JSON.parse(json);
  
  if (parsed.string !== original.string ||
      parsed.number !== original.number ||
      parsed.array[0] !== original.array[0] ||
      parsed.nested.key !== original.nested.key ||
      parsed.bool !== original.bool ||
      parsed.null !== original.null) {
    throw new Error('JSON round-trip failed');
  }
});

// Report results
console.log(`JSON API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
