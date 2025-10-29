// Test generateKey for AES-GCM
const key = await crypto.subtle.generateKey(
    { name: 'AES-GCM', length: 256 },
    true,
    ['encrypt', 'decrypt']
);

if (key.type !== 'secret') {
    throw new Error(`Expected key type 'secret', got '${key.type}'`);
}

if (key.extractable !== true) {
    throw new Error('Expected key to be extractable');
}

console.log('AES-GCM key generation test passed');
