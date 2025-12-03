// Compliance test for Fetch API
// Tests fetch, Headers, Request, and Response
// Note: Actual network requests are not tested as they require network access

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

// Test Headers exists
test('Headers exists', () => {
  if (typeof Headers !== 'function') throw new Error('Headers is not a function');
});

// Test Headers can be constructed
test('Headers can be constructed', () => {
  const headers = new Headers();
  if (typeof headers.get !== 'function') throw new Error('get is not a function');
});

// Test Headers can be constructed from object
test('Headers from object', () => {
  const headers = new Headers({ 'Content-Type': 'application/json' });
  const ct = headers.get('Content-Type');
  if (ct !== 'application/json') throw new Error(`expected 'application/json', got '${ct}'`);
});

// Test Headers set and get
test('Headers set/get', () => {
  const headers = new Headers();
  headers.set('X-Custom', 'value');
  if (headers.get('X-Custom') !== 'value') throw new Error('set/get failed');
});

// Test Headers append
test('Headers append', () => {
  const headers = new Headers();
  headers.append('X-Custom', 'value1');
  headers.append('X-Custom', 'value2');
  const value = headers.get('X-Custom');
  if (!value.includes('value1') || !value.includes('value2')) {
    throw new Error('append did not combine values');
  }
});

// Test Headers has
test('Headers has', () => {
  const headers = new Headers({ 'X-Custom': 'value' });
  if (!headers.has('X-Custom')) throw new Error('has returned false');
  if (headers.has('X-Missing')) throw new Error('has returned true for missing');
});

// Test Headers delete
test('Headers delete', () => {
  const headers = new Headers({ 'X-Custom': 'value' });
  headers.delete('X-Custom');
  if (headers.has('X-Custom')) throw new Error('delete did not remove header');
});

// Test Request exists
test('Request exists', () => {
  if (typeof Request !== 'function') throw new Error('Request is not a function');
});

// Test Request can be constructed
test('Request can be constructed', () => {
  const request = new Request('https://example.com');
  if (request.url !== 'https://example.com/' && request.url !== 'https://example.com') {
    throw new Error(`unexpected url: ${request.url}`);
  }
});

// Test Request with method
test('Request with method', () => {
  const request = new Request('https://example.com', { method: 'POST' });
  if (request.method !== 'POST') throw new Error(`expected 'POST', got '${request.method}'`);
});

// Test Request with headers
test('Request with headers', () => {
  const request = new Request('https://example.com', {
    headers: { 'Content-Type': 'application/json' }
  });
  const ct = request.headers.get('Content-Type');
  if (ct !== 'application/json') throw new Error(`expected 'application/json', got '${ct}'`);
});

// Test Response exists
test('Response exists', () => {
  if (typeof Response !== 'function') throw new Error('Response is not a function');
});

// Test Response can be constructed
test('Response can be constructed', () => {
  const response = new Response('body');
  if (response.ok !== true) throw new Error('expected ok to be true');
});

// Test Response with status
test('Response with status', () => {
  const response = new Response('body', { status: 404 });
  if (response.status !== 404) throw new Error(`expected 404, got ${response.status}`);
  if (response.ok !== false) throw new Error('expected ok to be false');
});

// Test Response with headers
test('Response with headers', () => {
  const response = new Response('body', {
    headers: { 'Content-Type': 'text/plain' }
  });
  const ct = response.headers.get('Content-Type');
  if (ct !== 'text/plain') throw new Error(`expected 'text/plain', got '${ct}'`);
});

// Test fetch exists
test('fetch exists', () => {
  if (typeof fetch !== 'function') throw new Error('fetch is not a function');
});

// Report results
console.log(`Fetch API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
