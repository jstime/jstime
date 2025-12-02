// Test Buffer.alloc
import { Buffer } from 'node:buffer';

const buf = Buffer.alloc(10);
globalThis.testBufferAlloc = buf.length === 10 && buf.every(b => b === 0);
