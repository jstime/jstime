import { writeFile, truncate, stat, unlink } from 'node:fs/promises';

const testFile = './tests/fixtures/fs/test-truncate-output.txt';
await writeFile(testFile, 'This is a long test content that will be truncated');

// Truncate to 10 bytes
await truncate(testFile, 10);

const stats = await stat(testFile);
globalThis.testTruncate = stats.size === 10;

// Cleanup
try {
  await unlink(testFile);
} catch (e) {}
