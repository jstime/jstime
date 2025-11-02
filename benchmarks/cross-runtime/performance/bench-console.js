// Performance benchmark: Console operations
// Measures console API performance

const ITERATIONS = 10000;

const results = [];
let totalElapsed = 0;

// Test 1: console.log
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  console.log('Test message', i);
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'log',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: console.error
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  console.error('Error message', i);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'error',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: console.warn
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  console.warn('Warning message', i);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'warn',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: console.info
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  console.info('Info message', i);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'info',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'console',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
