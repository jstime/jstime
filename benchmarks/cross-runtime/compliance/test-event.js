// Compliance test for Event and EventTarget API
// Tests Event and EventTarget classes for event handling

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

// Test Event constructor exists
test('Event constructor exists', () => {
  if (typeof Event !== 'function') throw new Error('Event is not a function');
});

// Test Event can be constructed
test('Event can be constructed', () => {
  const event = new Event('test');
  if (event.type !== 'test') throw new Error(`expected type 'test', got '${event.type}'`);
});

// Test Event with options
test('Event with bubbles option', () => {
  const event = new Event('test', { bubbles: true });
  if (!event.bubbles) throw new Error('bubbles should be true');
});

// Test Event with cancelable option
test('Event with cancelable option', () => {
  const event = new Event('test', { cancelable: true });
  if (!event.cancelable) throw new Error('cancelable should be true');
});

// Test Event.preventDefault
test('Event.preventDefault', () => {
  const event = new Event('test', { cancelable: true });
  event.preventDefault();
  if (!event.defaultPrevented) throw new Error('defaultPrevented should be true');
});

// Test EventTarget constructor exists
test('EventTarget constructor exists', () => {
  if (typeof EventTarget !== 'function') throw new Error('EventTarget is not a function');
});

// Test EventTarget can be constructed
test('EventTarget can be constructed', () => {
  const target = new EventTarget();
  if (typeof target.addEventListener !== 'function') throw new Error('addEventListener is not a function');
});

// Test addEventListener
test('addEventListener works', () => {
  const target = new EventTarget();
  let called = false;
  target.addEventListener('test', () => { called = true; });
  target.dispatchEvent(new Event('test'));
  if (!called) throw new Error('event listener was not called');
});

// Test dispatchEvent returns boolean
test('dispatchEvent returns boolean', () => {
  const target = new EventTarget();
  const result = target.dispatchEvent(new Event('test'));
  if (typeof result !== 'boolean') throw new Error(`expected boolean, got ${typeof result}`);
});

// Test event is passed to listener
test('event is passed to listener', () => {
  const target = new EventTarget();
  let receivedEvent = null;
  target.addEventListener('test', (e) => { receivedEvent = e; });
  const event = new Event('test');
  target.dispatchEvent(event);
  if (receivedEvent !== event) throw new Error('listener did not receive the event');
});

// Test multiple listeners
test('multiple listeners are called', () => {
  const target = new EventTarget();
  let count = 0;
  target.addEventListener('test', () => { count++; });
  target.addEventListener('test', () => { count++; });
  target.addEventListener('test', () => { count++; });
  target.dispatchEvent(new Event('test'));
  if (count !== 3) throw new Error(`expected 3 calls, got ${count}`);
});

// Test removeEventListener
test('removeEventListener works', () => {
  const target = new EventTarget();
  let count = 0;
  const handler = () => { count++; };
  target.addEventListener('test', handler);
  target.dispatchEvent(new Event('test'));
  if (count !== 1) throw new Error(`expected 1 call, got ${count}`);
  target.removeEventListener('test', handler);
  target.dispatchEvent(new Event('test'));
  if (count !== 1) throw new Error(`expected 1 call after removal, got ${count}`);
});

// Test stopImmediatePropagation
test('stopImmediatePropagation stops other listeners', () => {
  const target = new EventTarget();
  let count = 0;
  target.addEventListener('test', (e) => { 
    count++;
    e.stopImmediatePropagation();
  });
  target.addEventListener('test', () => { count++; });
  target.dispatchEvent(new Event('test'));
  if (count !== 1) throw new Error(`expected 1 call, got ${count}`);
});

// Test event target and currentTarget
test('event target is set correctly', () => {
  const target = new EventTarget();
  let eventTarget = null;
  let eventCurrentTarget = null;
  target.addEventListener('test', (e) => {
    eventTarget = e.target;
    eventCurrentTarget = e.currentTarget;
  });
  target.dispatchEvent(new Event('test'));
  if (eventTarget !== target) throw new Error('event.target should be the EventTarget');
  if (eventCurrentTarget !== target) throw new Error('event.currentTarget should be the EventTarget');
});

// Report results
console.log(`Event API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
