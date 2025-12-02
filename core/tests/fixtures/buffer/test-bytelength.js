// Test Buffer.byteLength
import { Buffer } from 'node:buffer';

const byteLen = Buffer.byteLength('Hello', 'utf8');
globalThis.testBufferByteLength = byteLen === 5;
