import { writeFile, rm, readdir } from 'node:fs/promises';

// Test removing a file
const testFile = './tests/fixtures/fs/test-rm-file.txt';
await writeFile(testFile, 'test content');
await rm(testFile);

// Try to list directory - file should be gone
const files = await readdir('./tests/fixtures/fs');
globalThis.testRm = !files.includes('test-rm-file.txt');
