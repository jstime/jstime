// Performance benchmark: JSON operations
// Measures JSON.parse and JSON.stringify performance

const ITERATIONS = 100000;

const results = [];
let totalElapsed = 0;

// Test objects of various sizes and complexities
const smallObj = { a: 1, b: 'test', c: [1, 2, 3] };
const mediumObj = {
  id: 12345,
  name: 'John Doe',
  email: 'john@example.com',
  age: 30,
  address: {
    street: '123 Main St',
    city: 'Springfield',
    state: 'IL',
    zip: '62701'
  },
  tags: ['javascript', 'nodejs', 'web'],
  active: true
};
const largeObj = {
  users: Array(50).fill(null).map((_, i) => ({
    id: i,
    name: `User ${i}`,
    email: `user${i}@example.com`,
    metadata: { created: Date.now(), updated: Date.now() }
  }))
};
const arrayOfNumbers = Array(100).fill(0).map((_, i) => i);
const arrayOfStrings = Array(100).fill(0).map((_, i) => `string-${i}`);
const deeplyNested = {
  level1: {
    level2: {
      level3: {
        level4: {
          level5: {
            value: 'deep'
          }
        }
      }
    }
  }
};

// Test 1: JSON.stringify (small object)
let start = performance.now();
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

// Test 2: JSON.stringify (medium object)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const json = JSON.stringify(mediumObj);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'stringify_medium',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: JSON.stringify (large object)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const json = JSON.stringify(largeObj);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'stringify_large',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: JSON.stringify (array of numbers)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const json = JSON.stringify(arrayOfNumbers);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'stringify_array_numbers',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 5: JSON.stringify (array of strings)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const json = JSON.stringify(arrayOfStrings);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'stringify_array_strings',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 6: JSON.stringify (deeply nested)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const json = JSON.stringify(deeplyNested);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'stringify_nested',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 7: JSON.parse (small object)
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

// Test 8: JSON.parse (medium object)
start = performance.now();
const mediumJson = JSON.stringify(mediumObj);
for (let i = 0; i < ITERATIONS; i++) {
  const obj = JSON.parse(mediumJson);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'parse_medium',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 9: JSON.parse (large object)
start = performance.now();
const largeJson = JSON.stringify(largeObj);
for (let i = 0; i < ITERATIONS; i++) {
  const obj = JSON.parse(largeJson);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'parse_large',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 10: JSON.parse (array of numbers)
start = performance.now();
const arrayNumbersJson = JSON.stringify(arrayOfNumbers);
for (let i = 0; i < ITERATIONS; i++) {
  const obj = JSON.parse(arrayNumbersJson);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'parse_array_numbers',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 11: JSON.parse (array of strings)
start = performance.now();
const arrayStringsJson = JSON.stringify(arrayOfStrings);
for (let i = 0; i < ITERATIONS; i++) {
  const obj = JSON.parse(arrayStringsJson);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'parse_array_strings',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 12: JSON round-trip (small object)
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

// Test 13: JSON round-trip (medium object)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const json = JSON.stringify(mediumObj);
  const obj = JSON.parse(json);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'roundtrip_medium',
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
