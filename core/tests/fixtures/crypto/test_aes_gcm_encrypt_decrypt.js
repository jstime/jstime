// Test AES-GCM encryption and decryption

const encoder = new TextEncoder();
const decoder = new TextDecoder();

// Generate an AES-GCM key
const key = await crypto.subtle.generateKey(
  {
    name: 'AES-GCM',
    length: 256,
  },
  true,
  ['encrypt', 'decrypt']
);

// Generate a random IV (12 bytes for AES-GCM)
const iv = crypto.getRandomValues(new Uint8Array(12));

// Data to encrypt
const plaintext = encoder.encode('Secret message');

// Encrypt the data
const ciphertext = await crypto.subtle.encrypt(
  {
    name: 'AES-GCM',
    iv: iv,
  },
  key,
  plaintext
);

// Decrypt the data
const decrypted = await crypto.subtle.decrypt(
  {
    name: 'AES-GCM',
    iv: iv,
  },
  key,
  ciphertext
);

// Verify decryption
const decryptedText = decoder.decode(decrypted);
if (decryptedText !== 'Secret message') {
  throw new Error(`Decryption failed: expected "Secret message", got "${decryptedText}"`);
}

console.log('AES-GCM encrypt/decrypt test passed');
