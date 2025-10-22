// Web Cryptography API Demo
// This example demonstrates jstime's implementation of the Web Cryptography API

console.log('=== Web Cryptography API Demo ===\n');

// 1. Generate random values
console.log('1. crypto.getRandomValues() - Generate random bytes');
console.log('   Fills a TypedArray with cryptographically secure random values\n');

const randomBytes = new Uint8Array(8);
crypto.getRandomValues(randomBytes);
console.log('   Random bytes (Uint8Array):', Array.from(randomBytes).join(', '));

const randomInts = new Uint32Array(4);
crypto.getRandomValues(randomInts);
console.log('   Random integers (Uint32Array):', Array.from(randomInts).join(', '));

// 2. Generate UUIDs
console.log('\n2. crypto.randomUUID() - Generate UUID v4');
console.log('   Generates a cryptographically secure random UUID\n');

for (let i = 0; i < 3; i++) {
  console.log(`   UUID ${i + 1}:`, crypto.randomUUID());
}

// 3. Hash data with SHA-256
console.log('\n3. crypto.subtle.digest() - Hash data');
console.log('   Computes cryptographic hash digests\n');

const encoder = new TextEncoder();

// SHA-256 example
console.log('   SHA-256 (32 bytes / 256 bits):');
const message = 'Hello, World!';
const data = encoder.encode(message);
const sha256Hash = await crypto.subtle.digest('SHA-256', data);

// Convert to hex string
const sha256Array = Array.from(new Uint8Array(sha256Hash));
const sha256Hex = sha256Array.map(b => b.toString(16).padStart(2, '0')).join('');
console.log(`   Input: "${message}"`);
console.log(`   Hash:  ${sha256Hex}`);

// SHA-384 example
console.log('\n   SHA-384 (48 bytes / 384 bits):');
const sha384Hash = await crypto.subtle.digest('SHA-384', data);
const sha384Array = Array.from(new Uint8Array(sha384Hash));
const sha384Hex = sha384Array.map(b => b.toString(16).padStart(2, '0')).join('');
console.log(`   Input: "${message}"`);
console.log(`   Hash:  ${sha384Hex}`);

// SHA-512 example
console.log('\n   SHA-512 (64 bytes / 512 bits):');
const sha512Hash = await crypto.subtle.digest('SHA-512', data);
const sha512Array = Array.from(new Uint8Array(sha512Hash));
const sha512Hex = sha512Array.map(b => b.toString(16).padStart(2, '0')).join('');
console.log(`   Input: "${message}"`);
console.log(`   Hash:  ${sha512Hex}`);

// 4. Practical example: Generate a secure token
console.log('\n4. Practical Example: Generate a secure token');
console.log('   Creating a URL-safe token using random values\n');

function generateToken(length) {
  const array = new Uint8Array(length);
  crypto.getRandomValues(array);
  // Convert to base64-like string
  return Array.from(array)
    .map(b => b.toString(36))
    .join('')
    .substring(0, length);
}

console.log('   Token 1:', generateToken(32));
console.log('   Token 2:', generateToken(32));
console.log('   Token 3:', generateToken(32));

// 5. Practical example: File integrity check simulation
console.log('\n5. Practical Example: Data integrity check');
console.log('   Using hash digests to verify data integrity\n');

async function computeChecksum(text) {
  const data = new TextEncoder().encode(text);
  const hashBuffer = await crypto.subtle.digest('SHA-256', data);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}

const document1 = 'This is an important document.';
const document2 = 'This is an important document.';
const document3 = 'This is a modified document.';

const checksum1 = await computeChecksum(document1);
const checksum2 = await computeChecksum(document2);
const checksum3 = await computeChecksum(document3);

console.log('   Document 1 checksum:', checksum1);
console.log('   Document 2 checksum:', checksum2);
console.log('   Document 3 checksum:', checksum3);
console.log('\n   Documents 1 and 2 match:', checksum1 === checksum2);
console.log('   Documents 1 and 3 match:', checksum1 === checksum3);

console.log('\n=== Demo Complete ===');
