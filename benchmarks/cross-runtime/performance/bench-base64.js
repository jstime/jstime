// Performance benchmark: Base64 operations
// Measures btoa/atob performance

const ITERATIONS = 100000;

const results = [];
let totalElapsed = 0;

// Test 1: btoa (encode) - short string
let start = performance.now();
const shortString = 'Hello, World!';
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = btoa(shortString);
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'encode_short',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: btoa (encode) - medium string
start = performance.now();
const mediumString = 'Hello, World! This is a test string. '.repeat(5);
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = btoa(mediumString);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'encode_medium',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: btoa (encode) - long string
start = performance.now();
const longString = 'Hello, World! This is a test string. '.repeat(20);
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = btoa(longString);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'encode_long',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: atob (decode) - short
start = performance.now();
const encodedShort = btoa(shortString);
for (let i = 0; i < ITERATIONS; i++) {
  const decoded = atob(encodedShort);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'decode_short',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 5: atob (decode) - medium
start = performance.now();
const encodedMedium = btoa(mediumString);
for (let i = 0; i < ITERATIONS; i++) {
  const decoded = atob(encodedMedium);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'decode_medium',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 6: atob (decode) - long
start = performance.now();
const encodedLong = btoa(longString);
for (let i = 0; i < ITERATIONS; i++) {
  const decoded = atob(encodedLong);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'decode_long',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 7: Round-trip - short
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = btoa(shortString);
  const decoded = atob(encoded);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'roundtrip_short',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 8: Round-trip - medium
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = btoa(mediumString);
  const decoded = atob(encoded);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'roundtrip_medium',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 9: Round-trip - long
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = btoa(longString);
  const decoded = atob(encoded);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'roundtrip_long',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'base64_roundtrip',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
