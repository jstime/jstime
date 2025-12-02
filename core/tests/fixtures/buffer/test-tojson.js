// Test Buffer.toJSON
import { Buffer } from 'node:buffer';

const buf = Buffer.from([1, 2, 3]);
const json = buf.toJSON();
globalThis.testBufferToJSON = json.type === 'Buffer' && JSON.stringify(json.data) === '[1,2,3]';
