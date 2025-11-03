// Compliance test for File System API (node:fs/promises)
// Tests basic file operations

import * as fs from 'node:fs/promises';

let passed = 0;
let failed = 0;

function test(name, fn) {
  try {
    fn();
    passed++;
  } catch (e) {
    console.error(`FAIL: ${name} - ${e.message}`);
    failed++;
  }
}

// Test basic fs functions exist
test('readFile exists', () => {
  if (typeof fs.readFile !== 'function') throw new Error('fs.readFile is not a function');
});

test('writeFile exists', () => {
  if (typeof fs.writeFile !== 'function') throw new Error('fs.writeFile is not a function');
});

test('appendFile exists', () => {
  if (typeof fs.appendFile !== 'function') throw new Error('fs.appendFile is not a function');
});

test('readdir exists', () => {
  if (typeof fs.readdir !== 'function') throw new Error('fs.readdir is not a function');
});

test('mkdir exists', () => {
  if (typeof fs.mkdir !== 'function') throw new Error('fs.mkdir is not a function');
});

test('rmdir exists', () => {
  if (typeof fs.rmdir !== 'function') throw new Error('fs.rmdir is not a function');
});

test('unlink exists', () => {
  if (typeof fs.unlink !== 'function') throw new Error('fs.unlink is not a function');
});

test('rename exists', () => {
  if (typeof fs.rename !== 'function') throw new Error('fs.rename is not a function');
});

test('copyFile exists', () => {
  if (typeof fs.copyFile !== 'function') throw new Error('fs.copyFile is not a function');
});

test('stat exists', () => {
  if (typeof fs.stat !== 'function') throw new Error('fs.stat is not a function');
});

test('access exists', () => {
  if (typeof fs.access !== 'function') throw new Error('fs.access is not a function');
});

test('rm exists', () => {
  if (typeof fs.rm !== 'function') throw new Error('fs.rm is not a function');
});

test('truncate exists', () => {
  if (typeof fs.truncate !== 'function') throw new Error('fs.truncate is not a function');
});

test('realpath exists', () => {
  if (typeof fs.realpath !== 'function') throw new Error('fs.realpath is not a function');
});

test('constants exists', () => {
  if (typeof fs.constants !== 'object') throw new Error('fs.constants is not an object');
  if (typeof fs.constants.F_OK !== 'number') throw new Error('fs.constants.F_OK is not defined');
  if (typeof fs.constants.R_OK !== 'number') throw new Error('fs.constants.R_OK is not defined');
  if (typeof fs.constants.W_OK !== 'number') throw new Error('fs.constants.W_OK is not defined');
  if (typeof fs.constants.X_OK !== 'number') throw new Error('fs.constants.X_OK is not defined');
});

// Test basic file operations
const testFilePath = '/tmp/jstime-fs-test-' + Date.now() + '.txt';
const testContent = 'Hello, World!';

test('write and read file', async () => {
  await fs.writeFile(testFilePath, testContent, 'utf-8');
  const content = await fs.readFile(testFilePath, 'utf-8');
  if (content !== testContent) throw new Error(`Expected "${testContent}", got "${content}"`);
});

test('stat file', async () => {
  const stats = await fs.stat(testFilePath);
  // Handle both jstime (boolean properties) and Node.js (functions)
  const isFile = typeof stats.isFile === 'function' ? stats.isFile() : stats.isFile;
  const isDirectory = typeof stats.isDirectory === 'function' ? stats.isDirectory() : stats.isDirectory;
  if (!isFile) throw new Error('stat.isFile should be true');
  if (isDirectory) throw new Error('stat.isDirectory should be false');
  if (stats.size !== testContent.length) throw new Error(`Expected size ${testContent.length}, got ${stats.size}`);
});

test('append to file', async () => {
  const appendText = '\nAppended line';
  await fs.appendFile(testFilePath, appendText, 'utf-8');
  const content = await fs.readFile(testFilePath, 'utf-8');
  const expected = testContent + appendText;
  if (content !== expected) throw new Error(`Expected "${expected}", got "${content}"`);
});

test('unlink file', async () => {
  await fs.unlink(testFilePath);
  try {
    await fs.access(testFilePath);
    throw new Error('File should not exist after unlink');
  } catch (e) {
    // Expected to fail
    if (e.message === 'File should not exist after unlink') throw e;
  }
});

// Test directory operations
const testDirPath = '/tmp/jstime-fs-test-dir-' + Date.now();

test('create directory', async () => {
  await fs.mkdir(testDirPath);
  const stats = await fs.stat(testDirPath);
  // Handle both jstime (boolean properties) and Node.js (functions)
  const isDirectory = typeof stats.isDirectory === 'function' ? stats.isDirectory() : stats.isDirectory;
  if (!isDirectory) throw new Error('stat.isDirectory should be true for directory');
});

test('write file in directory', async () => {
  const filePath = testDirPath + '/test.txt';
  await fs.writeFile(filePath, 'test content', 'utf-8');
  const content = await fs.readFile(filePath, 'utf-8');
  if (content !== 'test content') throw new Error('Failed to read file from directory');
});

test('readdir', async () => {
  const files = await fs.readdir(testDirPath);
  if (!Array.isArray(files)) throw new Error('readdir should return an array');
  if (files.length !== 1) throw new Error(`Expected 1 file, got ${files.length}`);
  if (files[0] !== 'test.txt') throw new Error(`Expected 'test.txt', got '${files[0]}'`);
});

test('remove directory recursively', async () => {
  await fs.rmdir(testDirPath, { recursive: true });
  try {
    await fs.access(testDirPath);
    throw new Error('Directory should not exist after recursive rmdir');
  } catch (e) {
    // Expected to fail
    if (e.message === 'Directory should not exist after recursive rmdir') throw e;
  }
});

// Report results
console.log(`File System API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
