#!/usr/bin/env jstime
// JIT Warmup Example
//
// This example demonstrates how JIT warmup can improve performance
// for compute-intensive JavaScript code.
//
// Usage:
//   Without warmup: jstime examples/warmup-demo.js
//   With warmup:    jstime --warmup 10 examples/warmup-demo.js
//
// The warmup runs execute the code multiple times before the actual
// run, allowing V8's TurboFan JIT compiler to optimize hot code paths.

console.log('=== JIT Warmup Performance Demo ===\n');

// Example 1: Prime number calculation
function isPrime(n) {
    if (n <= 1) return false;
    if (n <= 3) return true;
    if (n % 2 === 0 || n % 3 === 0) return false;
    
    for (let i = 5; i * i <= n; i += 6) {
        if (n % i === 0 || n % (i + 2) === 0) {
            return false;
        }
    }
    return true;
}

function countPrimes(limit) {
    let count = 0;
    for (let i = 2; i <= limit; i++) {
        if (isPrime(i)) count++;
    }
    return count;
}

console.log('Example 1: Prime Number Calculation');
console.log('Finding primes up to 10,000...');
const start1 = performance.now();
const primeCount = countPrimes(10000);
const end1 = performance.now();
console.log(`Found ${primeCount} primes`);
console.log(`Time: ${(end1 - start1).toFixed(3)}ms\n`);

// Example 2: Fibonacci with memoization
function fibonacci(n, memo = {}) {
    if (n <= 1) return n;
    if (memo[n]) return memo[n];
    
    memo[n] = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);
    return memo[n];
}

console.log('Example 2: Fibonacci with Memoization');
console.log('Computing fibonacci(30)...');
const start2 = performance.now();
const fib = fibonacci(30);
const end2 = performance.now();
console.log(`fibonacci(30) = ${fib}`);
console.log(`Time: ${(end2 - start2).toFixed(3)}ms\n`);

// Example 3: Array operations with heavy computation
function complexArrayOps(size) {
    const arr = Array.from({ length: size }, (_, i) => i);
    
    return arr
        .map(x => x * 2)
        .filter(x => x % 3 === 0)
        .reduce((sum, x) => sum + Math.sqrt(x), 0);
}

console.log('Example 3: Complex Array Operations');
console.log('Processing array of 10,000 elements...');
const start3 = performance.now();
const result = complexArrayOps(10000);
const end3 = performance.now();
console.log(`Result: ${Math.floor(result)}`);
console.log(`Time: ${(end3 - start3).toFixed(3)}ms\n`);

// Example 4: String manipulation
function processStrings(count) {
    let result = '';
    for (let i = 0; i < count; i++) {
        result += String.fromCharCode(65 + (i % 26));
        if ((i + 1) % 10 === 0) {
            result += ' ';
        }
    }
    return result.length;
}

console.log('Example 4: String Manipulation');
console.log('Generating string with 5,000 characters...');
const start4 = performance.now();
const strLen = processStrings(5000);
const end4 = performance.now();
console.log(`Generated string of length: ${strLen}`);
console.log(`Time: ${(end4 - start4).toFixed(3)}ms\n`);

console.log('=== Summary ===');
console.log('Run this example with --warmup flag to see potential improvements:');
console.log('  jstime --warmup 10 examples/warmup-demo.js');
console.log("\nWarmup allows V8's JIT compiler to optimize the code before");
console.log('the actual execution, which can improve performance for');
console.log('compute-intensive operations.');
