// Performance benchmark: Object operations
// Measures object creation and access

const ITERATIONS = 100000;

const results = [];
let totalElapsed = 0;

// Test 1: Object creation
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const obj = { a: 1, b: 2, c: 3, d: 4, e: 5 };
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'creation',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: Property access
start = performance.now();
const obj = { a: 1, b: 2, c: 3, d: 4, e: 5 };
for (let i = 0; i < ITERATIONS; i++) {
  const sum = obj.a + obj.b + obj.c + obj.d + obj.e;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'property_access',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: Property assignment
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const obj = {};
  obj.a = 1;
  obj.b = 2;
  obj.c = 3;
  obj.d = 4;
  obj.e = 5;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'property_assignment',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: Object spread
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const obj1 = { a: 1, b: 2 };
  const obj2 = { c: 3, d: 4 };
  const obj3 = { ...obj1, ...obj2 };
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'spread',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'object_ops',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
