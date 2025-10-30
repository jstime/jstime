// Test that exporting non-extractable key fails
let errorCaught = false;

try {
    const key = await crypto.subtle.generateKey(
        { name: 'AES-GCM', length: 256 },
        false,  // not extractable
        ['encrypt', 'decrypt']
    );
    await crypto.subtle.exportKey('raw', key);
} catch (e) {
    if (e.message.includes('not extractable')) {
        errorCaught = true;
    }
}

if (!errorCaught) {
    throw new Error('Expected error when exporting non-extractable key');
}

console.log('Non-extractable key export test passed');
