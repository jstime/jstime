// Compliance test for Microtask API
// Tests queueMicrotask for microtask scheduling

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

// Test queueMicrotask exists
test('queueMicrotask exists', () => {
  if (typeof queueMicrotask !== 'function') throw new Error('queueMicrotask is not a function');
});

// Test queueMicrotask accepts a callback
test('queueMicrotask accepts callback', () => {
  queueMicrotask(() => {});
});

// Note: We cannot easily test microtask execution order in a synchronous test
// since microtasks run after the current script, but we can test that it doesn't error

// Report results
console.log(`Microtask API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
