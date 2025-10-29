#!/usr/bin/env jstime
// JavaScript Performance Benchmarks for jstime
// Run with: jstime benchmarks/js_benchmarks.js

console.log('=== jstime JavaScript Benchmarks ===\n');

function benchmark(name, fn, iterations = 1000) {
  // Warmup
  for (let i = 0; i < 10; i++) fn();
  
  const start = performance.now();
  for (let i = 0; i < iterations; i++) {
    fn();
  }
  const end = performance.now();
  
  const totalTime = (end - start).toFixed(3);
  const avgTime = ((end - start) / iterations).toFixed(6);
  
  console.log(`${name}:`);
  console.log(`  Total: ${totalTime}ms for ${iterations} iterations`);
  console.log(`  Average: ${avgTime}ms per iteration\n`);
  
  return { totalTime: parseFloat(totalTime), avgTime: parseFloat(avgTime) };
}

// 1. Arithmetic Operations
benchmark('Arithmetic (1M ops)', () => {
  let sum = 0;
  for (let i = 0; i < 1000; i++) {
    sum += i * 2 / 3 + 7 - 5;
  }
  return sum;
}, 1000);

// 2. String Operations
benchmark('String Concatenation (100 chars)', () => {
  let s = '';
  for (let i = 0; i < 100; i++) {
    s += 'x';
  }
  return s;
}, 1000);

benchmark('String Template Literals', () => {
  const name = 'World';
  const age = 42;
  return `Hello, ${name}! You are ${age} years old.`;
}, 10000);

// 3. Array Operations
benchmark('Array Creation (1K elements)', () => {
  return Array.from({ length: 1000 }, (_, i) => i);
}, 1000);

benchmark('Array Map', () => {
  const arr = Array.from({ length: 1000 }, (_, i) => i);
  return arr.map(x => x * 2);
}, 1000);

benchmark('Array Filter', () => {
  const arr = Array.from({ length: 1000 }, (_, i) => i);
  return arr.filter(x => x % 2 === 0);
}, 1000);

benchmark('Array Reduce', () => {
  const arr = Array.from({ length: 1000 }, (_, i) => i);
  return arr.reduce((acc, x) => acc + x, 0);
}, 1000);

// 4. Object Operations
benchmark('Object Creation', () => {
  return { a: 1, b: 2, c: 3, d: 4, e: 5 };
}, 10000);

benchmark('Object Property Access', () => {
  const obj = { a: 1, b: 2, c: 3, d: 4, e: 5 };
  return obj.a + obj.b + obj.c + obj.d + obj.e;
}, 10000);

benchmark('Object Spread', () => {
  const obj1 = { a: 1, b: 2 };
  const obj2 = { c: 3, d: 4 };
  return { ...obj1, ...obj2 };
}, 10000);

// 5. Function Calls
benchmark('Function Calls (Simple)', () => {
  function add(a, b) {
    return a + b;
  }
  return add(1, 2);
}, 10000);

benchmark('Function Calls (Arrow)', () => {
  const add = (a, b) => a + b;
  return add(1, 2);
}, 10000);

benchmark('Recursive Fibonacci(20)', () => {
  function fib(n) {
    if (n <= 1) return n;
    return fib(n - 1) + fib(n - 2);
  }
  return fib(20);
}, 10);

// 6. JSON Operations
benchmark('JSON.stringify (small object)', () => {
  const obj = { a: 1, b: 'test', c: [1, 2, 3] };
  return JSON.stringify(obj);
}, 10000);

benchmark('JSON.parse (small object)', () => {
  return JSON.parse('{"a":1,"b":"test","c":[1,2,3]}');
}, 10000);

benchmark('JSON Round Trip', () => {
  const obj = { a: 1, b: 'test', c: [1, 2, 3] };
  return JSON.parse(JSON.stringify(obj));
}, 10000);

// 7. Console API
benchmark('console.log (suppressed output)', () => {
  // Note: This measures the overhead of console.log calls
  // In production, output would be visible
  console.log('benchmark', 123, { key: 'value' });
}, 1000);

// 8. Performance API
benchmark('performance.now() calls', () => {
  return performance.now();
}, 10000);

// 9. Base64 Operations
benchmark('btoa (base64 encode)', () => {
  return btoa('Hello, World! This is a test string.');
}, 10000);

benchmark('atob (base64 decode)', () => {
  return atob('SGVsbG8sIFdvcmxkISBUaGlzIGlzIGEgdGVzdCBzdHJpbmcu');
}, 10000);

// 10. URL Operations
benchmark('URL Parse', () => {
  return new URL('https://example.com/path?query=value&foo=bar#hash');
}, 10000);

benchmark('URLSearchParams', () => {
  const params = new URLSearchParams('a=1&b=2&c=3');
  return params.get('b');
}, 10000);

// 11. Crypto Operations
benchmark('crypto.randomUUID()', () => {
  return crypto.randomUUID();
}, 1000);

benchmark('crypto.getRandomValues()', () => {
  return crypto.getRandomValues(new Uint8Array(32));
}, 1000);

// 12. Event System
benchmark('Event Creation', () => {
  return new Event('test');
}, 10000);

benchmark('EventTarget + Dispatch', () => {
  const target = new EventTarget();
  let called = false;
  target.addEventListener('test', () => { called = true; });
  target.dispatchEvent(new Event('test'));
  return called;
}, 1000);

console.log('=== Benchmarks Complete ===');
