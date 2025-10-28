import { writeFile, chown, unlink } from 'node:fs/promises';

const testFile = './tests/fixtures/fs/test-chown-output.txt';
await writeFile(testFile, 'test content');

try {
  // Try to change ownership (this may fail without permissions)
  await chown(testFile, 1000, 1000);
  globalThis.testChown = true;
} catch (e) {
  // chown might not be supported or lack permissions
  if (e.message.includes('not supported') || e.message.includes('permission') || e.message.includes('Operation not permitted')) {
    globalThis.testChown = true;
  } else {
    globalThis.testChown = false;
  }
}

// Cleanup
try {
  await unlink(testFile);
} catch (e) {}
