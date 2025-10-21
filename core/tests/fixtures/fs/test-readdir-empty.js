import { readdir } from 'node:fs/promises';

const testDir = './tests/fixtures/fs/test-readdir-empty';
const files = await readdir(testDir);
// Filter out .gitkeep file
const actualFiles = files.filter(f => f !== '.gitkeep');
globalThis.testReadDirEmpty = actualFiles.length;
