// Performance benchmark: queueMicrotask
// Measures microtask scheduling performance

const ITERATIONS = 10000;

const results = [];
let totalElapsed = 0;

// Test 1: Microtask scheduling (synchronous cost)
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  queueMicrotask(() => {});
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'schedule_empty',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: Microtask with closure
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const value = i;
  queueMicrotask(() => {
    const x = value * 2;
  });
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'schedule_with_closure',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: Multiple microtasks per iteration
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  queueMicrotask(() => {});
  queueMicrotask(() => {});
  queueMicrotask(() => {});
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'schedule_multiple',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'microtask',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
