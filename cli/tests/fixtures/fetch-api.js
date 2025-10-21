// Test the fetch API structure
console.log("Testing fetch API...");

// Test 1: Check that fetch is defined
if (typeof fetch !== 'function') {
  throw new Error('fetch is not defined');
}
console.log("✓ fetch is defined");

// Test 2: Check Headers
if (typeof Headers !== 'function') {
  throw new Error('Headers is not defined');
}
const h = new Headers({'x-test': 'value'});
if (h.get('x-test') !== 'value') {
  throw new Error('Headers.get() failed');
}
console.log("✓ Headers works");

// Test 3: Check Request
if (typeof Request !== 'function') {
  throw new Error('Request is not defined');
}
const req = new Request('https://example.com', {method: 'POST'});
if (req.method !== 'POST') {
  throw new Error('Request method failed');
}
console.log("✓ Request works");

// Test 4: Check Response
if (typeof Response !== 'function') {
  throw new Error('Response is not defined');
}
const resp = new Response('test', {status: 200});
if (resp.status !== 200) {
  throw new Error('Response status failed');
}
console.log("✓ Response works");

// Test 5: Check Response.text() returns a promise
const textPromise = new Response('hello').text();
if (!(textPromise instanceof Promise)) {
  throw new Error('Response.text() should return a Promise');
}
console.log("✓ Response.text() returns Promise");

// Test 6: Check fetch returns a promise
const fetchPromise = fetch('https://example.com');
if (!(fetchPromise instanceof Promise)) {
  throw new Error('fetch() should return a Promise');
}
console.log("✓ fetch() returns Promise");

console.log("All fetch API tests passed!");
