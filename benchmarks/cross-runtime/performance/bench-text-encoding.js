// Performance benchmark: Text Encoding operations
// Measures TextEncoder and TextDecoder performance

const ITERATIONS = 10000;

const encoder = new TextEncoder();
const decoder = new TextDecoder();

// Test string with ASCII and UTF-8 multi-byte characters
const testString = 'Hello, World! 世界 🌍 Testing UTF-8 encoding and decoding performance.';

const start = performance.now();

for (let i = 0; i < ITERATIONS; i++) {
  const encoded = encoder.encode(testString);
  const decoded = decoder.decode(encoded);
}

const end = performance.now();
const elapsed = end - start;

console.log(JSON.stringify({
  test: 'text_encoding',
  iterations: ITERATIONS,
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
}));
