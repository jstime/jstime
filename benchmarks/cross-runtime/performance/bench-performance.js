// Performance benchmark: Performance API
// Measures performance.now() call overhead

const ITERATIONS = 1000000;

const results = [];
let totalElapsed = 0;

// Test 1: performance.now() calls
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const t = performance.now();
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'now',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: performance.timeOrigin access
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const t = performance.timeOrigin;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'timeOrigin',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: time measurements (start + end pattern)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const t1 = performance.now();
  const t2 = performance.now();
  const delta = t2 - t1;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'measurements',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'performance',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
