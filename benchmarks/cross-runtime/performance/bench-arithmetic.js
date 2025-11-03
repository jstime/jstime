// Performance benchmark: Arithmetic operations
// Measures performance of basic arithmetic operations

const ITERATIONS = 100000;

const results = [];
let totalElapsed = 0;

// Test 1: Integer addition
let start = performance.now();
let sum1 = 0;
for (let i = 0; i < ITERATIONS; i++) {
  sum1 += i + 1;
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'addition_int',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: Float addition
start = performance.now();
let sum2 = 0.0;
for (let i = 0; i < ITERATIONS; i++) {
  sum2 += i * 1.5 + 2.7;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'addition_float',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: Integer subtraction
start = performance.now();
let sum3 = 0;
for (let i = 0; i < ITERATIONS; i++) {
  sum3 = i - 1;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'subtraction_int',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: Float subtraction
start = performance.now();
let sum4 = 0.0;
for (let i = 0; i < ITERATIONS; i++) {
  sum4 = i * 1.5 - 2.7;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'subtraction_float',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 5: Integer multiplication
start = performance.now();
let sum5 = 1;
for (let i = 0; i < ITERATIONS; i++) {
  sum5 = i * 2;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'multiplication_int',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 6: Float multiplication
start = performance.now();
let sum6 = 1.0;
for (let i = 0; i < ITERATIONS; i++) {
  sum6 = i * 2.5;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'multiplication_float',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 7: Integer division
start = performance.now();
let sum7 = 0;
for (let i = 1; i < ITERATIONS; i++) {
  sum7 = i / 2;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'division_int',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 8: Float division
start = performance.now();
let sum8 = 0.0;
for (let i = 1; i < ITERATIONS; i++) {
  sum8 = i / 2.5;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'division_float',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 9: Modulo operation
start = performance.now();
let sum9 = 0;
for (let i = 0; i < ITERATIONS; i++) {
  sum9 = i % 7;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'modulo',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 10: Exponentiation
start = performance.now();
let sum10 = 0;
for (let i = 0; i < ITERATIONS; i++) {
  sum10 = i ** 2;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'exponentiation',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 11: Bitwise AND
start = performance.now();
let sum11 = 0;
for (let i = 0; i < ITERATIONS; i++) {
  sum11 = i & 0xFF;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'bitwise_and',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 12: Bitwise OR
start = performance.now();
let sum12 = 0;
for (let i = 0; i < ITERATIONS; i++) {
  sum12 = i | 0x10;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'bitwise_or',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 13: Bitwise XOR
start = performance.now();
let sum13 = 0;
for (let i = 0; i < ITERATIONS; i++) {
  sum13 = i ^ 0xAA;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'bitwise_xor',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 14: Bit shift left
start = performance.now();
let sum14 = 0;
for (let i = 0; i < ITERATIONS; i++) {
  sum14 = i << 2;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'shift_left',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 15: Bit shift right
start = performance.now();
let sum15 = 0;
for (let i = 0; i < ITERATIONS; i++) {
  sum15 = i >> 2;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'shift_right',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 16: Mixed operations - combines multiple arithmetic operations in a single expression
start = performance.now();
let sum16 = 0;
for (let i = 0; i < ITERATIONS; i++) {
  sum16 += i * 2 / 3 + 7 - 5;
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
