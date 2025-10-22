// Process API Demo
// Demonstrates the jstime Process API

console.log('=== Process API Demo ===\n');

// 1. Environment Variables
console.log('1. Environment Variables');
console.log('------------------------');
console.log('HOME:', process.env.HOME || process.env.USERPROFILE);
console.log('USER:', process.env.USER || process.env.USERNAME);
console.log('PATH exists:', !!process.env.PATH);
console.log('SHELL:', process.env.SHELL || 'N/A');
console.log();

// 2. Command-line Arguments
console.log('2. Command-line Arguments');
console.log('-------------------------');
console.log('Executable:', process.argv[0]);
if (process.argv.length > 1) {
  console.log('Script:', process.argv[1]);
}
if (process.argv.length > 2) {
  console.log('Additional arguments:', process.argv.slice(2));
} else {
  console.log('No additional arguments provided');
}
console.log('Total arguments:', process.argv.length);
console.log();

// 3. Current Working Directory
console.log('3. Current Working Directory');
console.log('---------------------------');
console.log('CWD:', process.cwd());
console.log();

// 4. Exit Function (demonstration only - not actually called)
console.log('4. Exit Function');
console.log('----------------');
console.log('process.exit is available:', typeof process.exit === 'function');
console.log('Note: process.exit(code) terminates the process immediately');
console.log();

// 5. Practical Example: Simple CLI Tool
console.log('5. Practical Example: Configuration Check');
console.log('-----------------------------------------');

// Check for required environment variables
const requiredVars = ['HOME', 'PATH', 'USER'];
let allPresent = true;

requiredVars.forEach(varName => {
  const value = process.env[varName];
  if (value) {
    console.log(`✅ ${varName}: set`);
  } else {
    console.log(`❌ ${varName}: not set`);
    allPresent = false;
  }
});

if (!allPresent) {
  console.log('\n⚠️  Warning: Some required environment variables are missing');
}

console.log('\n=== Demo Complete ===');
console.log('\nTry running with arguments:');
console.log('  jstime process-demo.js --verbose debug mode');
console.log('\nOr set environment variables:');
console.log('  MY_VAR=test jstime process-demo.js');
