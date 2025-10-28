// Event and EventTarget API Demo for jstime
// This example demonstrates the Event and EventTarget interfaces

console.log('=== jstime Event and EventTarget API Demo ===\n');

// 1. Creating events
console.log('1. Creating events:');
const event1 = new Event('custom');
const event2 = new Event('click', { bubbles: true, cancelable: true });

console.log('   Event 1 type:', event1.type);
console.log('   Event 1 bubbles:', event1.bubbles);
console.log('   Event 2 type:', event2.type);
console.log('   Event 2 bubbles:', event2.bubbles);
console.log('   Event 2 cancelable:', event2.cancelable);
console.log();

// 2. Creating EventTarget and adding listeners
console.log('2. Creating EventTarget and adding listeners:');
const target = new EventTarget();

target.addEventListener('message', (e) => {
  console.log('   ✓ Message event received!');
  console.log('   Event type:', e.type);
  console.log('   Event target:', e.target === target);
});

target.dispatchEvent(new Event('message'));
console.log();

// 3. Multiple listeners for the same event
console.log('3. Multiple listeners for the same event:');
const multiTarget = new EventTarget();

multiTarget.addEventListener('data', () => {
  console.log('   Handler 1 executed');
});

multiTarget.addEventListener('data', () => {
  console.log('   Handler 2 executed');
});

multiTarget.addEventListener('data', () => {
  console.log('   Handler 3 executed');
});

console.log('   Dispatching event...');
multiTarget.dispatchEvent(new Event('data'));
console.log();

// 4. Removing event listeners
console.log('4. Removing event listeners:');
const removeTarget = new EventTarget();

const handler = () => {
  console.log('   ✓ Handler executed');
};

removeTarget.addEventListener('test', handler);
console.log('   First dispatch:');
removeTarget.dispatchEvent(new Event('test'));

removeTarget.removeEventListener('test', handler);
console.log('   Second dispatch (after removal):');
removeTarget.dispatchEvent(new Event('test'));
console.log('   (no output above means handler was removed)');
console.log();

// 5. Event properties
console.log('5. Event properties:');
const propTarget = new EventTarget();

propTarget.addEventListener('check', (e) => {
  console.log('   Event type:', e.type);
  console.log('   Event target:', e.target === propTarget);
  console.log('   Current target:', e.currentTarget === propTarget);
  console.log('   Event phase:', e.eventPhase);
  console.log('   Bubbles:', e.bubbles);
  console.log('   Cancelable:', e.cancelable);
  console.log('   Timestamp:', typeof e.timeStamp, e.timeStamp.toFixed(2));
});

propTarget.dispatchEvent(new Event('check', { bubbles: true, cancelable: true }));
console.log();

// 6. Preventing default and stopping propagation
console.log('6. Preventing default and stopping propagation:');
const cancelTarget = new EventTarget();

cancelTarget.addEventListener('submit', (e) => {
  console.log('   Handler called');
  console.log('   Default prevented before:', e.defaultPrevented);
  e.preventDefault();
  console.log('   Default prevented after:', e.defaultPrevented);
});

const cancelEvent = new Event('submit', { cancelable: true });
const notCanceled = cancelTarget.dispatchEvent(cancelEvent);
console.log('   Event was canceled:', !notCanceled);
console.log();

// 7. Stop immediate propagation
console.log('7. Stop immediate propagation:');
const stopTarget = new EventTarget();

stopTarget.addEventListener('stop', (e) => {
  console.log('   Handler 1 - stopping propagation');
  e.stopImmediatePropagation();
});

stopTarget.addEventListener('stop', () => {
  console.log('   Handler 2 - should not execute');
});

stopTarget.addEventListener('stop', () => {
  console.log('   Handler 3 - should not execute');
});

stopTarget.dispatchEvent(new Event('stop'));
console.log();

// 8. Custom event types
console.log('8. Custom event types:');
const customTarget = new EventTarget();

customTarget.addEventListener('user-login', (e) => {
  console.log('   ✓ User login event received');
});

customTarget.addEventListener('data-received', (e) => {
  console.log('   ✓ Data received event received');
});

customTarget.addEventListener('connection-error', (e) => {
  console.log('   ✓ Connection error event received');
});

console.log('   Dispatching custom events:');
customTarget.dispatchEvent(new Event('user-login'));
customTarget.dispatchEvent(new Event('data-received'));
customTarget.dispatchEvent(new Event('connection-error'));
console.log();

// 9. Event emitter pattern
console.log('9. Event emitter pattern:');
class DataSource extends EventTarget {
  fetchData() {
    console.log('   Fetching data...');
    
    // Simulate async operation
    setTimeout(() => {
      this.dispatchEvent(new Event('data-ready'));
    }, 0);
  }
  
  connect() {
    console.log('   Connecting...');
    this.dispatchEvent(new Event('connected'));
  }
}

const source = new DataSource();

source.addEventListener('connected', () => {
  console.log('   ✓ Connected to data source');
});

source.addEventListener('data-ready', () => {
  console.log('   ✓ Data is ready');
});

source.connect();
source.fetchData();

// Note: In a script file, we need to wait manually for events
setTimeout(() => {
  console.log();
  console.log('=== Events Demo Complete ===');
}, 100);
