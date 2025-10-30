// Performance benchmark: Arithmetic operations
// Measures performance of basic arithmetic operations

const ITERATIONS = 100000;

const start = performance.now();

let sum = 0;
for (let i = 0; i < ITERATIONS; i++) {
  sum += i * 2 / 3 + 7 - 5;
}

const end = performance.now();
const elapsed = end - start;

console.log(JSON.stringify({
  test: 'arithmetic',
  iterations: ITERATIONS,
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
}));
