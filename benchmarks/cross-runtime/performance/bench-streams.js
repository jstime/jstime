// Performance benchmark: Streams API
// Measures ReadableStream, WritableStream, and TransformStream performance

const ITERATIONS = 1000;

const results = [];
let totalElapsed = 0;

// Test 1: ReadableStream creation
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const stream = new ReadableStream({
    start(controller) {
      controller.close();
    }
  });
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'readable_creation',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: WritableStream creation
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const stream = new WritableStream({
    write(chunk) {}
  });
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'writable_creation',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: TransformStream creation
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const stream = new TransformStream();
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'transform_creation',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: ReadableStream read operations
async function testRead() {
  start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) {
    const stream = new ReadableStream({
      start(controller) {
        controller.enqueue('data');
        controller.close();
      }
    });
    const reader = stream.getReader();
    await reader.read();
  }
  end = performance.now();
  elapsed = end - start;
  totalElapsed += elapsed;
  results.push({
    name: 'readable_read',
    elapsed_ms: elapsed.toFixed(3),
    ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
  });

  // Test 5: WritableStream write operations
  start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) {
    const stream = new WritableStream({
      write(chunk) {}
    });
    const writer = stream.getWriter();
    await writer.write('data');
    await writer.close();
  }
  end = performance.now();
  elapsed = end - start;
  totalElapsed += elapsed;
  results.push({
    name: 'writable_write',
    elapsed_ms: elapsed.toFixed(3),
    ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
  });

  console.log(JSON.stringify({
    test: 'streams',
    iterations: ITERATIONS,
    elapsed_ms: totalElapsed.toFixed(3),
    ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
    sub_tests: results
  }));
}

testRead();
