// Performance benchmark: URL operations
// Measures URL parsing performance

const ITERATIONS = 100000;

const results = [];
let totalElapsed = 0;

// Test 1: URL parsing - simple URL
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const url = new URL('https://example.com/path?query=value&foo=bar#hash');
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'parse',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: URL parsing - complex URL with many params
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const url = new URL('https://user:pass@example.com:8080/api/v2/endpoint?param1=value1&param2=value2&param3=value3&param4=value4&param5=value5#section');
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'parse_complex',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: URLSearchParams creation from string
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const params = new URLSearchParams('a=1&b=2&c=3');
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'searchparams_string',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: URLSearchParams creation from object
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const params = new URLSearchParams({a: '1', b: '2', c: '3'});
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'searchparams_object',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 5: URLSearchParams operations (get, set, append)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const params = new URLSearchParams('a=1&b=2');
  params.get('a');
  params.set('c', '3');
  params.append('d', '4');
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'searchparams_operations',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 6: URLSearchParams toString
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const params = new URLSearchParams('a=1&b=2&c=3&d=4');
  params.toString();
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'searchparams_tostring',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 7: URL property access
start = performance.now();
const url = new URL('https://example.com/path?query=value&foo=bar#hash');
for (let i = 0; i < ITERATIONS; i++) {
  const h = url.hostname;
  const p = url.pathname;
  const s = url.search;
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'property_access',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 8: URL property modification
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const url = new URL('https://example.com/path');
  url.pathname = '/newpath';
  url.search = '?new=param';
  url.hash = '#newhash';
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'property_modification',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 9: URL.searchParams interaction
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const url = new URL('https://example.com?a=1&b=2');
  url.searchParams.set('c', '3');
  const val = url.searchParams.get('a');
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'searchparams_via_url',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

console.log(JSON.stringify({
  test: 'url_parse',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
