// Test HMAC with SHA-512
const encoder = new TextEncoder();

const key = await crypto.subtle.generateKey(
    { name: 'HMAC', hash: 'SHA-512' },
    true,
    ['sign', 'verify']
);

const data = encoder.encode('test');
const signature = await crypto.subtle.sign('HMAC', key, data);
const isValid = await crypto.subtle.verify('HMAC', key, signature, data);

if (!isValid) {
    throw new Error('HMAC SHA-512 signature verification failed');
}

console.log('HMAC SHA-512 test passed');
