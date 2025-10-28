// Text Encoding API Demo
// Demonstrates TextEncoder and TextDecoder for encoding/decoding UTF-8 text

console.log('=== Text Encoding API Demo ===\n');

// Create encoder and decoder
const encoder = new TextEncoder();
const decoder = new TextDecoder();

console.log('1. Basic Encoding and Decoding');
console.log('--------------------------------');
const text = 'Hello, World!';
const encoded = encoder.encode(text);
console.log('Original text:', text);
console.log('Encoded bytes:', Array.from(encoded).join(', '));
console.log('Encoded length:', encoded.length, 'bytes');

const decoded = decoder.decode(encoded);
console.log('Decoded text:', decoded);
console.log('Match:', text === decoded);
console.log();

// UTF-8 Multi-byte Characters
console.log('2. UTF-8 Multi-byte Characters');
console.log('--------------------------------');
const characters = ['€', '世', '界', '😀', '🌍'];
characters.forEach(char => {
  const bytes = encoder.encode(char);
  console.log(`'${char}': ${bytes.length} bytes →`, Array.from(bytes).join(', '));
});
console.log();

// Different Languages
console.log('3. Different Languages');
console.log('--------------------------------');
const greetings = [
  { lang: 'English', text: 'Hello' },
  { lang: 'Spanish', text: 'Hola' },
  { lang: 'Chinese', text: '你好' },
  { lang: 'Japanese', text: 'こんにちは' },
  { lang: 'Arabic', text: 'مرحبا' },
  { lang: 'Russian', text: 'Привет' },
  { lang: 'Korean', text: '안녕하세요' }
];

greetings.forEach(({ lang, text }) => {
  const bytes = encoder.encode(text);
  console.log(`${lang}: "${text}" (${bytes.length} bytes)`);
});
console.log();

// Using encodeInto
console.log('4. Using encodeInto');
console.log('--------------------------------');
const buffer = new Uint8Array(20);
const source = 'Hello 世界';
const result = encoder.encodeInto(source, buffer);
console.log('Source text:', source);
console.log('Buffer size:', buffer.length, 'bytes');
console.log('Result:', result);
console.log('Written bytes:', Array.from(buffer.slice(0, result.written)).join(', '));
console.log('Decoded:', decoder.decode(buffer.slice(0, result.written)));
console.log();

// Buffer Overflow Handling
console.log('5. Buffer Overflow Handling');
console.log('--------------------------------');
const smallBuffer = new Uint8Array(5);
const longText = 'Hello, World!';
const overflowResult = encoder.encodeInto(longText, smallBuffer);
console.log('Text:', longText);
console.log('Buffer size:', smallBuffer.length, 'bytes');
console.log('Result:', overflowResult);
console.log('Partial decode:', decoder.decode(smallBuffer));
console.log();

// Round-trip with Special Characters
console.log('6. Round-trip with Special Characters');
console.log('--------------------------------');
const specialTexts = [
  'Line break: Hello\nWorld',
  'Tab: Hello\tWorld',
  'Null byte: Hello\x00World',
  'Mixed: Hello 世界 🚀 Привет',
];

specialTexts.forEach(text => {
  const encoded = encoder.encode(text);
  const decoded = decoder.decode(encoded);
  const match = text === decoded;
  console.log(`✓ Round-trip ${match ? 'passed' : 'FAILED'}: "${text.replace(/\n/g, '\\n').replace(/\t/g, '\\t').replace(/\x00/g, '\\x00')}"`);
});
console.log();

// Encoding Properties
console.log('7. Encoding Properties');
console.log('--------------------------------');
console.log('TextEncoder encoding:', encoder.encoding);
console.log('TextDecoder encoding:', decoder.encoding);
console.log();

// Emoji Encoding
console.log('8. Emoji Encoding');
console.log('--------------------------------');
const emojis = '😀😃😄😁🤣😂🙂🙃😉';
const emojiBytes = encoder.encode(emojis);
console.log('Emojis:', emojis);
console.log('Total bytes:', emojiBytes.length);
console.log('Bytes per emoji:', emojiBytes.length / emojis.length);
console.log('First emoji bytes:', Array.from(emojiBytes.slice(0, 4)).join(', '));
console.log();

// Decoding Empty/Null
console.log('9. Decoding Edge Cases');
console.log('--------------------------------');
console.log('Empty array:', `"${decoder.decode(new Uint8Array([]))}"`);
console.log('Null:', `"${decoder.decode(null)}"`);
console.log('Undefined:', `"${decoder.decode()}"`);
console.log();

// Byte Size Comparison
console.log('10. Byte Size Comparison');
console.log('--------------------------------');
const comparisonText = 'A€世😀';
console.log('Text:', comparisonText);
console.log('JavaScript length (UTF-16 code units):', comparisonText.length);
const comparisonBytes = encoder.encode(comparisonText);
console.log('UTF-8 bytes:', comparisonBytes.length);
console.log('Character breakdown:');
console.log('  A (ASCII):', encoder.encode('A').length, 'byte');
console.log('  € (Latin-1 Supplement):', encoder.encode('€').length, 'bytes');
console.log('  世 (BMP):', encoder.encode('世').length, 'bytes');
console.log('  😀 (Supplementary Plane):', encoder.encode('😀').length, 'bytes');
console.log();

console.log('=== Demo Complete ===');
