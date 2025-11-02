// Performance benchmark: Event dispatch operations
// Measures Event and EventTarget performance

const ITERATIONS = 100000;

const results = [];
let totalElapsed = 0;

// Test 1: Event creation
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const event = new Event('test');
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'creation',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: EventTarget addEventListener
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const target = new EventTarget();
  target.addEventListener('test', () => {});
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'addEventListener',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: Event dispatch
start = performance.now();
const target = new EventTarget();
let counter = 0;
target.addEventListener('test', () => {
  counter++;
});
const event = new Event('test');
for (let i = 0; i < ITERATIONS; i++) {
  target.dispatchEvent(event);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'dispatch',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Use counter to prevent dead code elimination
if (counter !== ITERATIONS) {
  throw new Error(`Expected ${ITERATIONS} events, got ${counter}`);
}

console.log(JSON.stringify({
  test: 'event_dispatch',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
