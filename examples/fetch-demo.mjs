// Fetch API Demo for jstime
// This example demonstrates the Fetch API for making HTTP requests
// Note: This example requires network access and may fail in some environments

console.log('=== jstime Fetch API Demo ===\n');

// 1. Simple GET request
console.log('1. Simple GET request:');
fetch('https://api.github.com/repos/jstime/jstime')
  .then(response => {
    console.log('   Status:', response.status, response.statusText);
    console.log('   OK:', response.ok);
    return response.json();
  })
  .then(data => {
    console.log('   Repository:', data.name);
    console.log('   Description:', data.description);
    console.log('   Stars:', data.stargazers_count);
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

    return fetch(request);
  })
  .then(response => response.text())
  .then(zen => {
    console.log('   GitHub Zen:', zen);
    console.log();
    
    // 4. Checking response status
    console.log('4. Response status handling:');
    return fetch('https://httpbin.org/status/404');
  })
  .then(response => {
    console.log('   Status:', response.status);
    console.log('   OK:', response.ok);
    
    if (!response.ok) {
      console.log('   ✓ Correctly detected non-OK response');
    }
    console.log();
    
    // 5. Response methods
    console.log('5. Response parsing methods:');
    return fetch('https://httpbin.org/json');
  })
  .then(response => response.json())
  .then(jsonData => {
    console.log('   JSON data type:', typeof jsonData);
    return fetch('https://httpbin.org/html');
  })
  .then(response => response.text())
  .then(textData => {
    console.log('   Text data length:', textData.length, 'characters');
    console.log();
    console.log('=== Fetch Demo Complete ===');
  })
  .catch(error => {
    console.log('   Error:', error.message);
    console.log();
    console.log('=== Fetch Demo Complete (with errors) ===');
  });
