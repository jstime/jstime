// Test Buffer.isEncoding
import { Buffer } from 'node:buffer';

const validEncodings = Buffer.isEncoding('utf8') && Buffer.isEncoding('hex') && Buffer.isEncoding('base64');
const invalidEncoding = !Buffer.isEncoding('invalid');
globalThis.testBufferIsEncoding = validEncodings && invalidEncoding;
