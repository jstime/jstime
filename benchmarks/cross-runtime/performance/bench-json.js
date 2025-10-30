// Performance benchmark: JSON operations
// Measures JSON.parse and JSON.stringify performance

const ITERATIONS = 100000;
const testObj = { a: 1, b: 'test', c: [1, 2, 3] };

const start = performance.now();

for (let i = 0; i < ITERATIONS; i++) {
  const json = JSON.stringify(testObj);
  const obj = JSON.parse(json);
}

const end = performance.now();
const elapsed = end - start;

console.log(JSON.stringify({
  test: 'json_roundtrip',
  iterations: ITERATIONS,
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
}));
