import { writeFile, copyFile, readFile, unlink } from 'node:fs/promises';

const srcPath = './tests/fixtures/fs/test-copy-src.txt';
const destPath = './tests/fixtures/fs/test-copy-dest.txt';
const content = 'copied content';

await writeFile(srcPath, content);
await copyFile(srcPath, destPath);
const readContent = await readFile(destPath, 'utf-8');
globalThis.testCopyFile = readContent === content;

// Cleanup
try {
  await unlink(srcPath);
  await unlink(destPath);
} catch (e) {}
