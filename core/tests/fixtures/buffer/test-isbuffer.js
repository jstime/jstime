// Test Buffer.isBuffer
import { Buffer } from 'node:buffer';

const buf = Buffer.from('test');
const arr = new Uint8Array(4);
globalThis.testBufferIsBuffer = Buffer.isBuffer(buf) && !Buffer.isBuffer(arr) && !Buffer.isBuffer('test');
