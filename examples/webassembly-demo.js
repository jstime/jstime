// WebAssembly Demo for jstime
// This example demonstrates how to use WebAssembly in jstime

console.log('=== jstime WebAssembly Demo ===\n');

// 1. Validate WebAssembly bytecode
console.log('1. Validating WebAssembly bytecode:');
const validWasm = new Uint8Array([
  0x00, 0x61, 0x73, 0x6d, // WASM_BINARY_MAGIC
  0x01, 0x00, 0x00, 0x00, // WASM_BINARY_VERSION
]);
console.log('   Valid module:', WebAssembly.validate(validWasm));

const invalidWasm = new Uint8Array([0x00, 0x01, 0x02, 0x03]);
console.log('   Invalid module:', WebAssembly.validate(invalidWasm));
console.log();

// 2. Create and use a simple add function
console.log('2. Creating WebAssembly module with add function:');
const addWasm = new Uint8Array([
  0x00, 0x61, 0x73, 0x6d, // WASM_BINARY_MAGIC
  0x01, 0x00, 0x00, 0x00, // WASM_BINARY_VERSION
  // Type section
  0x01, 0x07, 0x01,       // section code, section size, num types
  0x60, 0x02, 0x7f, 0x7f, // func type: (i32, i32) -> ...
  0x01, 0x7f,             // ... -> i32
  // Function section
  0x03, 0x02, 0x01, 0x00, // section code, section size, num functions, func 0 type
  // Export section
  0x07, 0x07, 0x01,       // section code, section size, num exports
  0x03, 0x61, 0x64, 0x64, // field_len, field_str "add"
  0x00, 0x00,             // export kind (func), export func index
  // Code section
  0x0a, 0x09, 0x01,       // section code, section size, num functions
  0x07, 0x00,             // body size, local decl count
  0x20, 0x00,             // local.get 0
  0x20, 0x01,             // local.get 1
  0x6a,                   // i32.add
  0x0b                    // end
]);

const addModule = new WebAssembly.Module(addWasm);
const addInstance = new WebAssembly.Instance(addModule);
console.log('   add(5, 7) =', addInstance.exports.add(5, 7));
console.log('   add(100, 200) =', addInstance.exports.add(100, 200));
console.log();

// 3. Using the add function with different values
console.log('3. More operations with add:');
console.log('   add(0, 0) =', addInstance.exports.add(0, 0));
console.log('   add(-10, 10) =', addInstance.exports.add(-10, 10));
console.log('   add(999, 1) =', addInstance.exports.add(999, 1));
console.log();

// 4. Working with WebAssembly Memory
console.log('4. Working with WebAssembly Memory:');
const memory = new WebAssembly.Memory({ initial: 1, maximum: 10 });
console.log('   Initial memory size:', memory.buffer.byteLength, 'bytes');
console.log('   Initial pages:', memory.buffer.byteLength / 65536);

// Write some data to memory
const view = new Uint8Array(memory.buffer);
view[0] = 42;
view[1] = 100;
console.log('   Data written to memory[0]:', view[0]);
console.log('   Data written to memory[1]:', view[1]);

// Grow memory
const oldSize = memory.grow(2);
console.log('   Grew memory by 2 pages (old size:', oldSize, 'pages)');
console.log('   New memory size:', memory.buffer.byteLength, 'bytes');
console.log('   New pages:', memory.buffer.byteLength / 65536);
console.log();

// 5. Working with WebAssembly Tables
console.log('5. Working with WebAssembly Tables:');
const table = new WebAssembly.Table({ initial: 2, element: 'anyfunc' });
console.log('   Initial table size:', table.length);

table.grow(3);
console.log('   After growing by 3:', table.length);
console.log();

// 6. Error handling
console.log('6. Error handling:');
try {
  const badWasm = new Uint8Array([0xFF, 0xFF, 0xFF, 0xFF]);
  new WebAssembly.Module(badWasm);
} catch (error) {
  console.log('   Caught CompileError:', error.name);
  console.log('   Error message:', error.message.split('\n')[0]);
}
console.log();

console.log('=== WebAssembly Demo Complete ===');
