import { writeFile, appendFile, readFile, unlink } from 'node:fs/promises';

const testFile = './tests/fixtures/fs/test-appendfile-output.txt';
const initialContent = 'Initial content\n';
const appendedContent = 'Appended content\n';

// Write initial content
await writeFile(testFile, initialContent);

// Append more content
await appendFile(testFile, appendedContent);

// Read and verify
const finalContent = await readFile(testFile, 'utf-8');
globalThis.testAppendFile = finalContent === (initialContent + appendedContent);

// Cleanup
try {
  await unlink(testFile);
} catch (e) {}
