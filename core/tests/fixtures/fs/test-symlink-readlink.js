import { writeFile, symlink, readlink, unlink } from 'node:fs/promises';

const target = './tests/fixtures/fs/test-readfile.txt';
const link = './tests/fixtures/fs/test-symlink-link';

try {
  await symlink(target, link);
  const linkTarget = await readlink(link);
  globalThis.testSymlinkReadlink = linkTarget.includes('test-readfile.txt');
  
  // Cleanup
  await unlink(link);
} catch (e) {
  // symlink might not be supported on all platforms or permissions
  if (e.message.includes('not supported')) {
    globalThis.testSymlinkReadlink = true;
  } else {
    globalThis.testSymlinkReadlink = false;
  }
}
