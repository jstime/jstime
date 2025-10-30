// Test key import and export

// Generate a key
const originalKey = await crypto.subtle.generateKey(
  {
    name: 'AES-GCM',
    length: 256,
  },
  true,
  ['encrypt', 'decrypt']
);

// Export the key
const exportedKey = await crypto.subtle.exportKey('raw', originalKey);

// Import the key
const importedKey = await crypto.subtle.importKey(
  'raw',
  exportedKey,
  {
    name: 'AES-GCM',
  },
  true,
  ['encrypt', 'decrypt']
);

// Test that the imported key works
const encoder = new TextEncoder();
const decoder = new TextDecoder();
const iv = crypto.getRandomValues(new Uint8Array(12));
const plaintext = encoder.encode('Test data');

const encrypted = await crypto.subtle.encrypt(
  {
    name: 'AES-GCM',
    iv: iv,
  },
  importedKey,
  plaintext
);

const decrypted = await crypto.subtle.decrypt(
  {
    name: 'AES-GCM',
    iv: iv,
  },
  importedKey,
  encrypted
);

const decryptedText = decoder.decode(decrypted);
if (decryptedText !== 'Test data') {
  throw new Error(`Imported key failed: expected "Test data", got "${decryptedText}"`);
}

console.log('Key import/export test passed');
