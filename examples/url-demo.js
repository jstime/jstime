// URL API Demo for jstime
// This example demonstrates URL and URLSearchParams APIs

console.log('=== jstime URL API Demo ===\n');

// 1. Parsing URLs
console.log('1. Parsing URLs:');
const url = new URL('https://user:pass@example.com:8080/path/to/page?query=value&foo=bar#section');
console.log('   Full URL:', url.href);
console.log('   Protocol:', url.protocol);
console.log('   Username:', url.username);
console.log('   Password:', url.password);
console.log('   Hostname:', url.hostname);
console.log('   Port:', url.port);
console.log('   Pathname:', url.pathname);
console.log('   Search:', url.search);
console.log('   Hash:', url.hash);
console.log('   Origin:', url.origin);
console.log();

// 2. Constructing URLs with base
console.log('2. Constructing URLs with base:');
const baseUrl = 'https://api.example.com/v1/';
const endpoint1 = new URL('users', baseUrl);
const endpoint2 = new URL('posts', baseUrl);
const endpoint3 = new URL('/absolute/path', baseUrl);

console.log('   users endpoint:', endpoint1.href);
console.log('   posts endpoint:', endpoint2.href);
console.log('   absolute path:', endpoint3.href);
console.log();

// 3. Modifying URLs
console.log('3. Modifying URLs:');
const modUrl = new URL('https://example.com/old/path');
console.log('   Original:', modUrl.href);

modUrl.pathname = '/new/path';
modUrl.search = '?updated=true';
modUrl.hash = '#new-section';
console.log('   Modified:', modUrl.href);
console.log();

// 4. Working with URLSearchParams
console.log('4. Working with URLSearchParams:');
const params = new URLSearchParams('foo=1&bar=2&foo=3');

console.log('   Get single value:', params.get('foo'));
console.log('   Get all values:', params.getAll('foo'));
console.log('   Has parameter:', params.has('bar'));
console.log();

// 5. Modifying search parameters
console.log('5. Modifying search parameters:');
params.append('baz', '4');
params.set('foo', '10');
params.delete('bar');
console.log('   Modified params:', params.toString());
console.log();

// 6. Iterating over parameters
console.log('6. Iterating over parameters:');
for (const [key, value] of params) {
  console.log(`   ${key} = ${value}`);
}
console.log();

// 7. Using URLSearchParams with URL
console.log('7. Using URLSearchParams with URL:');
const apiUrl = new URL('https://api.example.com/search');
apiUrl.searchParams.append('q', 'javascript');
apiUrl.searchParams.append('limit', '10');
apiUrl.searchParams.append('sort', 'relevance');

console.log('   Final URL:', apiUrl.href);
console.log('   Query string:', apiUrl.search);
console.log();

// 8. Creating URLSearchParams from object
console.log('8. Creating URLSearchParams from object:');
const searchObj = {
  name: 'Alice',
  age: '30',
  city: 'New York'
};

const objParams = new URLSearchParams(searchObj);
console.log('   From object:', objParams.toString());
console.log();

// 9. URL encoding
console.log('9. URL encoding:');
const specialParams = new URLSearchParams();
specialParams.append('message', 'Hello, World!');
specialParams.append('symbols', 'a+b=c&d<>e');

console.log('   Encoded params:', specialParams.toString());
console.log();

console.log('=== URL Demo Complete ===');
