// Timers API Demo for jstime
// This example demonstrates setTimeout, setInterval, and related functions

console.log('=== jstime Timers API Demo ===\n');

// 1. Basic setTimeout
console.log('1. Basic setTimeout:');
console.log('   Starting timer for 1 second...');
setTimeout(() => {
  console.log('   ✓ Timer fired after 1 second!');
}, 1000);

// 2. setTimeout with arguments
setTimeout((name, value) => {
  console.log('\n2. setTimeout with arguments:');
  console.log('   ✓ Received:', name, '=', value);
}, 1500, 'message', 'Hello!');

// 3. setInterval
let counter = 0;
const intervalId = setInterval(() => {
  counter++;
  console.log(`\n3. Interval tick #${counter}`);
  
  if (counter === 3) {
    console.log('   ✓ Stopping interval after 3 ticks');
    clearInterval(intervalId);
  }
}, 500);

// 4. Clearing a timeout
setTimeout(() => {
  console.log('\n4. Clearing timeouts:');
  
  const timeoutId = setTimeout(() => {
    console.log('   ✗ This should not print');
  }, 5000);
  
  clearTimeout(timeoutId);
  console.log('   ✓ Timeout was cleared before firing');
}, 2500);

// 5. Multiple concurrent timers
setTimeout(() => {
  console.log('\n5. Multiple concurrent timers:');
  
  setTimeout(() => console.log('   Timer A (100ms)'), 100);
  setTimeout(() => console.log('   Timer B (200ms)'), 200);
  setTimeout(() => console.log('   Timer C (300ms)'), 300);
}, 3000);

// 6. Nested timers
setTimeout(() => {
  console.log('\n6. Nested timers:');
  console.log('   Outer timer');
  
  setTimeout(() => {
    console.log('   ✓ Nested timer');
  }, 300);
}, 3500);

// Final message
setTimeout(() => {
  console.log('\n=== Timers Demo Complete ===');
}, 4200);
