// Performance benchmark: Array operations
// Measures various array operation performance

const ITERATIONS = 1000;
const ARRAY_SIZE = 1000;

const results = [];
let totalElapsed = 0;

// Test 1: Array creation
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const arr = Array.from({ length: ARRAY_SIZE }, (_, i) => i);
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'creation',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: Array map
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const arr = Array.from({ length: ARRAY_SIZE }, (_, i) => i);
  const result = arr.map(x => x * 2);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'map',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: Array filter
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const arr = Array.from({ length: ARRAY_SIZE }, (_, i) => i);
  const result = arr.filter(x => x % 2 === 0);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'filter',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: Array reduce
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const arr = Array.from({ length: ARRAY_SIZE }, (_, i) => i);
  const result = arr.reduce((acc, x) => acc + x, 0);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'reduce',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 5: Array push/pop
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const arr = [];
  for (let j = 0; j < ARRAY_SIZE; j++) {
    arr.push(j);
  }
  for (let j = 0; j < ARRAY_SIZE; j++) {
    arr.pop();
  }
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'push_pop',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'array_map',
  iterations: ITERATIONS,
  array_size: ARRAY_SIZE,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
