import { readFile } from 'node:fs/promises';

const testFile = './tests/fixtures/fs/test-readfile.txt';
const data = await readFile(testFile, 'utf-8');
globalThis.testReadFileString = data;
