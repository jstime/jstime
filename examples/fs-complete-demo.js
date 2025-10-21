// Demo: Complete Node.js fs/promises API in jstime
// This example demonstrates all implemented filesystem operations

import { 
  readFile, 
  writeFile, 
  appendFile,
  readdir, 
  mkdir, 
  rmdir, 
  unlink, 
  rename, 
  copyFile, 
  stat, 
  access,
  constants 
} from 'node:fs/promises';

console.log('=== jstime Complete Node.js fs API Demo ===\n');

// 1. Writing Files
console.log('1. Writing files:');
await writeFile('./test-demo-file.txt', 'Hello, jstime fs API!', 'utf-8');
console.log('   ✓ Wrote test-demo-file.txt');

// 2. Appending to Files
console.log('\n2. Appending to files:');
await appendFile('./test-demo-file.txt', '\nAppended line!', 'utf-8');
console.log('   ✓ Appended to test-demo-file.txt');

// 3. Reading Files
console.log('\n3. Reading files:');
const content = await readFile('./test-demo-file.txt', 'utf-8');
console.log('   ✓ Read content:', content);

// 4. File Statistics
console.log('\n4. File statistics:');
const stats = await stat('./test-demo-file.txt');
console.log('   ✓ File size:', stats.size, 'bytes');
console.log('   ✓ Is file:', stats.isFile);
console.log('   ✓ Is directory:', stats.isDirectory);

// 5. File Access Check
console.log('\n5. File access:');
try {
  await access('./test-demo-file.txt', constants.F_OK);
  console.log('   ✓ File exists and is accessible');
} catch (e) {
  console.log('   ✗ File not accessible');
}

// 6. Copying Files
console.log('\n6. Copying files:');
await copyFile('./test-demo-file.txt', './test-demo-copy.txt');
console.log('   ✓ Copied to test-demo-copy.txt');

// 7. Renaming Files
console.log('\n7. Renaming files:');
await rename('./test-demo-copy.txt', './test-demo-renamed.txt');
console.log('   ✓ Renamed to test-demo-renamed.txt');

// 8. Creating Directories
console.log('\n8. Creating directories:');
await mkdir('./test-demo-dir');
console.log('   ✓ Created test-demo-dir/');

// 9. Listing Directory Contents
console.log('\n9. Listing directories:');
const files = await readdir('./');
const demoFiles = files.filter(f => f.startsWith('test-demo'));
console.log('   ✓ Demo files:', demoFiles.join(', '));

// 10. Creating Nested Directories
console.log('\n10. Creating nested directories:');
await mkdir('./test-demo-dir/nested/deep', { recursive: true });
console.log('   ✓ Created nested directory structure');

// 11. Deleting Files
console.log('\n11. Deleting files:');
await unlink('./test-demo-file.txt');
await unlink('./test-demo-renamed.txt');
console.log('   ✓ Deleted test files');

// 12. Removing Directories
console.log('\n12. Removing directories:');
await rmdir('./test-demo-dir', { recursive: true });
console.log('   ✓ Removed test-demo-dir/ and all contents');

// 13. Constants
console.log('\n13. File system constants:');
console.log('   ✓ F_OK (exists):', constants.F_OK);
console.log('   ✓ R_OK (readable):', constants.R_OK);
console.log('   ✓ W_OK (writable):', constants.W_OK);
console.log('   ✓ X_OK (executable):', constants.X_OK);

console.log('\n=== Demo Complete ===');
console.log('All filesystem operations working correctly! ✨');
