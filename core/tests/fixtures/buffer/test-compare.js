// Test Buffer.compare
import { Buffer } from 'node:buffer';

const buf1 = Buffer.from('ABC');
const buf2 = Buffer.from('ABC');
const buf3 = Buffer.from('ABD');
const cmp1 = Buffer.compare(buf1, buf2) === 0;
const cmp2 = Buffer.compare(buf1, buf3) === -1;
globalThis.testBufferCompare = cmp1 && cmp2;
