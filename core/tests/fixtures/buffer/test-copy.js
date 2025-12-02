// Test buffer copy
import { Buffer } from 'node:buffer';

const buf1 = Buffer.from([1, 2, 3, 4]);
const buf2 = Buffer.alloc(4);
buf1.copy(buf2);
globalThis.testBufferCopy = buf2.equals(buf1);
