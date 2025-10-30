// Compliance test for Performance API
// Tests performance.now() and performance.timeOrigin

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

// Test performance object
test('performance exists', () => {
  if (typeof performance !== 'object') throw new Error('performance is not an object');
});

// Test performance.now
test('performance.now exists', () => {
  if (typeof performance.now !== 'function') throw new Error('performance.now is not a function');
});

// Test performance.now returns a number
test('performance.now returns a number', () => {
  const t = performance.now();
  if (typeof t !== 'number') throw new Error(`performance.now returned ${typeof t} instead of number`);
});

// Test performance.now is monotonically increasing
test('performance.now is monotonically increasing', () => {
  const t1 = performance.now();
  // Do some work
  let sum = 0;
  for (let i = 0; i < 1000; i++) sum += i;
  const t2 = performance.now();
  if (t2 < t1) throw new Error('performance.now is not monotonically increasing');
});

// Test performance.timeOrigin exists
test('performance.timeOrigin exists', () => {
  if (typeof performance.timeOrigin !== 'number') throw new Error('performance.timeOrigin is not a number');
});

// Test performance.timeOrigin is reasonable (Unix timestamp in milliseconds)
test('performance.timeOrigin is reasonable', () => {
  const origin = performance.timeOrigin;
  // Should be a recent timestamp (after 2020 and before 2100)
  const year2020 = 1577836800000;
  const year2100 = 4102444800000;
  if (origin < year2020 || origin > year2100) {
    throw new Error(`performance.timeOrigin (${origin}) is not a reasonable timestamp`);
  }
});

// Report results
console.log(`Performance API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
