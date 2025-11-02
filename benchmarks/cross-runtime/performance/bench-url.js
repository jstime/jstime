// Performance benchmark: URL operations
// Measures URL parsing performance

const ITERATIONS = 100000;

const results = [];
let totalElapsed = 0;

// Test 1: URL parsing
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const url = new URL('https://example.com/path?query=value&foo=bar#hash');
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'parse',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: URLSearchParams creation
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const params = new URLSearchParams('a=1&b=2&c=3');
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'searchparams',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: URL property access
start = performance.now();
const url = new URL('https://example.com/path?query=value&foo=bar#hash');
for (let i = 0; i < ITERATIONS; i++) {
  const h = url.hostname;
  const p = url.pathname;
  const s = url.search;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'property_access',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'url_parse',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
