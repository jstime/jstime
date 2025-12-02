// Test Buffer.from with string
import { Buffer } from 'node:buffer';

const buf = Buffer.from('Hello');
globalThis.testBufferFromString = buf.length === 5 && buf.toString() === 'Hello';
