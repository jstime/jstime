// Performance benchmark: Structured Clone operations
// Measures structuredClone performance

const ITERATIONS = 10000;

// Test object with nested structures
const testObject = {
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

const start = performance.now();

let sum = 0;
for (let i = 0; i < ITERATIONS; i++) {
  const cloned = structuredClone(testObject);
  sum += cloned.number;
}

const end = performance.now();
const elapsed = end - start;

console.log(JSON.stringify({
  test: 'structured_clone',
  iterations: ITERATIONS,
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
}));
