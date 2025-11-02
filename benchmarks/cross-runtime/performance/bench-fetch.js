// Performance benchmark: Fetch API
// Measures Headers, Request, and Response object performance
// Note: Actual fetch() network calls are not benchmarked as they depend on network conditions

const ITERATIONS = 10000;

const results = [];
let totalElapsed = 0;

// Test 1: Headers creation
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const headers = new Headers();
  headers.set('Content-Type', 'application/json');
  headers.set('Authorization', 'Bearer token');
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'headers_creation',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: Headers from object
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const headers = new Headers({
    'Content-Type': 'application/json',
    'Authorization': 'Bearer token',
    'X-Custom-Header': 'value'
  });
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'headers_from_object',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: Request creation
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const request = new Request('https://example.com/api', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ data: 'test' })
  });
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'request_creation',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: Response creation
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const response = new Response(JSON.stringify({ data: 'test' }), {
    status: 200,
    headers: { 'Content-Type': 'application/json' }
  });
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'response_creation',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'fetch',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
