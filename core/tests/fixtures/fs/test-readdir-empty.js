import { readdir } from 'node:fs/promises';

const testDir = './tests/fixtures/fs/test-readdir-empty';
const files = await readdir(testDir);
globalThis.testReadDirEmpty = files.length;
