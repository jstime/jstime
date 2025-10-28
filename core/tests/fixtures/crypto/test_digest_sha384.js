const encoder = new TextEncoder();
const data = encoder.encode('hello');
const hash = await crypto.subtle.digest('SHA-384', data);

// Convert ArrayBuffer to hex string
const hashArray = Array.from(new Uint8Array(hash));
const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');

// Known SHA-384 hash of 'hello'
if (hashHex !== '59e1748777448c69de6b800d7a33bbfb9ff1b463e44354c3553bcdb9c666fa90125a3c79f90397bdf5f6a13de828684f') {
    throw new Error(`SHA-384 hash mismatch: expected 59e1748777448c69de6b800d7a33bbfb9ff1b463e44354c3553bcdb9c666fa90125a3c79f90397bdf5f6a13de828684f, got ${hashHex}`);
}
