// Performance benchmark: Object operations
// Measures object creation and access

const ITERATIONS = 100000;

const start = performance.now();

for (let i = 0; i < ITERATIONS; i++) {
  const obj = { a: 1, b: 2, c: 3, d: 4, e: 5 };
  const sum = obj.a + obj.b + obj.c + obj.d + obj.e;
}

const end = performance.now();
const elapsed = end - start;

console.log(JSON.stringify({
  test: 'object_ops',
  iterations: ITERATIONS,
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
}));
