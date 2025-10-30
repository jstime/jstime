// Performance benchmark: Base64 operations
// Measures btoa/atob performance

const ITERATIONS = 100000;
const testString = 'Hello, World! This is a test string.';

const start = performance.now();

for (let i = 0; i < ITERATIONS; i++) {
  const encoded = btoa(testString);
  const decoded = atob(encoded);
}

const end = performance.now();
const elapsed = end - start;

console.log(JSON.stringify({
  test: 'base64_roundtrip',
  iterations: ITERATIONS,
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
}));
