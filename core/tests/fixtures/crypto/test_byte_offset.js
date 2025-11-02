// Test that crypto.getRandomValues correctly handles typed arrays with byte offsets

// Create a buffer and two views with different offsets
const buffer = new ArrayBuffer(32);
const view1 = new Uint8Array(buffer, 0, 16);  // First 16 bytes
const view2 = new Uint8Array(buffer, 16, 16); // Second 16 bytes

// Fill both views with random data
crypto.getRandomValues(view1);
crypto.getRandomValues(view2);

// Check that both views have non-zero data
let hasDataView1 = view1.some(v => v !== 0);
let hasDataView2 = view2.some(v => v !== 0);

if (!hasDataView1 || !hasDataView2) {
  throw new Error('Views should have random data');
}

// Check that the views are different (extremely unlikely to be same with random data)
let allSame = true;
for (let i = 0; i < 16; i++) {
  if (view1[i] !== view2[i]) {
    allSame = false;
    break;
  }
}

if (allSame) {
  throw new Error('Views should have different random data');
}

// Test with a view that has both offset and is smaller
const offsetView = new Uint8Array(buffer, 8, 8);
const beforeFill = new Uint8Array(offsetView);
crypto.getRandomValues(offsetView);

// Check that the data changed
let changed = false;
for (let i = 0; i < 8; i++) {
  if (beforeFill[i] !== offsetView[i]) {
    changed = true;
    break;
  }
}

if (!changed) {
  throw new Error('Offset view should have been filled with random data');
}

// Test edge case: view at the very end
const endView = new Uint8Array(buffer, 31, 1);
crypto.getRandomValues(endView);

console.log('All byte offset tests passed');
