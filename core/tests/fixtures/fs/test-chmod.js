import { writeFile, chmod, stat, unlink } from 'node:fs/promises';

const testFile = './tests/fixtures/fs/test-chmod-output.txt';
await writeFile(testFile, 'test content');

// Change permissions to 0o644 (rw-r--r--)
try {
  await chmod(testFile, 0o644);
  globalThis.testChmod = true;
} catch (e) {
  // chmod might not be supported on all platforms
  if (e.message.includes('not supported')) {
    globalThis.testChmod = true;
  } else {
    globalThis.testChmod = false;
  }
}

// Cleanup
try {
  await unlink(testFile);
} catch (e) {}
