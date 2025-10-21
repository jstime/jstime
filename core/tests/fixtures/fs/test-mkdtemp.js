import { mkdtemp, rmdir, readdir } from 'node:fs/promises';

const prefix = './tests/fixtures/fs/test-tmp-';
const tmpDir = await mkdtemp(prefix);
globalThis.testMkdtemp = tmpDir.startsWith(prefix);

// Cleanup
try {
  await rmdir(tmpDir);
} catch (e) {}
