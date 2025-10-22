import { writeFile, readFile } from 'node:fs/promises';

const testFile = './tests/fixtures/fs/test-writefile-output.txt';
const content = 'Hello from writeFile!';
await writeFile(testFile, content, 'utf-8');
const readContent = await readFile(testFile, 'utf-8');
globalThis.testWriteFile = readContent === content;
