// Test HMAC sign and verify

const encoder = new TextEncoder();

// Generate an HMAC key
const key = await crypto.subtle.generateKey(
  {
    name: 'HMAC',
    hash: 'SHA-256',
  },
  true,
  ['sign', 'verify']
);

// Data to sign
const data = encoder.encode('Hello, World!');

// Sign the data
const signature = await crypto.subtle.sign('HMAC', key, data);

// Verify the signature
const isValid = await crypto.subtle.verify('HMAC', key, signature, data);

if (!isValid) {
  throw new Error('Signature verification failed');
}

// Verify with wrong data should fail
const wrongData = encoder.encode('Different data');
const isInvalid = await crypto.subtle.verify('HMAC', key, signature, wrongData);

if (isInvalid) {
  throw new Error('Signature verification should have failed with wrong data');
}

console.log('HMAC sign/verify test passed');
