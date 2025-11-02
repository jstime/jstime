// Performance benchmark: Base64 operations
// Measures btoa/atob performance

const ITERATIONS = 100000;

const results = [];
let totalElapsed = 0;

// Test 1: btoa (encode)
let start = performance.now();
const testString = 'Hello, World! This is a test string.';
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = btoa(testString);
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'encode',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: atob (decode)
start = performance.now();
const encoded = btoa(testString);
for (let i = 0; i < ITERATIONS; i++) {
  const decoded = atob(encoded);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'decode',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: Round-trip
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = btoa(testString);
  const decoded = atob(encoded);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'roundtrip',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'base64_roundtrip',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
