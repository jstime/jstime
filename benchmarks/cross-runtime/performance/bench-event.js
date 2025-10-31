// Performance benchmark: Event dispatch operations
// Measures Event and EventTarget performance

const ITERATIONS = 100000;

const target = new EventTarget();
let counter = 0;

// Add a listener that will be called
target.addEventListener('test', () => {
  counter++;
});

const event = new Event('test');

const start = performance.now();

for (let i = 0; i < ITERATIONS; i++) {
  target.dispatchEvent(event);
}

const end = performance.now();
const elapsed = end - start;

console.log(JSON.stringify({
  test: 'event_dispatch',
  iterations: ITERATIONS,
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
}));
