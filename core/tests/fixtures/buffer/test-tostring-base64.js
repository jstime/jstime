// Test Buffer.toString with base64
import { Buffer } from 'node:buffer';

const buf = Buffer.from('Hello');
globalThis.testBufferToStringBase64 = buf.toString('base64') === 'SGVsbG8=';
