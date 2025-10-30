// Compliance test for URL API
// Tests URL and URLSearchParams

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

// Test URL constructor
test('URL exists', () => {
  if (typeof URL !== 'function') throw new Error('URL is not a constructor');
});

// Test URL parsing
test('URL parses a simple URL', () => {
  const url = new URL('https://example.com/path?query=value#hash');
  if (url.protocol !== 'https:') throw new Error('URL protocol parsing failed');
  if (url.hostname !== 'example.com') throw new Error('URL hostname parsing failed');
  if (url.pathname !== '/path') throw new Error('URL pathname parsing failed');
  if (url.search !== '?query=value') throw new Error('URL search parsing failed');
  if (url.hash !== '#hash') throw new Error('URL hash parsing failed');
});

// Test URLSearchParams
test('URLSearchParams exists', () => {
  if (typeof URLSearchParams !== 'function') throw new Error('URLSearchParams is not a constructor');
});

// Test URLSearchParams.get()
test('URLSearchParams.get() works', () => {
  const params = new URLSearchParams('a=1&b=2&c=3');
  if (params.get('b') !== '2') throw new Error('URLSearchParams.get() failed');
});

// Test URLSearchParams.has()
test('URLSearchParams.has() works', () => {
  const params = new URLSearchParams('a=1&b=2');
  if (!params.has('a')) throw new Error('URLSearchParams.has() failed');
  if (params.has('z')) throw new Error('URLSearchParams.has() false positive');
});

// Test URLSearchParams.set()
test('URLSearchParams.set() works', () => {
  const params = new URLSearchParams('a=1');
  params.set('b', '2');
  if (params.get('b') !== '2') throw new Error('URLSearchParams.set() failed');
});

// Report results
console.log(`URL API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
