// Performance benchmark: Crypto operations
// Measures crypto.randomUUID performance

const ITERATIONS = 10000;

const start = performance.now();

for (let i = 0; i < ITERATIONS; i++) {
  const uuid = crypto.randomUUID();
}

const end = performance.now();
const elapsed = end - start;

console.log(JSON.stringify({
  test: 'crypto_uuid',
  iterations: ITERATIONS,
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
}));
