# Fetch API Implementation

This document describes the implementation of the WHATWG Fetch API in jstime.

## Overview

The Fetch API provides a modern interface for making HTTP requests in JavaScript. This implementation includes:

- **Headers**: Interface for working with HTTP headers
- **Request**: Represents an HTTP request
- **Response**: Represents an HTTP response
- **fetch()**: Global function for making HTTP requests

## API Support

### Headers

The `Headers` class provides methods to work with HTTP headers:

```javascript
const headers = new Headers();
headers.append('Content-Type', 'application/json');
headers.set('Authorization', 'Bearer token');
headers.get('content-type'); // 'application/json' (case-insensitive)
headers.has('authorization'); // true
headers.delete('authorization');

// Iterate over headers
for (const [key, value] of headers) {
  console.log(key, value);
}
```

### Request

The `Request` class represents an HTTP request:

```javascript
const request = new Request('https://example.com/api', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({ data: 'value' })
});

console.log(request.url);    // 'https://example.com/api'
console.log(request.method); // 'POST'
```

### Response

The `Response` class represents an HTTP response:

```javascript
const response = new Response('{"key":"value"}', {
  status: 200,
  statusText: 'OK',
  headers: { 'Content-Type': 'application/json' }
});

console.log(response.ok);     // true (status 200-299)
console.log(response.status); // 200

// Parse response body
const text = await response.text();
const json = await response.json();
```

### fetch()

The global `fetch()` function makes HTTP requests:

```javascript
fetch('https://example.com/api')
  .then(response => response.json())
  .then(data => console.log(data))
  .catch(error => console.error(error));
```

## Implementation Details

### Architecture

The implementation consists of three parts:

1. **JavaScript Layer** (`core/src/builtins/fetch.js`)
   - Implements the Headers, Request, Response classes
   - Provides the fetch() function
   - Handles promise creation and data marshalling

2. **Rust Bindings** (`core/src/builtins/mod.rs`)
   - `fetch_send()`: Native binding that creates a promise
   - Queues fetch requests for execution

3. **Event Loop Integration** (`core/src/event_loop.rs`)
   - Processes pending fetch requests
   - Uses `ureq` HTTP client to execute requests
   - Resolves promises with response data

### Promise Handling

Fetch requests are asynchronous and return promises:

1. `fetch()` is called from JavaScript
2. A Promise is created using V8's PromiseResolver
3. The fetch request is queued in `pending_fetches`
4. The event loop processes the request
5. The HTTP request is executed using `ureq`
6. The promise is resolved with the response data

### Supported HTTP Methods

- GET
- POST
- PUT
- DELETE
- HEAD
- PATCH

### Response Body Methods

- `text()`: Returns the body as a text string
- `json()`: Parses the body as JSON

## Limitations

- No streaming support (entire response body is loaded into memory)
- Limited to basic fetch options (no advanced features like credentials, cache control)
- Network access depends on the environment

## Testing

Comprehensive tests are available in `core/tests/test_fetch.rs`:

```bash
cargo test --test test_fetch
```

## Example Usage

```javascript
// Simple GET request
fetch('https://api.example.com/data')
  .then(response => {
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return response.json();
  })
  .then(data => console.log(data))
  .catch(error => console.error('Fetch error:', error));

// POST request with JSON body
fetch('https://api.example.com/users', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({ name: 'Alice', email: 'alice@example.com' })
})
  .then(response => response.json())
  .then(data => console.log('Created:', data))
  .catch(error => console.error('Error:', error));
```

## Dependencies

- `ureq`: Rust HTTP client library for making HTTP requests
