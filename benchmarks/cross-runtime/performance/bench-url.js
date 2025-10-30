// Performance benchmark: URL operations
// Measures URL parsing performance

const ITERATIONS = 100000;

const start = performance.now();

for (let i = 0; i < ITERATIONS; i++) {
  const url = new URL('https://example.com/path?query=value&foo=bar#hash');
}

const end = performance.now();
const elapsed = end - start;

console.log(JSON.stringify({
  test: 'url_parse',
  iterations: ITERATIONS,
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
}));
