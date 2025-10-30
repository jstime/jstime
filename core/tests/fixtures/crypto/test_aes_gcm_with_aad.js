// Test AES-GCM with additional authenticated data
const encoder = new TextEncoder();
const decoder = new TextDecoder();

const key = await crypto.subtle.generateKey(
    { name: 'AES-GCM', length: 256 },
    true,
    ['encrypt', 'decrypt']
);

const iv = crypto.getRandomValues(new Uint8Array(12));
const additionalData = encoder.encode('metadata');
const plaintext = encoder.encode('secret');

const ciphertext = await crypto.subtle.encrypt(
    { name: 'AES-GCM', iv: iv, additionalData: additionalData },
    key,
    plaintext
);

const decrypted = await crypto.subtle.decrypt(
    { name: 'AES-GCM', iv: iv, additionalData: additionalData },
    key,
    ciphertext
);

const decryptedText = decoder.decode(decrypted);
if (decryptedText !== 'secret') {
    throw new Error(`Expected 'secret', got '${decryptedText}'`);
}

console.log('AES-GCM with AAD test passed');
