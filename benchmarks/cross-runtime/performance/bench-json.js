// Performance benchmark: JSON operations
// Measures JSON.parse and JSON.stringify performance

const ITERATIONS = 100000;

const results = [];
let totalElapsed = 0;

// Test 1: JSON.stringify (small object)
let start = performance.now();
const smallObj = { a: 1, b: 'test', c: [1, 2, 3] };
for (let i = 0; i < ITERATIONS; i++) {
  const json = JSON.stringify(smallObj);
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'stringify_small',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: JSON.parse (small object)
start = performance.now();
const smallJson = JSON.stringify(smallObj);
for (let i = 0; i < ITERATIONS; i++) {
  const obj = JSON.parse(smallJson);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'parse_small',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: JSON round-trip (small object)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const json = JSON.stringify(smallObj);
  const obj = JSON.parse(json);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'roundtrip_small',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'json_roundtrip',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
