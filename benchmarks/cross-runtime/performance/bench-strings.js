// Performance benchmark: String operations
// Measures string concatenation performance

const ITERATIONS = 10000;

const start = performance.now();

for (let i = 0; i < ITERATIONS; i++) {
  let s = '';
  for (let j = 0; j < 100; j++) {
    s += 'x';
  }
}

const end = performance.now();
const elapsed = end - start;

console.log(JSON.stringify({
  test: 'string_concat',
  iterations: ITERATIONS,
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
}));
