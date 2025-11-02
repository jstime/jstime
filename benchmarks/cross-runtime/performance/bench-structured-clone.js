// Performance benchmark: Structured Clone operations
// Measures structuredClone performance

const ITERATIONS = 10000;

const results = [];
let totalElapsed = 0;

// Test object with nested structures
const complexObject = {
  number: 42,
  string: 'Hello, World!',
  boolean: true,
  nested: {
    array: [1, 2, 3, 4, 5],
    date: new Date('2024-01-01'),
    map: new Map([['key1', 'value1'], ['key2', 'value2']]),
    set: new Set([1, 2, 3])
  },
  arr: [1, 2, 3, { x: 10, y: 20 }]
};

// Test 1: Clone simple object
let start = performance.now();
let sum = 0;
const simpleObj = { a: 1, b: 2, c: 3 };
for (let i = 0; i < ITERATIONS; i++) {
  const cloned = structuredClone(simpleObj);
  sum += cloned.a;
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'simple',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: Clone complex object
start = performance.now();
sum = 0;
for (let i = 0; i < ITERATIONS; i++) {
  const cloned = structuredClone(complexObject);
  sum += cloned.number;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'complex',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: Clone array
start = performance.now();
sum = 0;
const testArray = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
for (let i = 0; i < ITERATIONS; i++) {
  const cloned = structuredClone(testArray);
  sum += cloned[0];
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'array',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'structured_clone',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
