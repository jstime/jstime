// Test buffer fill
import { Buffer } from 'node:buffer';

const buf = Buffer.alloc(5);
buf.fill(0x42);
globalThis.testBufferFill = buf.every(b => b === 0x42);
