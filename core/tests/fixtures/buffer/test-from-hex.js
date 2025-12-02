// Test Buffer.from with hex encoding
import { Buffer } from 'node:buffer';

const buf = Buffer.from('48656c6c6f', 'hex');
globalThis.testBufferFromHex = buf.length === 5 && buf.toString() === 'Hello';
