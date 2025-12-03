// Performance benchmark: Function operations
// Measures function call and closure performance

const ITERATIONS = 100000;

const results = [];
let totalElapsed = 0;

// Test 1: Simple function calls
function simpleAdd(a, b) {
  return a + b;
}

let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  simpleAdd(i, 1);
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'simple_call',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: Arrow function calls
const arrowAdd = (a, b) => a + b;

start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  arrowAdd(i, 1);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'arrow_call',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: Method calls
const obj = {
  value: 10,
  add(x) {
    return this.value + x;
  }
};

start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  obj.add(i);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'method_call',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: Closure calls
function makeAdder(x) {
  return function(y) {
    return x + y;
  };
}
const addFive = makeAdder(5);

start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  addFive(i);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'closure_call',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 5: Recursive calls (fibonacci 15)
function fib(n) {
  if (n <= 1) return n;
  return fib(n - 1) + fib(n - 2);
}

const RECURSIVE_ITERATIONS = 1000;
start = performance.now();
for (let i = 0; i < RECURSIVE_ITERATIONS; i++) {
  fib(15);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'recursive_fib15',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (RECURSIVE_ITERATIONS / elapsed).toFixed(2)
});

// Test 6: Function.prototype.call
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  simpleAdd.call(null, i, 1);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'call_method',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 7: Function.prototype.apply
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  simpleAdd.apply(null, [i, 1]);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'apply_method',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 8: Spread call
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const args = [i, 1];
  simpleAdd(...args);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'spread_call',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'functions',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * (results.length - 1) / totalElapsed).toFixed(2),
  sub_tests: results
}));
