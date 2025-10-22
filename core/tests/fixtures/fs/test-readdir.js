import { readdir } from 'node:fs/promises';

const testDir = './tests/fixtures/fs/test-readdir';
const files = await readdir(testDir);
files.sort();
globalThis.testReadDir = files.join(',');
