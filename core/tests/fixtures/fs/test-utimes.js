import { writeFile, utimes, stat, unlink } from 'node:fs/promises';

const testFile = './tests/fixtures/fs/test-utimes-output.txt';
await writeFile(testFile, 'test content');

const newTime = Date.now() - 10000; // 10 seconds ago
await utimes(testFile, newTime, newTime);

const stats = await stat(testFile);
const diff = Math.abs(stats.mtimeMs - newTime);
globalThis.testUtimes = diff < 2000; // Allow 2 second tolerance

// Cleanup
try {
  await unlink(testFile);
} catch (e) {}
