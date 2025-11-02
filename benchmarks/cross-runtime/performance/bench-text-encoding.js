// Performance benchmark: Text Encoding operations
// Measures TextEncoder and TextDecoder performance

const ITERATIONS = 10000;

const results = [];
let totalElapsed = 0;

const encoder = new TextEncoder();
const decoder = new TextDecoder();

// Test string with ASCII and UTF-8 multi-byte characters
const testString = 'Hello, World! 世界 🌍 Testing UTF-8 encoding and decoding performance.';

// Test 1: TextEncoder.encode
let start = performance.now();
let sum = 0;
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = encoder.encode(testString);
  sum += encoded.length;
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'encode',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: TextDecoder.decode
start = performance.now();
const encoded = encoder.encode(testString);
sum = 0;
for (let i = 0; i < ITERATIONS; i++) {
  const decoded = decoder.decode(encoded);
  sum += decoded.length;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'decode',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: Round-trip
start = performance.now();
sum = 0;
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = encoder.encode(testString);
  const decoded = decoder.decode(encoded);
  sum += decoded.length;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'roundtrip',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'text_encoding',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
