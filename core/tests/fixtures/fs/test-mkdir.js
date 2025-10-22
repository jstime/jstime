import { mkdir, readdir, rmdir } from 'node:fs/promises';

const testDir = './tests/fixtures/fs/test-mkdir-output';
await mkdir(testDir);
const files = await readdir('./tests/fixtures/fs');
globalThis.testMkdir = files.includes('test-mkdir-output');
await rmdir(testDir);
