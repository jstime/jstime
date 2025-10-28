import { readFile } from 'node:fs/promises';

const testFile = './tests/fixtures/fs/test-content.txt';
const data = await readFile(testFile, { encoding: 'utf-8' });
globalThis.testReadFileEncoding = data;
