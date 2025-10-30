// Compliance test for Crypto API
// Tests crypto.getRandomValues and crypto.randomUUID

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

// Test crypto object
test('crypto exists', () => {
  if (typeof crypto !== 'object') throw new Error('crypto is not an object');
});

// Test crypto.getRandomValues
test('crypto.getRandomValues exists', () => {
  if (typeof crypto.getRandomValues !== 'function') throw new Error('crypto.getRandomValues is not a function');
});

// Test crypto.getRandomValues with Uint8Array
test('crypto.getRandomValues works with Uint8Array', () => {
  const arr = new Uint8Array(16);
  crypto.getRandomValues(arr);
  // Check that at least one value is non-zero (probabilistically should always pass)
  let hasNonZero = false;
  for (let i = 0; i < arr.length; i++) {
    if (arr[i] !== 0) {
      hasNonZero = true;
      break;
    }
  }
  if (!hasNonZero) throw new Error('crypto.getRandomValues returned all zeros');
});

// Test crypto.randomUUID
test('crypto.randomUUID exists', () => {
  if (typeof crypto.randomUUID !== 'function') throw new Error('crypto.randomUUID is not a function');
});

// Test crypto.randomUUID format
test('crypto.randomUUID returns valid UUID', () => {
  const uuid = crypto.randomUUID();
  const uuidPattern = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
  if (!uuidPattern.test(uuid)) throw new Error(`crypto.randomUUID returned invalid UUID: ${uuid}`);
});

// Report results
console.log(`Crypto API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
