// Performance benchmark: Crypto operations
// Measures crypto operations performance

const ITERATIONS = 10000;

const results = [];
let totalElapsed = 0;

// Test 1: crypto.randomUUID
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const uuid = crypto.randomUUID();
}
let end = performance.now();
let elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'randomUUID',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 2: crypto.getRandomValues (small - 16 bytes)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const arr = new Uint8Array(16);
  crypto.getRandomValues(arr);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'getRandomValues_16bytes',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 3: crypto.getRandomValues (medium - 256 bytes)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const arr = new Uint8Array(256);
  crypto.getRandomValues(arr);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'getRandomValues_256bytes',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 4: crypto.getRandomValues (large - 1024 bytes)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const arr = new Uint8Array(1024);
  crypto.getRandomValues(arr);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'getRandomValues_1024bytes',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 5: crypto.getRandomValues (very large - 4096 bytes)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const arr = new Uint8Array(4096);
  crypto.getRandomValues(arr);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'getRandomValues_4096bytes',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 6: crypto.getRandomValues with different typed arrays
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const arr = new Uint32Array(64);
  crypto.getRandomValues(arr);
}
end = performance.now();
elapsed = end - start;
totalElapsed += elapsed;
results.push({
  name: 'getRandomValues_uint32',
  elapsed_ms: elapsed.toFixed(3),
  ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
});

// Test 7: crypto.subtle.digest SHA-256 (small data)
async function testDigest() {
  const encoder = new TextEncoder();
  
  start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) {
    const data = encoder.encode('Hello, World!');
    await crypto.subtle.digest('SHA-256', data);
  }
  end = performance.now();
  elapsed = end - start;
  totalElapsed += elapsed;
  results.push({
    name: 'digest_sha256_small',
    elapsed_ms: elapsed.toFixed(3),
    ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
  });

  // Test 8: crypto.subtle.digest SHA-256 (medium data)
  const mediumData = encoder.encode('Hello, World! '.repeat(100));
  start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) {
    await crypto.subtle.digest('SHA-256', mediumData);
  }
  end = performance.now();
  elapsed = end - start;
  totalElapsed += elapsed;
  results.push({
    name: 'digest_sha256_medium',
    elapsed_ms: elapsed.toFixed(3),
    ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
  });

  // Test 9: crypto.subtle.digest SHA-256 (large data)
  const largeData = encoder.encode('Hello, World! '.repeat(1000));
  start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) {
    await crypto.subtle.digest('SHA-256', largeData);
  }
  end = performance.now();
  elapsed = end - start;
  totalElapsed += elapsed;
  results.push({
    name: 'digest_sha256_large',
    elapsed_ms: elapsed.toFixed(3),
    ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
  });

  // Test 10: crypto.subtle.digest SHA-384
  start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) {
    const data = encoder.encode('Hello, World!');
    await crypto.subtle.digest('SHA-384', data);
  }
  end = performance.now();
  elapsed = end - start;
  totalElapsed += elapsed;
  results.push({
    name: 'digest_sha384',
    elapsed_ms: elapsed.toFixed(3),
    ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
  });

  // Test 11: crypto.subtle.digest SHA-512
  start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) {
    const data = encoder.encode('Hello, World!');
    await crypto.subtle.digest('SHA-512', data);
  }
  end = performance.now();
  elapsed = end - start;
  totalElapsed += elapsed;
  results.push({
    name: 'digest_sha512',
    elapsed_ms: elapsed.toFixed(3),
    ops_per_ms: (ITERATIONS / elapsed).toFixed(2)
  });

  console.log(JSON.stringify({
    test: 'crypto_uuid',
    iterations: ITERATIONS,
    elapsed_ms: totalElapsed.toFixed(3),
    ops_per_ms: (ITERATIONS * results.length / totalElapsed).toFixed(2),
    sub_tests: results
  }));
}

testDigest();
