// Compliance test for Text Encoding API
// Tests TextEncoder and TextDecoder for UTF-8 encoding/decoding

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

// Test TextEncoder exists
test('TextEncoder exists', () => {
  if (typeof TextEncoder !== 'function') throw new Error('TextEncoder is not a function');
});

// Test TextEncoder.prototype.encode exists
test('TextEncoder.prototype.encode exists', () => {
  const encoder = new TextEncoder();
  if (typeof encoder.encode !== 'function') throw new Error('encode is not a function');
});

// Test TextEncoder encoding property
test('TextEncoder encoding property', () => {
  const encoder = new TextEncoder();
  if (encoder.encoding !== 'utf-8') throw new Error(`expected encoding to be 'utf-8', got '${encoder.encoding}'`);
});

// Test TextEncoder encodes ASCII correctly
test('TextEncoder encodes ASCII', () => {
  const encoder = new TextEncoder();
  const result = encoder.encode('Hello');
  if (!(result instanceof Uint8Array)) throw new Error('encode did not return Uint8Array');
  if (result.length !== 5) throw new Error(`expected length 5, got ${result.length}`);
  if (result[0] !== 72 || result[1] !== 101 || result[2] !== 108 || result[3] !== 108 || result[4] !== 111) {
    throw new Error('incorrect encoding');
  }
});

// Test TextEncoder encodes UTF-8 multi-byte characters
test('TextEncoder encodes UTF-8 multi-byte', () => {
  const encoder = new TextEncoder();
  const result = encoder.encode('€'); // Euro sign (3 bytes in UTF-8)
  if (result.length !== 3) throw new Error(`expected length 3, got ${result.length}`);
  if (result[0] !== 0xE2 || result[1] !== 0x82 || result[2] !== 0xAC) {
    throw new Error('incorrect UTF-8 encoding');
  }
});

// Test TextDecoder exists
test('TextDecoder exists', () => {
  if (typeof TextDecoder !== 'function') throw new Error('TextDecoder is not a function');
});

// Test TextDecoder.prototype.decode exists
test('TextDecoder.prototype.decode exists', () => {
  const decoder = new TextDecoder();
  if (typeof decoder.decode !== 'function') throw new Error('decode is not a function');
});

// Test TextDecoder encoding property
test('TextDecoder encoding property', () => {
  const decoder = new TextDecoder();
  if (decoder.encoding !== 'utf-8') throw new Error(`expected encoding to be 'utf-8', got '${decoder.encoding}'`);
});

// Test TextDecoder decodes ASCII correctly
test('TextDecoder decodes ASCII', () => {
  const decoder = new TextDecoder();
  const bytes = new Uint8Array([72, 101, 108, 108, 111]);
  const result = decoder.decode(bytes);
  if (result !== 'Hello') throw new Error(`expected 'Hello', got '${result}'`);
});

// Test TextDecoder decodes UTF-8 multi-byte
test('TextDecoder decodes UTF-8 multi-byte', () => {
  const decoder = new TextDecoder();
  const bytes = new Uint8Array([0xE2, 0x82, 0xAC]); // Euro sign
  const result = decoder.decode(bytes);
  if (result !== '€') throw new Error(`expected '€', got '${result}'`);
});

// Test round-trip encoding and decoding
test('round-trip encode/decode', () => {
  const encoder = new TextEncoder();
  const decoder = new TextDecoder();
  const original = 'Hello, 世界! 🌍';
  const encoded = encoder.encode(original);
  const decoded = decoder.decode(encoded);
  if (decoded !== original) throw new Error(`round-trip failed: expected '${original}', got '${decoded}'`);
});

// Report results
console.log(`Text Encoding API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
