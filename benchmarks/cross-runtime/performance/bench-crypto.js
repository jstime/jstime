// Performance benchmark: Crypto operations
// Measures crypto operations performance

const ITERATIONS = 10000;

const results = [];
let totalElapsed = 0;

// Test 1: crypto.randomUUID
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const uuid = crypto.randomUUID();
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'randomUUID',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: crypto.getRandomValues (small)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const arr = new Uint8Array(16);
  crypto.getRandomValues(arr);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'getRandomValues_small',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: crypto.getRandomValues (large)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const arr = new Uint8Array(1024);
  crypto.getRandomValues(arr);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'getRandomValues_large',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'crypto_uuid',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
