// Performance benchmark: Process API
// Measures process environment and argument access

const ITERATIONS = 100000;

const results = [];
let totalElapsed = 0;

// Test 1: process.env property access
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const path = process.env.PATH || process.env.Path;
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'env_access',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: process.argv access
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const argv0 = process.argv[0];
  const length = process.argv.length;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'argv_access',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: process.cwd() calls
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const cwd = process.cwd();
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'cwd_call',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: process.stdout.write (writing to stdout)
start = performance.now();
const testString = 'x';
for (let i = 0; i < ITERATIONS; i++) {
  // Note: we don't actually write to avoid flooding stdout
  // Just measure the method access
  const write = process.stdout.write;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'stdout_access',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 5: Iterate over process.env keys
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const keys = Object.keys(process.env);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'env_keys',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'process',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
