// Performance benchmark: Arithmetic operations
// Measures performance of basic arithmetic operations

const ITERATIONS = 100000;

const results = [];
let totalElapsed = 0;

// Test 1: Addition
let start = performance.now();
let sum1 = 0;
for (let i = 0; i < ITERATIONS; i++) {
  sum1 += i + 1;
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'addition',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: Subtraction
start = performance.now();
let sum2 = 0;
for (let i = 0; i < ITERATIONS; i++) {
  sum2 = i - 1;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'subtraction',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: Multiplication
start = performance.now();
let sum3 = 1;
for (let i = 0; i < ITERATIONS; i++) {
  sum3 = i * 2;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'multiplication',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: Division
start = performance.now();
let sum4 = 0;
for (let i = 1; i < ITERATIONS; i++) {
  sum4 = i / 2;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'division',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 5: Mixed operations
start = performance.now();
let sum5 = 0;
for (let i = 0; i < ITERATIONS; i++) {
  sum5 += i * 2 / 3 + 7 - 5;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'mixed',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'arithmetic',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
