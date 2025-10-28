const encoder = new TextEncoder();
const data = encoder.encode('hello');
const hash = await crypto.subtle.digest('SHA-256', data);

// Convert ArrayBuffer to hex string
const hashArray = Array.from(new Uint8Array(hash));
const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');

// Known SHA-256 hash of 'hello'
if (hashHex !== '2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824') {
    throw new Error(`SHA-256 hash mismatch: expected 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824, got ${hashHex}`);
}
