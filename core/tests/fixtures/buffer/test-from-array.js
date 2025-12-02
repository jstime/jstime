// Test Buffer.from with array
import { Buffer } from 'node:buffer';

const buf = Buffer.from([0x48, 0x65, 0x6c, 0x6c, 0x6f]);
globalThis.testBufferFromArray = buf.length === 5 && buf.toString() === 'Hello';
