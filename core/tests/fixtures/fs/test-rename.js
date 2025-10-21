import { writeFile, rename, readFile } from 'node:fs/promises';

const oldPath = './tests/fixtures/fs/test-rename-old.txt';
const newPath = './tests/fixtures/fs/test-rename-new.txt';
const content = 'renamed content';

await writeFile(oldPath, content);
await rename(oldPath, newPath);
const readContent = await readFile(newPath, 'utf-8');
globalThis.testRename = readContent === content;

// Cleanup
try {
  await import('node:fs/promises').then(fs => fs.unlink(newPath));
} catch (e) {}
