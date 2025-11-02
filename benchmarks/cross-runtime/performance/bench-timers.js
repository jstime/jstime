// Performance benchmark: Timers API
// Measures setTimeout/setInterval performance (without actual waiting)

const ITERATIONS = 10000;

const results = [];
let totalElapsed = 0;

// Test 1: setTimeout creation and cancellation
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const id = setTimeout(() => {}, 1000);
  clearTimeout(id);
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'setTimeout_cancel',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: setInterval creation and cancellation
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const id = setInterval(() => {}, 1000);
  clearInterval(id);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'setInterval_cancel',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: Multiple setTimeout/clearTimeout cycles
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const id1 = setTimeout(() => {}, 100);
  const id2 = setTimeout(() => {}, 200);
  const id3 = setTimeout(() => {}, 300);
  clearTimeout(id1);
  clearTimeout(id2);
  clearTimeout(id3);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'multiple_timers',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'timers',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
