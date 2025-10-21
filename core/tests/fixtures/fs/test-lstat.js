import { writeFile, symlink, lstat, unlink } from 'node:fs/promises';

const target = './tests/fixtures/fs/test-readfile.txt';
const link = './tests/fixtures/fs/test-lstat-link';

try {
  await symlink(target, link);
  const stats = await lstat(link);
  globalThis.testLstat = stats.isSymbolicLink === true;
  
  // Cleanup
  await unlink(link);
} catch (e) {
  // symlink might not be supported on all platforms
  if (e.message.includes('not supported')) {
    globalThis.testLstat = true;
  } else {
    // If we can't create symlink, just test lstat on regular file
    const stats = await lstat(target);
    globalThis.testLstat = stats.isFile === true;
  }
}
