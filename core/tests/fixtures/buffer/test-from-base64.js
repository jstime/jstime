// Test Buffer.from with base64 encoding
import { Buffer } from 'node:buffer';

const buf = Buffer.from('SGVsbG8=', 'base64');
globalThis.testBufferFromBase64 = buf.length === 5 && buf.toString() === 'Hello';
