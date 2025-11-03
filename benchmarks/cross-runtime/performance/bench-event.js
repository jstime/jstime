// Performance benchmark: Event dispatch operations
// Measures Event and EventTarget performance

const ITERATIONS = 100000;

const results = [];
let totalElapsed = 0;

// Test 1: Event creation - simple
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const event = new Event('test');
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'creation_simple',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: Event creation - with options
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const event = new Event('test', { bubbles: true, cancelable: true });
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'creation_with_options',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: EventTarget creation
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const target = new EventTarget();
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'target_creation',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: EventTarget addEventListener - single listener
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const target = new EventTarget();
  target.addEventListener('test', () => {});
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'addEventListener_single',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 5: EventTarget addEventListener - multiple listeners
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const target = new EventTarget();
  target.addEventListener('test', () => {});
  target.addEventListener('test', () => {});
  target.addEventListener('test', () => {});
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'addEventListener_multiple',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 6: EventTarget removeEventListener
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const target = new EventTarget();
  const listener = () => {};
  target.addEventListener('test', listener);
  target.removeEventListener('test', listener);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'removeEventListener',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 7: Event dispatch - single listener
start = performance.now();
const target1 = new EventTarget();
let counter1 = 0;
target1.addEventListener('test', () => {
  counter1++;
});
const event1 = new Event('test');
for (let i = 0; i < ITERATIONS; i++) {
  target1.dispatchEvent(event1);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'dispatch_single_listener',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 8: Event dispatch - multiple listeners
start = performance.now();
const target2 = new EventTarget();
let counter2 = 0;
target2.addEventListener('test', () => { counter2++; });
target2.addEventListener('test', () => { counter2++; });
target2.addEventListener('test', () => { counter2++; });
const event2 = new Event('test');
for (let i = 0; i < ITERATIONS; i++) {
  target2.dispatchEvent(event2);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'dispatch_multiple_listeners',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 9: Event dispatch - with new event each time
start = performance.now();
const target3 = new EventTarget();
let counter3 = 0;
target3.addEventListener('test', () => {
  counter3++;
});
for (let i = 0; i < ITERATIONS; i++) {
  target3.dispatchEvent(new Event('test'));
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'dispatch_new_event',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Use counters to prevent dead code elimination
if (counter1 !== ITERATIONS) {
  throw new Error(`Expected ${ITERATIONS} events, got ${counter1}`);
}
if (counter2 !== ITERATIONS * 3) {
  throw new Error(`Expected ${ITERATIONS * 3} events, got ${counter2}`);
}
if (counter3 !== ITERATIONS) {
  throw new Error(`Expected ${ITERATIONS} events, got ${counter3}`);
}

console.log(JSON.stringify({
  test: 'event_dispatch',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
