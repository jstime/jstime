// Test buffer indexOf and includes
import { Buffer } from 'node:buffer';

const buf = Buffer.from('Hello World');
const indexOf = buf.indexOf('World') === 6;
const includes = buf.includes('World');
globalThis.testBufferIndexOf = indexOf && includes;
