// Performance benchmark: Array operations
// Measures array map performance

const ITERATIONS = 1000;
const ARRAY_SIZE = 1000;

const start = performance.now();

for (let i = 0; i < ITERATIONS; i++) {
  const arr = Array.from({ length: ARRAY_SIZE }, (_, i) => i);
  const result = arr.map(x => x * 2);
}

const end = performance.now();
const elapsed = end - start;

console.log(JSON.stringify({
  test: 'array_map',
  iterations: ITERATIONS,
  array_size: ARRAY_SIZE,
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
}));
