// Compliance test for Streams API
// Tests ReadableStream, WritableStream, and TransformStream

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

// Test ReadableStream exists
test('ReadableStream exists', () => {
  if (typeof ReadableStream !== 'function') throw new Error('ReadableStream is not a function');
});

// Test ReadableStream can be constructed
test('ReadableStream can be constructed', () => {
  const stream = new ReadableStream({
    start(controller) {
      controller.close();
    }
  });
  if (typeof stream.getReader !== 'function') throw new Error('getReader is not a function');
});

// Test ReadableStream getReader
test('ReadableStream.getReader works', () => {
  const stream = new ReadableStream({
    start(controller) {
      controller.close();
    }
  });
  const reader = stream.getReader();
  if (typeof reader.read !== 'function') throw new Error('reader.read is not a function');
});

// Test WritableStream exists
test('WritableStream exists', () => {
  if (typeof WritableStream !== 'function') throw new Error('WritableStream is not a function');
});

// Test WritableStream can be constructed
test('WritableStream can be constructed', () => {
  const stream = new WritableStream({
    write(chunk) {}
  });
  if (typeof stream.getWriter !== 'function') throw new Error('getWriter is not a function');
});

// Test WritableStream getWriter
test('WritableStream.getWriter works', () => {
  const stream = new WritableStream({
    write(chunk) {}
  });
  const writer = stream.getWriter();
  if (typeof writer.write !== 'function') throw new Error('writer.write is not a function');
});

// Test TransformStream exists
test('TransformStream exists', () => {
  if (typeof TransformStream !== 'function') throw new Error('TransformStream is not a function');
});

// Test TransformStream can be constructed
test('TransformStream can be constructed', () => {
  const transform = new TransformStream({
    transform(chunk, controller) {
      controller.enqueue(chunk);
    }
  });
  if (typeof transform.readable !== 'object') throw new Error('readable is not an object');
  if (typeof transform.writable !== 'object') throw new Error('writable is not an object');
});

// Test TransformStream has readable and writable
test('TransformStream has readable and writable', () => {
  const transform = new TransformStream();
  if (!(transform.readable instanceof ReadableStream)) throw new Error('readable is not a ReadableStream');
  if (!(transform.writable instanceof WritableStream)) throw new Error('writable is not a WritableStream');
});

// Report results
console.log(`Streams API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
