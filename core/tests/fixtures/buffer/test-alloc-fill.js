// Test Buffer.alloc with fill
import { Buffer } from 'node:buffer';

const buf = Buffer.alloc(5, 0x41);
globalThis.testBufferAllocFill = buf.length === 5 && buf.every(b => b === 0x41);
