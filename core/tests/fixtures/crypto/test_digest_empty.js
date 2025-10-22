const data = new Uint8Array([]);
const hash = await crypto.subtle.digest('SHA-256', data);

const hashArray = Array.from(new Uint8Array(hash));
const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');

// SHA-256 hash of empty string
if (hashHex !== 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855') {
    throw new Error(`SHA-256 hash of empty data mismatch: expected e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855, got ${hashHex}`);
}
