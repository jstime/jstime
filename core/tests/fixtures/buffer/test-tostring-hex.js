// Test Buffer.toString with hex
import { Buffer } from 'node:buffer';

const buf = Buffer.from('Hello');
globalThis.testBufferToStringHex = buf.toString('hex') === '48656c6c6f';
