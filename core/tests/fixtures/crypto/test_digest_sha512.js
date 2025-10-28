const encoder = new TextEncoder();
const data = encoder.encode('hello');
const hash = await crypto.subtle.digest('SHA-512', data);

// Convert ArrayBuffer to hex string
const hashArray = Array.from(new Uint8Array(hash));
const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');

// Known SHA-512 hash of 'hello'
if (hashHex !== '9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043') {
    throw new Error(`SHA-512 hash mismatch: expected 9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043, got ${hashHex}`);
}
