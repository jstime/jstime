// Dynamic Import Example
// This demonstrates the dynamic import() feature in jstime

console.log('=== Dynamic Import Example ===\n');

// Example 1: Basic dynamic import
console.log('1. Basic dynamic import:');
const mathModule = await import('./math-helper.mjs');
console.log('  Loaded math module');
console.log('  5 + 3 =', mathModule.add(5, 3));
console.log('  5 * 3 =', mathModule.multiply(5, 3));
console.log();

// Example 2: Conditional import
console.log('2. Conditional import:');
const shouldLoadExtra = true;
if (shouldLoadExtra) {
  const extra = await import('./math-helper.mjs');
  console.log('  PI =', extra.PI);
}
console.log();

// Example 3: Dynamic import with error handling
console.log('3. Error handling:');
try {
  await import('./non-existent-module.mjs');
} catch (error) {
  console.log('  Caught error:', error.message);
}
console.log();

// Example 4: Loading multiple modules in parallel
console.log('4. Parallel imports:');
const [mod1, mod2] = await Promise.all([
  import('./math-helper.mjs'),
  import('./data-example.json')
]);
console.log('  Math module has add:', typeof mod1.add === 'function');
console.log('  JSON data:', mod2.default.name);
console.log();

// Example 5: Dynamic import with Promise .then()
console.log('5. Promise-based import:');
import('./math-helper.mjs')
  .then(module => {
    console.log('  10 + 20 =', module.add(10, 20));
  });

console.log('\n=== Example Complete ===');
