// Performance benchmark: String operations
// Measures string operation performance

const ITERATIONS = 10000;

const results = [];
let totalElapsed = 0;

// Test 1: Concatenation with +
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  let s = '';
  for (let j = 0; j < 100; j++) {
    s += 'x';
  }
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'concatenation',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: Template literals
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const name = 'World';
  const age = 42;
  const s = `Hello, ${name}! You are ${age} years old.`;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'template_literals',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: String repeat
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const s = 'x'.repeat(100);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'repeat',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: String split and join
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const s = 'hello,world,foo,bar,baz'.split(',').join('-');
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'split_join',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'string_concat',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
