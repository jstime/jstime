// Compliance test for Base64 encoding/decoding
// Tests btoa and atob

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

// Test btoa exists
test('btoa exists', () => {
  if (typeof btoa !== 'function') throw new Error('btoa is not a function');
});

// Test atob exists
test('atob exists', () => {
  if (typeof atob !== 'function') throw new Error('atob is not a function');
});

// Test btoa encoding
test('btoa encodes correctly', () => {
  const encoded = btoa('Hello, World!');
  if (encoded !== 'SGVsbG8sIFdvcmxkIQ==') throw new Error(`btoa encoding failed: got ${encoded}`);
});

// Test atob decoding
test('atob decodes correctly', () => {
  const decoded = atob('SGVsbG8sIFdvcmxkIQ==');
  if (decoded !== 'Hello, World!') throw new Error(`atob decoding failed: got ${decoded}`);
});

// Test round-trip
test('btoa/atob round-trip', () => {
  const original = 'Test string with special chars: !@#$%^&*()';
  const encoded = btoa(original);
  const decoded = atob(encoded);
  if (decoded !== original) throw new Error('btoa/atob round-trip failed');
});

// Report results
console.log(`Base64 API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
