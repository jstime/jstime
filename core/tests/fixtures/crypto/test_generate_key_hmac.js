// Test generateKey for HMAC
const key = await crypto.subtle.generateKey(
    { name: 'HMAC', hash: 'SHA-256' },
    true,
    ['sign', 'verify']
);

if (key.type !== 'secret') {
    throw new Error(`Expected key type 'secret', got '${key.type}'`);
}

if (key.extractable !== true) {
    throw new Error('Expected key to be extractable');
}

console.log('HMAC key generation test passed');
