// Performance benchmark: File System API
// Measures file read/write operations from node:fs/promises

import * as fs from 'node:fs/promises';

const ITERATIONS = 1000;

const results = [];
let totalElapsed = 0;

// Create a temporary directory for testing
const tmpDir = '/tmp/jstime-fs-perf-' + Date.now();
await fs.mkdir(tmpDir);

const testData = 'Hello, World! This is test data for benchmarking.\n'.repeat(10);
const testFilePath = tmpDir + '/test.txt';

// Test 1: writeFile (small file)
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  await fs.writeFile(testFilePath, testData, 'utf-8');
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'write_small',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: readFile (small file)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  await fs.readFile(testFilePath, 'utf-8');
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'read_small',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: stat
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  await fs.stat(testFilePath);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'stat',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: appendFile
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  await fs.appendFile(testFilePath, 'x', 'utf-8');
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'append',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 5: readdir
// Create some test files
for (let i = 0; i < 10; i++) {
  await fs.writeFile(`${tmpDir}/file${i}.txt`, 'test', 'utf-8');
}

start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  await fs.readdir(tmpDir);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'readdir',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Clean up
await fs.rm(tmpDir, { recursive: true });

console.log(JSON.stringify({
  test: 'fs',
  iterations: ITERATIONS,
  elapsed_ms: totalElapsed.toFixed(3),
  ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
  sub_tests: results
}));
