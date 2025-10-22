// Console API Demo for jstime
// This example demonstrates the console API features

console.log('=== jstime Console API Demo ===\n');

// 1. Basic logging
console.log('1. Basic console.log():');
console.log('   Simple message');
console.log('   Multiple', 'arguments', 'work', 'too');
console.log();

// 2. Different log levels
console.log('2. Different log levels:');
console.info('   This is an info message');
console.debug('   This is a debug message');
console.warn('   This is a warning message');
console.error('   This is an error message');
console.log();

// 3. Logging objects
console.log('3. Logging objects and arrays:');
const person = { name: 'Alice', age: 30, city: 'New York' };
console.log('   Object:', person);

const numbers = [1, 2, 3, 4, 5];
console.log('   Array:', numbers);

const nested = { user: { name: 'Bob', preferences: { theme: 'dark' } } };
console.log('   Nested object:', nested);
console.log();

// 4. Format specifiers
console.log('4. Format specifiers:');
console.log('   String: %s', 'Hello');
console.log('   Integer: %d', 42);
console.log('   Float: %f', 3.14159);
console.log('   Multiple: Name is %s and age is %d', 'Alice', 30);
console.log();

// 5. Logging special values
console.log('5. Special values:');
console.log('   null:', null);
console.log('   undefined:', undefined);
console.log('   boolean:', true, false);
console.log('   NaN:', NaN);
console.log('   Infinity:', Infinity);
console.log();

console.log('=== Console Demo Complete ===');
