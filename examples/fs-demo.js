// Demo: Node.js fs/promises API in jstime
// This example demonstrates the minimal Node.js fs API implementation

import { readFile, readdir } from 'node:fs/promises';

console.log('=== jstime Node.js fs API Demo ===\n');

// Example 1: Reading a file as text (UTF-8)
console.log('1. Reading file as text:');
const readmeText = await readFile('./README.md', 'utf-8');
console.log('   README.md (first 100 chars):', readmeText.substring(0, 100));
console.log();

// Example 2: Reading a file as buffer
console.log('2. Reading file as buffer:');
const readmeBuffer = await readFile('./README.md');
console.log('   Buffer type:', readmeBuffer.constructor.name);
console.log('   Buffer length:', readmeBuffer.length, 'bytes');
console.log();

// Example 3: Reading file with options object
console.log('3. Reading file with options object:');
const licenseText = await readFile('./LICENSE', { encoding: 'utf-8' });
console.log('   LICENSE (first 100 chars):', licenseText.substring(0, 100));
console.log();

// Example 4: Listing directory contents
console.log('4. Listing directory contents:');
const coreFiles = await readdir('./core/src');
console.log('   Files in core/src:', coreFiles.length, 'items');
console.log('   First 5 items:', coreFiles.slice(0, 5).join(', '));
console.log();

// Example 5: Error handling - file not found
console.log('5. Error handling - file not found:');
try {
  await readFile('./nonexistent-file.txt');
} catch (error) {
  console.log('   ✓ Caught error:', error.message);
}
console.log();

// Example 6: Error handling - directory not found
console.log('6. Error handling - directory not found:');
try {
  await readdir('./nonexistent-directory');
} catch (error) {
  console.log('   ✓ Caught error:', error.message);
}
console.log();

console.log('=== Demo Complete ===');
