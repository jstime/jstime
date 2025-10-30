// Advanced Web Cryptography API Demo
// Demonstrates the new sign/verify, encrypt/decrypt, and key management features

console.log('=== Advanced Web Cryptography API Demo ===\n');

// Helper function to convert ArrayBuffer to hex string
function bufferToHex(buffer) {
  return Array.from(new Uint8Array(buffer))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');
}

// 1. HMAC Signing and Verification
console.log('1. HMAC Digital Signatures');
console.log('   Sign and verify data using HMAC with SHA-256\n');

const encoder = new TextEncoder();

// Generate an HMAC key
const hmacKey = await crypto.subtle.generateKey(
  {
    name: 'HMAC',
    hash: 'SHA-256',
  },
  true, // extractable
  ['sign', 'verify']
);

console.log('   Generated HMAC key');
console.log('   Key type:', hmacKey.type);
console.log('   Extractable:', hmacKey.extractable);

// Sign some data
const message = 'Important message to authenticate';
const messageData = encoder.encode(message);
const signature = await crypto.subtle.sign('HMAC', hmacKey, messageData);

console.log(`   Message: "${message}"`);
console.log('   Signature:', bufferToHex(signature).substring(0, 32) + '...');

// Verify the signature
const isValid = await crypto.subtle.verify('HMAC', hmacKey, signature, messageData);
console.log('   Signature valid:', isValid);

// Try with tampered data
const tamperedData = encoder.encode('Important message to authenticat!');
const isTamperedValid = await crypto.subtle.verify('HMAC', hmacKey, signature, tamperedData);
console.log('   Tampered data valid:', isTamperedValid, '(expected: false)');

// 2. AES-GCM Encryption and Decryption
console.log('\n2. AES-GCM Encryption and Decryption');
console.log('   Encrypt and decrypt data using AES-GCM (256-bit)\n');

// Generate an AES-GCM key
const aesKey = await crypto.subtle.generateKey(
  {
    name: 'AES-GCM',
    length: 256, // 256-bit key
  },
  true, // extractable
  ['encrypt', 'decrypt']
);

console.log('   Generated AES-GCM-256 key');

// Generate a random IV (Initialization Vector)
const iv = crypto.getRandomValues(new Uint8Array(12));
console.log('   IV:', bufferToHex(iv));

// Encrypt some data
const plaintext = 'Secret message that needs encryption';
const plaintextData = encoder.encode(plaintext);

const ciphertext = await crypto.subtle.encrypt(
  {
    name: 'AES-GCM',
    iv: iv,
  },
  aesKey,
  plaintextData
);

console.log(`   Plaintext: "${plaintext}"`);
console.log('   Ciphertext length:', ciphertext.byteLength, 'bytes');
console.log('   Ciphertext:', bufferToHex(ciphertext).substring(0, 32) + '...');

// Decrypt the data
const decrypted = await crypto.subtle.decrypt(
  {
    name: 'AES-GCM',
    iv: iv,
  },
  aesKey,
  ciphertext
);

const decoder = new TextDecoder();
const decryptedText = decoder.decode(decrypted);
console.log('   Decrypted:', `"${decryptedText}"`);
console.log('   Match:', decryptedText === plaintext);

// 3. AES-GCM with Additional Authenticated Data (AAD)
console.log('\n3. AES-GCM with Additional Authenticated Data');
console.log('   Use AAD to authenticate metadata without encrypting it\n');

const metadata = encoder.encode('user:alice,action:transfer');
const sensitiveData = encoder.encode('$1000 to Bob');
const aadIv = crypto.getRandomValues(new Uint8Array(12));

console.log('   Metadata (not encrypted):', decoder.decode(metadata));
console.log('   Sensitive data (encrypted):', decoder.decode(sensitiveData));

const encryptedWithAAD = await crypto.subtle.encrypt(
  {
    name: 'AES-GCM',
    iv: aadIv,
    additionalData: metadata,
  },
  aesKey,
  sensitiveData
);

console.log('   Encrypted with AAD');

