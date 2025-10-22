// Performance API Demo for jstime
// This example demonstrates the Performance API for high-resolution timing

console.log('=== jstime Performance API Demo ===\n');

// 1. Basic timing with performance.now()
console.log('1. Basic timing with performance.now():');
const start = performance.now();
console.log('   Start time:', start, 'ms');

// Simulate some work
let sum = 0;
for (let i = 0; i < 1000000; i++) {
  sum += i;
}

const end = performance.now();
console.log('   End time:', end, 'ms');
console.log('   Duration:', (end - start).toFixed(3), 'ms');
console.log();

// 2. Measuring function execution time
console.log('2. Measuring function execution time:');

function slowFunction() {
  let result = 0;
  for (let i = 0; i < 5000000; i++) {
    result += Math.sqrt(i);
  }
  return result;
}

const funcStart = performance.now();
const result = slowFunction();
const funcEnd = performance.now();

console.log('   Function result:', result.toFixed(2));
console.log('   Execution time:', (funcEnd - funcStart).toFixed(3), 'ms');
console.log();

// 3. Comparing different operations
console.log('3. Comparing different operations:');

// Array operations
const arrStart = performance.now();
const arr = new Array(100000).fill(0).map((_, i) => i * 2);
const arrEnd = performance.now();
console.log('   Array operations:', (arrEnd - arrStart).toFixed(3), 'ms');

// Object operations
const objStart = performance.now();
const obj = {};
for (let i = 0; i < 100000; i++) {
  obj[`key${i}`] = i;
}
const objEnd = performance.now();
console.log('   Object operations:', (objEnd - objStart).toFixed(3), 'ms');

// String operations
const strStart = performance.now();
let str = '';
for (let i = 0; i < 10000; i++) {
  str += 'a';
}
const strEnd = performance.now();
console.log('   String concatenation:', (strEnd - strStart).toFixed(3), 'ms');
console.log();

// 4. High-resolution timing
console.log('4. High-resolution timing (microsecond precision):');
const t1 = performance.now();
const t2 = performance.now();
const t3 = performance.now();

console.log('   Time 1:', t1);
console.log('   Time 2:', t2);
console.log('   Time 3:', t3);
console.log('   Minimum measurable interval:', Math.min(t2 - t1, t3 - t2).toFixed(6), 'ms');
console.log();

// 5. Measuring async operations
console.log('5. Measuring async operations:');

async function measureAsync() {
  const asyncStart = performance.now();
  await new Promise(resolve => setTimeout(resolve, 100));
  const asyncEnd = performance.now();
  console.log('   Async operation (100ms delay):', (asyncEnd - asyncStart).toFixed(3), 'ms');
  console.log();
}

// Note: In a script file, we can't use top-level await
// This would work in an ES module file
console.log('   (async measurement requires ES module context)');
console.log();

// 6. Performance.timeOrigin
console.log('6. Performance time origin:');
console.log('   timeOrigin:', performance.timeOrigin, 'ms since Unix epoch');
console.log('   Current time:', Date.now(), 'ms since Unix epoch');
console.log('   Difference:', (Date.now() - performance.timeOrigin).toFixed(3), 'ms');
console.log();

// 7. Practical example: Benchmarking
console.log('7. Practical example: Benchmarking two implementations:');

function fibonacci1(n) {
  if (n <= 1) return n;
  return fibonacci1(n - 1) + fibonacci1(n - 2);
}

function fibonacci2(n) {
  const fib = [0, 1];
  for (let i = 2; i <= n; i++) {
    fib[i] = fib[i - 1] + fib[i - 2];
  }
  return fib[n];
}

const n = 30;

const fib1Start = performance.now();
const result1 = fibonacci1(n);
const fib1End = performance.now();

const fib2Start = performance.now();
const result2 = fibonacci2(n);
const fib2End = performance.now();

console.log(`   Recursive fibonacci(${n}):`, (fib1End - fib1Start).toFixed(3), 'ms');
console.log(`   Iterative fibonacci(${n}):`, (fib2End - fib2Start).toFixed(3), 'ms');
console.log(`   Speedup: ${((fib1End - fib1Start) / (fib2End - fib2Start)).toFixed(2)}x faster`);
console.log();

console.log('=== Performance Demo Complete ===');
