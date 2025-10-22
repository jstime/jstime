// Fetch API Demo for jstime
// This example demonstrates the Fetch API for making HTTP requests

console.log('=== jstime Fetch API Demo ===\n');

// 1. Simple GET request
console.log('1. Simple GET request:');
try {
  const response = await fetch('https://api.github.com/repos/jstime/jstime');
  console.log('   Status:', response.status, response.statusText);
  console.log('   OK:', response.ok);
  
  const data = await response.json();
  console.log('   Repository:', data.name);
  console.log('   Description:', data.description);
  console.log('   Stars:', data.stargazers_count);
} catch (error) {
  console.log('   Error:', error.message);
}
console.log();

// 2. Working with Headers
console.log('2. Working with Headers:');
const headers = new Headers();
headers.append('Accept', 'application/json');
headers.set('User-Agent', 'jstime-example');

console.log('   Headers created:');
for (const [key, value] of headers) {
  console.log(`     ${key}: ${value}`);
}
console.log();

// 3. Using Request object
console.log('3. Using Request object:');
const request = new Request('https://api.github.com/zen', {
  method: 'GET',
  headers: { 'Accept': 'text/plain' }
});

console.log('   Request URL:', request.url);
console.log('   Request method:', request.method);

try {
  const response = await fetch(request);
  const zen = await response.text();
  console.log('   GitHub Zen:', zen);
} catch (error) {
  console.log('   Error:', error.message);
}
console.log();

// 4. Checking response status
console.log('4. Response status handling:');
try {
  const response = await fetch('https://httpbin.org/status/404');
  console.log('   Status:', response.status);
  console.log('   OK:', response.ok);
  
  if (!response.ok) {
    console.log('   ✓ Correctly detected non-OK response');
  }
} catch (error) {
  console.log('   Error:', error.message);
}
console.log();

// 5. Response methods
console.log('5. Response parsing methods:');
try {
  // JSON response
  const jsonResponse = await fetch('https://httpbin.org/json');
  const jsonData = await jsonResponse.json();
  console.log('   JSON data type:', typeof jsonData);
  
  // Text response
  const textResponse = await fetch('https://httpbin.org/html');
  const textData = await textResponse.text();
  console.log('   Text data length:', textData.length, 'characters');
} catch (error) {
  console.log('   Error:', error.message);
}
console.log();

console.log('=== Fetch Demo Complete ===');
