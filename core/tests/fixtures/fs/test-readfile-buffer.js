import { readFile } from 'node:fs/promises';

const testFile = './tests/fixtures/fs/test-readfile.txt';
const data = await readFile(testFile);
globalThis.testReadFileBuffer = data instanceof Uint8Array;
