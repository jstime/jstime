// Test Buffer.concat
import { Buffer } from 'node:buffer';

const buf1 = Buffer.from('Hello');
const buf2 = Buffer.from(' World');
const buf3 = Buffer.concat([buf1, buf2]);
globalThis.testBufferConcat = buf3.toString() === 'Hello World';
