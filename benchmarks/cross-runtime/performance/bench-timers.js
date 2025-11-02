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

// Test 3: setTimeout with immediate execution (0ms)
start = performance.now();
let count = 0;
const promises = [];
for (let i = 0; i < ITERATIONS; i++) {
  promises.push(new Promise(resolve => {
    setTimeout(() => {
      count++;
      resolve();
    }, 0);
  }));
}
// Wait for all to complete
Promise.all(promises).then(() => {
  end = performance.now();
  elapsed = end - start;
  totalElapsed += elapsed;
  results.push({
    name: 'setTimeout_execute',
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
});
