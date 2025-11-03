// Compliance test for Fetch API
// Tests Headers, Request, and Response classes
// Note: Actual fetch() network calls are not tested in compliance tests

let passed = 0;
let failed = 0;

function test(name, fn) {
  try {
    fn();
    passed++;
  } catch (e) {
    console.error(`FAIL: ${name} - ${e.message}`);
    failed++;
  }
}

// Test Headers class
test('Headers class exists', () => {
  if (typeof Headers !== 'function') throw new Error('Headers is not a function');
});

test('Headers creation and basic operations', () => {
  const headers = new Headers();
  headers.set('Content-Type', 'application/json');
  headers.append('X-Custom', 'value');
  if (headers.get('content-type') !== 'application/json') throw new Error('Headers.get() failed');
  if (!headers.has('x-custom')) throw new Error('Headers.has() failed');
});

test('Headers from object', () => {
  const headers = new Headers({
    'Content-Type': 'application/json',
    'Authorization': 'Bearer token'
  });
  if (headers.get('content-type') !== 'application/json') throw new Error('Headers initialization from object failed');
});

// Test Request class
test('Request class exists', () => {
  if (typeof Request !== 'function') throw new Error('Request is not a function');
});

test('Request creation', () => {
  const request = new Request('https://example.com/api', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ data: 'test' })
  });
  if (request.method !== 'POST') throw new Error('Request.method incorrect');
  if (request.url !== 'https://example.com/api') throw new Error('Request.url incorrect');
});

// Test Response class
test('Response class exists', () => {
  if (typeof Response !== 'function') throw new Error('Response is not a function');
});

test('Response creation', () => {
  const response = new Response(JSON.stringify({ data: 'test' }), {
    status: 200,
    statusText: 'OK',
    headers: { 'Content-Type': 'application/json' }
  });
  if (response.status !== 200) throw new Error('Response.status incorrect');
  if (response.statusText !== 'OK') throw new Error('Response.statusText incorrect');
  if (!response.ok) throw new Error('Response.ok should be true for status 200');
});

test('Response.ok property', () => {
  const okResponse = new Response('', { status: 200 });
  const errorResponse = new Response('', { status: 404 });
  if (!okResponse.ok) throw new Error('Response.ok should be true for 200');
  if (errorResponse.ok) throw new Error('Response.ok should be false for 404');
});

// Test fetch function
test('fetch function exists', () => {
  if (typeof fetch !== 'function') throw new Error('fetch is not a function');
});

// Report results
console.log(`Fetch API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
