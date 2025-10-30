// Compliance test for Console API
// Tests basic console functionality across runtimes

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

// Test console.log
test('console.log exists', () => {
  if (typeof console.log !== 'function') throw new Error('console.log is not a function');
});

// Test console.error
test('console.error exists', () => {
  if (typeof console.error !== 'function') throw new Error('console.error is not a function');
});

// Test console.warn
test('console.warn exists', () => {
  if (typeof console.warn !== 'function') throw new Error('console.warn is not a function');
});

// Test console.info
test('console.info exists', () => {
  if (typeof console.info !== 'function') throw new Error('console.info is not a function');
});

// Test console.debug
test('console.debug exists', () => {
  if (typeof console.debug !== 'function') throw new Error('console.debug is not a function');
});

// Report results
console.log(`Console API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
