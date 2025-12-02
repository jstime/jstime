// Test buffer read/write methods
import { Buffer } from 'node:buffer';

const buf = Buffer.alloc(8);

// Test writeUInt32LE and readUInt32LE
buf.writeUInt32LE(0x12345678, 0);
const readLE = buf.readUInt32LE(0) === 0x12345678;

// Test writeUInt32BE and readUInt32BE
buf.writeUInt32BE(0xABCDEF01, 4);
const readBE = buf.readUInt32BE(4) === 0xABCDEF01;

globalThis.testBufferReadWrite = readLE && readBE;