// Decrypt with correct AAD
const decryptedWithAAD = await crypto.subtle.decrypt(
  {
    name: 'AES-GCM',
    iv: aadIv,
    additionalData: metadata,
  },
  aesKey,
  encryptedWithAAD
);

console.log('   Decrypted:', decoder.decode(decryptedWithAAD));

// Try decrypting with wrong AAD (will fail)
try {
  const wrongMetadata = encoder.encode('user:alice,action:withdraw');
  await crypto.subtle.decrypt(
    {
      name: 'AES-GCM',
      iv: aadIv,
      additionalData: wrongMetadata,
    },
    aesKey,
    encryptedWithAAD
  );
  console.log('   ❌ Wrong AAD should have failed!');
} catch (e) {
  console.log('   ✓ Decryption with wrong AAD failed (expected)');
}

// 4. Key Import and Export
console.log('\n4. Key Import and Export');
console.log('   Export and re-import cryptographic keys\n');

// Export the AES key
const exportedKey = await crypto.subtle.exportKey('raw', aesKey);
console.log('   Exported key length:', exportedKey.byteLength, 'bytes');
console.log('   Exported key:', bufferToHex(exportedKey).substring(0, 32) + '...');

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

console.log('   Re-imported key');

// Test that the imported key works
const testIv = crypto.getRandomValues(new Uint8Array(12));
const testData = encoder.encode('Test with imported key');
const testEncrypted = await crypto.subtle.encrypt(
  {
    name: 'AES-GCM',
    iv: testIv,
  },
  importedKey,
  testData
);

const testDecrypted = await crypto.subtle.decrypt(
  {
    name: 'AES-GCM',
    iv: testIv,
  },
  importedKey,
  testEncrypted
);

console.log('   Imported key works:', decoder.decode(testDecrypted) === 'Test with imported key');

// 5. Multiple Hash Algorithms with HMAC
console.log('\n5. HMAC with Different Hash Algorithms');
console.log('   Compare signature sizes for SHA-256, SHA-384, and SHA-512\n');

const testMessage = encoder.encode('Test message');

for (const hash of ['SHA-256', 'SHA-384', 'SHA-512']) {
  const key = await crypto.subtle.generateKey(
    { name: 'HMAC', hash },
    false,
    ['sign']
  );
  const sig = await crypto.subtle.sign('HMAC', key, testMessage);
  console.log(`   ${hash}: ${sig.byteLength} bytes`);
}

// 6. Non-extractable Keys
console.log('\n6. Non-extractable Keys');
console.log('   Generate keys that cannot be exported\n');

const nonExtractableKey = await crypto.subtle.generateKey(
  {
    name: 'AES-GCM',
    length: 256,
  },
  false, // not extractable
  ['encrypt', 'decrypt']
);

console.log('   Generated non-extractable key');
console.log('   Key extractable:', nonExtractableKey.extractable);

try {
  await crypto.subtle.exportKey('raw', nonExtractableKey);
  console.log('   ❌ Export should have failed!');
} catch (e) {
  console.log('   ✓ Export failed as expected:', e.message);
}

console.log('\n7. Practical Use Case: Secure Session Token');
console.log('   Generate and sign a session token\n');

// Generate a session ID
const sessionId = crypto.randomUUID();
const sessionData = {
  id: sessionId,
  user: 'alice',
  expires: Date.now() + 3600000, // 1 hour
};

// Convert to bytes
const sessionJson = JSON.stringify(sessionData);
const sessionBytes = encoder.encode(sessionJson);

// Sign the session data
const sessionKey = await crypto.subtle.generateKey(
  { name: 'HMAC', hash: 'SHA-256' },
  false,
  ['sign', 'verify']
);

const sessionSignature = await crypto.subtle.sign('HMAC', sessionKey, sessionBytes);

console.log('   Session:', sessionJson);
console.log('   Signature:', bufferToHex(sessionSignature).substring(0, 32) + '...');

// Later: verify the session
const isSessionValid = await crypto.subtle.verify('HMAC', sessionKey, sessionSignature, sessionBytes);
console.log('   Session valid:', isSessionValid);

console.log('\n=== Demo Complete ===');
