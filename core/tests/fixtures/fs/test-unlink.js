import { writeFile, unlink, access } from 'node:fs/promises';

const testFile = './tests/fixtures/fs/test-unlink-output.txt';
await writeFile(testFile, 'temp file');
await unlink(testFile);

// Try to access the deleted file - should throw
try {
  await access(testFile);
  globalThis.testUnlink = false;
} catch (e) {
  globalThis.testUnlink = true;
}
