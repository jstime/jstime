// Performance benchmark: Text Encoding operations
// Measures TextEncoder and TextDecoder performance

const ITERATIONS = 10000;

const results = [];
let totalElapsed = 0;

const encoder = new TextEncoder();
const decoder = new TextDecoder();

// Test strings of different sizes and content types
const asciiShort = 'Hello, World!';
const asciiMedium = 'Hello, World! '.repeat(10);
const asciiLong = 'Hello, World! '.repeat(100);
const utf8Short = '世界 🌍';
const utf8Medium = '世界 🌍 '.repeat(10);
const utf8Long = '世界 🌍 '.repeat(100);
const mixed = 'Hello, World! 世界 🌍 Testing UTF-8 encoding and decoding performance.';

// Test 1: TextEncoder.encode - ASCII short
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = encoder.encode(asciiShort);
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'encode_ascii_short',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: TextEncoder.encode - ASCII medium
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = encoder.encode(asciiMedium);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'encode_ascii_medium',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: TextEncoder.encode - ASCII long
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = encoder.encode(asciiLong);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'encode_ascii_long',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: TextEncoder.encode - UTF-8 short
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = encoder.encode(utf8Short);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'encode_utf8_short',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 5: TextEncoder.encode - UTF-8 medium
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = encoder.encode(utf8Medium);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'encode_utf8_medium',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 6: TextEncoder.encode - UTF-8 long
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = encoder.encode(utf8Long);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'encode_utf8_long',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 7: TextEncoder.encode - mixed
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = encoder.encode(mixed);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'encode_mixed',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 8: TextDecoder.decode - ASCII short
start = performance.now();
const encodedAsciiShort = encoder.encode(asciiShort);
for (let i = 0; i < ITERATIONS; i++) {
  const decoded = decoder.decode(encodedAsciiShort);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'decode_ascii_short',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 9: TextDecoder.decode - ASCII medium
start = performance.now();
const encodedAsciiMedium = encoder.encode(asciiMedium);
for (let i = 0; i < ITERATIONS; i++) {
  const decoded = decoder.decode(encodedAsciiMedium);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'decode_ascii_medium',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 10: TextDecoder.decode - ASCII long
start = performance.now();
const encodedAsciiLong = encoder.encode(asciiLong);
for (let i = 0; i < ITERATIONS; i++) {
  const decoded = decoder.decode(encodedAsciiLong);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'decode_ascii_long',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 11: TextDecoder.decode - UTF-8 short
start = performance.now();
const encodedUtf8Short = encoder.encode(utf8Short);
for (let i = 0; i < ITERATIONS; i++) {
  const decoded = decoder.decode(encodedUtf8Short);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'decode_utf8_short',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 12: TextDecoder.decode - UTF-8 medium
start = performance.now();
const encodedUtf8Medium = encoder.encode(utf8Medium);
for (let i = 0; i < ITERATIONS; i++) {
  const decoded = decoder.decode(encodedUtf8Medium);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'decode_utf8_medium',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 13: TextDecoder.decode - UTF-8 long
start = performance.now();
const encodedUtf8Long = encoder.encode(utf8Long);
for (let i = 0; i < ITERATIONS; i++) {
  const decoded = decoder.decode(encodedUtf8Long);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'decode_utf8_long',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 14: Round-trip - short
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const encoded = encoder.encode(mixed);
  const decoded = decoder.decode(encoded);
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
