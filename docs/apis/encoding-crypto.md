# Text Encoding and Cryptography APIs

This document describes the text encoding and cryptography APIs implemented in jstime, following WHATWG and W3C standards.

## Table of Contents

- [Text Encoding API](#text-encoding-api)
- [Web Cryptography API](#web-cryptography-api)

## Text Encoding API

jstime implements the [WHATWG Encoding Standard](https://encoding.spec.whatwg.org/), providing `TextEncoder` and `TextDecoder` for encoding and decoding text as UTF-8 bytes.

**📁 Example:** See [examples/text-encoding-demo.js](../../examples/text-encoding-demo.js) for a complete demonstration.

### Supported APIs

- `TextEncoder` - Encode strings to UTF-8 bytes
- `TextDecoder` - Decode UTF-8 bytes to strings

### TextEncoder

The `TextEncoder` class represents an encoder for UTF-8 encoding. It takes a stream of code points as input and emits a stream of UTF-8 bytes.

#### Constructor

```javascript
new TextEncoder()
```

The `TextEncoder` constructor takes no arguments and always encodes to UTF-8.

#### Properties

- `encoding` (read-only) - Always returns `"utf-8"`

#### Methods

- `encode(input)` - Encodes a string into a `Uint8Array` of UTF-8 bytes
  - `input` (string, optional) - The string to encode. Defaults to empty string.
  - Returns: `Uint8Array` containing the UTF-8 encoded bytes

- `encodeInto(source, destination)` - Encodes a string into an existing `Uint8Array`
  - `source` (string) - The string to encode
  - `destination` (Uint8Array) - The destination buffer to write to
  - Returns: `{ read: number, written: number }` object indicating how many UTF-16 code units were read and how many UTF-8 bytes were written

### TextDecoder

The `TextDecoder` class represents a decoder for UTF-8 encoded text. It takes a stream of UTF-8 bytes as input and emits a stream of code points.

#### Constructor

```javascript
new TextDecoder(label = 'utf-8', options = {})
```

**Parameters:**
- `label` (string, optional) - The encoding label. Only `"utf-8"`, `"utf8"`, and `"unicode-1-1-utf-8"` are supported. Defaults to `"utf-8"`.
- `options` (object, optional) - Currently accepts but ignores `fatal` and `ignoreBOM` options for compatibility

#### Properties

- `encoding` (read-only) - Returns `"utf-8"`
- `fatal` (read-only) - Returns the value set in the constructor options
- `ignoreBOM` (read-only) - Returns the value set in the constructor options

#### Methods

- `decode(input, options)` - Decodes a buffer of UTF-8 bytes into a string
  - `input` (ArrayBuffer | ArrayBufferView, optional) - The bytes to decode
  - `options` (object, optional) - Currently accepts but ignores `stream` option for compatibility
  - Returns: `string` containing the decoded text

### Examples

#### Basic Encoding and Decoding

```javascript
// Create encoder and decoder
const encoder = new TextEncoder();
const decoder = new TextDecoder();

// Encode a string
const text = "Hello, World!";
const encoded = encoder.encode(text);
console.log(encoded); // Uint8Array(13) [72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33]

// Decode the bytes
const decoded = decoder.decode(encoded);
console.log(decoded); // "Hello, World!"
```

#### Encoding UTF-8 Multi-byte Characters

```javascript
const encoder = new TextEncoder();

// Euro sign (3 bytes in UTF-8)
const euro = encoder.encode('€');
console.log(euro); // Uint8Array(3) [226, 130, 172]

// Chinese characters (3 bytes each)
const chinese = encoder.encode('世界');
console.log(chinese); // Uint8Array(6) [228, 184, 150, 231, 149, 140]

// Emoji (4 bytes in UTF-8)
const emoji = encoder.encode('😀');
console.log(emoji); // Uint8Array(4) [240, 159, 152, 128]
```

#### Using encodeInto for Efficient Encoding

```javascript
const encoder = new TextEncoder();

// Pre-allocate a buffer
const buffer = new Uint8Array(100);

// Encode into the buffer
const result = encoder.encodeInto('Hello', buffer);
console.log(result); // { read: 5, written: 5 }

// Check what was written
const written = buffer.slice(0, result.written);
console.log(Array.from(written)); // [72, 101, 108, 108, 111]
```

#### Handling Buffer Overflow with encodeInto

```javascript
const encoder = new TextEncoder();

// Small buffer that can't fit the entire string
const buffer = new Uint8Array(3);

const result = encoder.encodeInto('hello', buffer);
console.log(result); // { read: 3, written: 3 }

// Only 'hel' was written
const decoder = new TextDecoder();
console.log(decoder.decode(buffer)); // "hel"
```

#### Decoding Different Buffer Types

```javascript
const decoder = new TextDecoder();

// Decode from Uint8Array
const uint8 = new Uint8Array([72, 101, 108, 108, 111]);
console.log(decoder.decode(uint8)); // "Hello"

// Decode from ArrayBuffer
const buffer = new Uint8Array([72, 105]).buffer;
console.log(decoder.decode(buffer)); // "Hi"

// Decode from other TypedArray views
const uint16 = new Uint16Array([0x4865, 0x6c6c, 0x6f00]);
const uint8View = new Uint8Array(uint16.buffer, 0, 5);
console.log(decoder.decode(uint8View)); // "Hello"
```

#### Round-trip Encoding and Decoding

```javascript
const encoder = new TextEncoder();
const decoder = new TextDecoder();

// Test with various character sets
const testStrings = [
  'ASCII text',
  'Español',
  '日本語',
  'Emoji: 🌍🌎🌏',
  'Mixed: Hello 世界 🚀',
  'Special chars: \n\t\r\0'
];

testStrings.forEach(original => {
  const encoded = encoder.encode(original);
  const decoded = decoder.decode(encoded);
  console.log(decoded === original); // true for all
});
```

#### Working with File Data

```javascript
import { readFile, writeFile } from 'node:fs/promises';

const encoder = new TextEncoder();
const decoder = new TextDecoder();

// Write text to a file as UTF-8 bytes
const text = 'Hello, World! 🌍';
const bytes = encoder.encode(text);
await writeFile('message.txt', bytes);

// Read UTF-8 bytes from a file and decode
const fileBytes = await readFile('message.txt');
const fileText = decoder.decode(fileBytes);
console.log(fileText); // "Hello, World! 🌍"
```

#### Getting Encoding Information

```javascript
const encoder = new TextEncoder();
const decoder = new TextDecoder();

console.log(encoder.encoding); // "utf-8"
console.log(decoder.encoding); // "utf-8"

// TextEncoder only supports UTF-8
// const encoder2 = new TextEncoder('iso-8859-1'); // Would still be UTF-8

// TextDecoder only supports UTF-8 in jstime
// const decoder2 = new TextDecoder('iso-8859-1'); // Throws RangeError
```

### Use Cases

The Text Encoding API is useful for:

- **Binary data processing**: Converting between strings and byte arrays
- **File I/O**: Reading and writing text files with explicit encoding
- **Network communication**: Encoding/decoding text data for transmission
- **WebAssembly**: Passing string data between JavaScript and WebAssembly modules
- **Cryptography**: Preparing text data for hashing or encryption
- **Data serialization**: Converting text to bytes for storage or transmission

### UTF-8 Support

jstime's `TextEncoder` and `TextDecoder` support the full UTF-8 character set:

- ✅ **ASCII** (1 byte): Basic Latin characters
- ✅ **Latin-1 Supplement** (2 bytes): European characters
- ✅ **BMP** (Basic Multilingual Plane, 2-3 bytes): Most modern scripts
- ✅ **Supplementary Planes** (4 bytes): Emoji, historic scripts, rare characters

```javascript
const encoder = new TextEncoder();

// 1 byte (ASCII)
console.log(encoder.encode('A').length); // 1

// 2 bytes (Latin-1 Supplement)
console.log(encoder.encode('ñ').length); // 2

// 3 bytes (BMP)
console.log(encoder.encode('€').length); // 3
console.log(encoder.encode('世').length); // 3

// 4 bytes (Supplementary Planes)
console.log(encoder.encode('😀').length); // 4
```

### Comparison with Base64

While both deal with text encoding, they serve different purposes:

| Feature | Text Encoding API | Base64 (btoa/atob) |
|---------|------------------|-------------------|
| **Purpose** | Convert between strings and UTF-8 bytes | Convert binary data to ASCII text |
| **Input** | Any Unicode string | Latin-1 strings (0-255) only |
| **Output** | Uint8Array (bytes) | ASCII string |
| **Character support** | Full Unicode (UTF-8) | Latin-1 only |
| **Use case** | Binary data processing | Text-safe binary transmission |

```javascript
const encoder = new TextEncoder();

// Text Encoding: Unicode → UTF-8 bytes
const utf8Bytes = encoder.encode('Hello 世界');
console.log(utf8Bytes); // Uint8Array with UTF-8 bytes

// Base64: Latin-1 string → Base64 ASCII
const base64 = btoa('Hello');
console.log(base64); // "SGVsbG8="

// Base64 cannot handle Unicode directly
// btoa('世界'); // Error!
```

## Web Cryptography API

jstime implements a subset of the [W3C Web Cryptography API](https://w3c.github.io/webcrypto/), providing cryptographically secure operations for generating random values and hashing data.

### Supported APIs

- `crypto.getRandomValues(typedArray)` - Fill a TypedArray with cryptographically strong random values
- `crypto.randomUUID()` - Generate a random UUID v4 string
- `crypto.subtle.digest(algorithm, data)` - Compute a hash digest

### crypto.getRandomValues()

Fills a TypedArray with cryptographically secure random values. The array is modified in-place and also returned.

**Parameters:**
- `typedArray` (TypedArray) - An integer-based TypedArray (Uint8Array, Uint16Array, Uint32Array, etc.)
  - Maximum size: 65,536 bytes

**Returns:** The same TypedArray, filled with random values

**Throws:**
- `TypeError` - If the argument is not a TypedArray
- `Error` - If the array exceeds 65,536 bytes

#### Examples

```javascript
// Fill Uint8Array with random bytes
const array = new Uint8Array(16);
crypto.getRandomValues(array);
console.log(array); // Uint8Array(16) [random values]

// Works with other TypedArrays
const uint32 = new Uint32Array(4);
crypto.getRandomValues(uint32);

// Returns the same array
const result = crypto.getRandomValues(new Uint8Array(32));
console.log(result.length); // 32
```

### crypto.randomUUID()

Generates a cryptographically secure random UUID (Universally Unique Identifier) v4 string.

**Returns:** A string containing a random UUID in the format `xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx`

#### Examples

```javascript
// Generate a UUID
const uuid = crypto.randomUUID();
console.log(uuid); // e.g., "a82be31c-b35d-4f88-8c5e-d5e8f3b0c2a1"

// Each call generates a unique UUID
const uuid1 = crypto.randomUUID();
const uuid2 = crypto.randomUUID();
console.log(uuid1 !== uuid2); // true

// UUID v4 format verification
const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/;
console.log(uuidRegex.test(crypto.randomUUID())); // true
```

### crypto.subtle.digest()

Computes a cryptographic hash digest of the provided data. This is an asynchronous operation that returns a Promise.

**Parameters:**
- `algorithm` (string or object) - The hash algorithm to use:
  - `"SHA-256"` - SHA-256 (256-bit hash)
  - `"SHA-384"` - SHA-384 (384-bit hash)
  - `"SHA-512"` - SHA-512 (512-bit hash)
  - Or an object with a `name` property: `{ name: "SHA-256" }`
- `data` (ArrayBuffer or ArrayBufferView) - The data to hash

**Returns:** Promise<ArrayBuffer> - Resolves with the hash digest as an ArrayBuffer

**Throws:**
- `TypeError` - If the algorithm is invalid
- `Error` - If the algorithm is not supported
- `TypeError` - If data is not an ArrayBuffer or ArrayBufferView

#### Examples

##### Basic SHA-256 Hash

```javascript
const encoder = new TextEncoder();
const data = encoder.encode('hello world');

const hashBuffer = await crypto.subtle.digest('SHA-256', data);

// Convert to hex string
const hashArray = Array.from(new Uint8Array(hashBuffer));
const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
console.log(hashHex); // b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
```

##### SHA-384 Hash

```javascript
const data = new TextEncoder().encode('hello');
const hash = await crypto.subtle.digest('SHA-384', data);
console.log(hash.byteLength); // 48 (384 bits = 48 bytes)
```

##### SHA-512 Hash

```javascript
const data = new TextEncoder().encode('hello');
const hash = await crypto.subtle.digest('SHA-512', data);
console.log(hash.byteLength); // 64 (512 bits = 64 bytes)
```

##### Hash with ArrayBuffer

```javascript
// Can use ArrayBuffer directly
const buffer = new Uint8Array([104, 101, 108, 108, 111]).buffer; // 'hello'
const hash = await crypto.subtle.digest('SHA-256', buffer);
```

##### Algorithm Object Syntax

```javascript
// Can pass algorithm as object
const data = new TextEncoder().encode('test');
const hash = await crypto.subtle.digest({ name: 'SHA-256' }, data);
```

##### Hash Empty Data

```javascript
// Hashing empty data is supported
const empty = new Uint8Array([]);
const hash = await crypto.subtle.digest('SHA-256', empty);

// Convert to hex
const hashArray = Array.from(new Uint8Array(hash));
const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
console.log(hashHex); // e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
```

##### Practical Example: File Integrity Check

```javascript
import { readFile } from 'node:fs/promises';

// Read file as buffer
const fileData = await readFile('./document.pdf');

// Compute SHA-256 hash
const hashBuffer = await crypto.subtle.digest('SHA-256', fileData);

// Convert to hex string for comparison
const hashArray = Array.from(new Uint8Array(hashBuffer));
const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');

console.log('File SHA-256:', hashHex);
```

##### Practical Example: Password Hashing Check

```javascript
// Note: For actual password hashing, use a proper password hashing function like bcrypt or argon2
// This is just an example of basic hashing

async function hashPassword(password) {
  const encoder = new TextEncoder();
  const data = encoder.encode(password);
  const hashBuffer = await crypto.subtle.digest('SHA-256', data);
  
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}

const password = 'my-secure-password';
const hash = await hashPassword(password);
console.log('Password hash:', hash);
```

### Use Cases

The Web Cryptography API is useful for:

- **Random number generation**: Generate cryptographically secure random values for tokens, IDs, or cryptographic operations
- **UUID generation**: Create unique identifiers for resources, sessions, or tracking
- **Data integrity**: Verify file or data integrity using hash digests
- **Fingerprinting**: Create unique fingerprints of data for comparison or caching
- **Non-reversible data storage**: Store hashed data (though use proper password hashing for passwords)

### Security Notes

- **Random values**: All random values are generated using a cryptographically secure random number generator (ring's `SystemRandom`)
- **Hash algorithms**: SHA-256, SHA-384, and SHA-512 are cryptographically secure hash functions
- **Password hashing**: For password hashing, consider using specialized password hashing functions (like bcrypt, scrypt, or argon2) instead of simple SHA hashing
- **HTTPS**: When transmitting sensitive data, always use HTTPS

### Supported Hash Algorithms

| Algorithm | Output Size | Use Case |
|-----------|-------------|----------|
| SHA-256   | 256 bits (32 bytes) | General purpose, most common |
| SHA-384   | 384 bits (48 bytes) | Higher security than SHA-256 |
| SHA-512   | 512 bits (64 bytes) | Maximum security for hash digests |

### Comparison with Node.js crypto

While jstime's crypto API is based on the Web Cryptography API standard, Node.js uses its own `crypto` module. Here are the key differences:

| Feature | jstime (Web Crypto API) | Node.js crypto |
|---------|------------------------|----------------|
| **API Standard** | W3C Web Cryptography API | Node.js-specific API |
| **getRandomValues** | ✅ Supported | ❌ Use `crypto.randomBytes()` instead |
| **randomUUID** | ✅ Supported | ✅ Supported (crypto.randomUUID()) |
| **subtle.digest** | ✅ Supported | ✅ Supported |
| **Hash algorithms** | SHA-256, SHA-384, SHA-512 | Many more (MD5, SHA-1, etc.) |
| **Encryption** | ❌ Not yet supported | ✅ Supported |
| **Key generation** | ❌ Not yet supported | ✅ Supported |

### Future Enhancements

Potential additions being considered:

- Additional SubtleCrypto methods (encrypt, decrypt, sign, verify)
- Key generation and management
- Additional hash algorithms
- HMAC support
- AES encryption/decryption
