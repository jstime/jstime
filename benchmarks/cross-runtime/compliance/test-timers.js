// Compliance test for Timers API
// Tests setTimeout, setInterval, and clearing functions

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

// Test setTimeout
test('setTimeout exists', () => {
  if (typeof setTimeout !== 'function') throw new Error('setTimeout is not a function');
});

// Test clearTimeout
test('clearTimeout exists', () => {
  if (typeof clearTimeout !== 'function') throw new Error('clearTimeout is not a function');
});

// Test setInterval
test('setInterval exists', () => {
  if (typeof setInterval !== 'function') throw new Error('setInterval is not a function');
});

// Test clearInterval
test('clearInterval exists', () => {
  if (typeof clearInterval !== 'function') throw new Error('clearInterval is not a function');
});

// Test setTimeout returns a value
test('setTimeout returns a timer ID', () => {
  const id = setTimeout(() => {}, 1000);
  if (id === undefined || id === null) throw new Error('setTimeout did not return a timer ID');
  clearTimeout(id);
});

// Test setInterval returns a value
test('setInterval returns a timer ID', () => {
  const id = setInterval(() => {}, 1000);
  if (id === undefined || id === null) throw new Error('setInterval did not return a timer ID');
  clearInterval(id);
});

// Report results
console.log(`Timers API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
