# System APIs

This document describes the system-level APIs implemented in jstime for interacting with the operating system and file system.

## Table of Contents

- [Process API](#process-api)
- [Buffer API](#buffer-api)
- [File System API](#file-system-api)

## Process API

jstime implements a subset of the [Node.js Process API](https://nodejs.org/api/process.html), providing access to process information, environment variables, command-line arguments, and process control.

### Supported APIs

- `process.env` - Access environment variables
- `process.argv` - Access command-line arguments
- `process.cwd()` - Get current working directory
- `process.exit(code?)` - Exit the process with an optional exit code

### process.env

The `process.env` property returns an object containing the user environment variables.

#### Example

```javascript
// Access environment variables
console.log('Home directory:', process.env.HOME);
console.log('PATH:', process.env.PATH);

// Check if a variable exists
if (process.env.NODE_ENV) {
  console.log('Running in:', process.env.NODE_ENV);
}

// Common environment variables
console.log('User:', process.env.USER);
console.log('Shell:', process.env.SHELL);
```

**Note:** `process.env` is read-only. Setting properties on it will not modify the actual environment variables.

### process.argv

The `process.argv` property returns an array containing the command-line arguments passed when the jstime process was launched. The first element is the path to the jstime executable, and the second element (if present) is the path to the JavaScript file being executed.

#### Example

```javascript
// Print all command-line arguments
console.log('Arguments:', process.argv);

// Get the executable path
console.log('Executable:', process.argv[0]);

// Get the script path (if running a script)
if (process.argv.length > 1) {
  console.log('Script:', process.argv[1]);
}

// Get additional arguments
if (process.argv.length > 2) {
  console.log('Additional args:', process.argv.slice(2));
}
```

**Example usage:**
```bash
$ jstime script.js arg1 arg2 arg3
# process.argv will be: ['jstime', 'script.js', 'arg1', 'arg2', 'arg3']
```

### process.cwd()

The `process.cwd()` method returns the current working directory of the jstime process.

#### Example

```javascript
// Get current working directory
const cwd = process.cwd();
console.log('Current directory:', cwd);

// Use with path operations
import { readFile } from 'node:fs/promises';

// Read a file relative to cwd
const configPath = `${process.cwd()}/config.json`;
const config = await readFile(configPath, 'utf-8');
console.log('Config:', config);
```

**Returns:** String representing the current working directory path.

### process.exit(code?)

The `process.exit()` method instructs jstime to terminate the process synchronously with an exit status. If `code` is omitted, exit uses the 'success' code `0`.

#### Example

```javascript
// Exit with success code (0)
process.exit();

// Exit with a specific code
process.exit(1); // Indicates an error

// Conditional exit
if (someErrorCondition) {
  console.error('Fatal error occurred');
  process.exit(1);
}

// Exit after cleanup
console.log('Performing cleanup...');
// ... cleanup code ...
process.exit(0);
```

**Parameters:**
- `code` (number, optional): The exit code. Defaults to `0` (success).

**Common exit codes:**
- `0`: Success
- `1`: General error
- `2`: Misuse of shell command
- Other non-zero values indicate various error conditions

**Note:** `process.exit()` terminates the process immediately, preventing any remaining asynchronous operations from completing. Use it only when necessary.

### Complete Example

```javascript
// script.js - A simple script using the Process API

// Check environment
console.log('=== Environment ===');
console.log('User:', process.env.USER || process.env.USERNAME);
console.log('Home:', process.env.HOME || process.env.USERPROFILE);

// Parse command-line arguments
console.log('\n=== Arguments ===');
console.log('Executable:', process.argv[0]);
if (process.argv.length > 1) {
  console.log('Script:', process.argv[1]);
}

// Check for --help flag
if (process.argv.includes('--help')) {
  console.log('\nUsage: jstime script.js [options]');
  process.exit(0);
}

// Get additional arguments
const args = process.argv.slice(2);
if (args.length > 0) {
  console.log('Additional arguments:', args);
}

// Show current directory
console.log('\n=== Working Directory ===');
console.log('CWD:', process.cwd());

// Exit with success
console.log('\n✅ Script completed successfully');
process.exit(0);
```

**Running the example:**
```bash
$ jstime script.js --verbose arg1 arg2
=== Environment ===
User: alice
Home: /home/alice

=== Arguments ===
Executable: jstime
Script: script.js
Additional arguments: [ '--verbose', 'arg1', 'arg2' ]

=== Working Directory ===
CWD: /home/alice/projects/myapp

✅ Script completed successfully
```

### Use Cases

The Process API is useful for:

- **Configuration**: Reading API keys, database URLs, and other config from environment variables
- **Command-line tools**: Building CLI applications that parse arguments
- **Path resolution**: Working with file paths relative to the current directory
- **Error handling**: Exiting with appropriate error codes for shell scripts
- **Debugging**: Inspecting the runtime environment

### Comparison with Node.js

jstime's Process API implements a minimal subset of Node.js's process object:

| Feature | jstime | Node.js |
|---------|--------|---------|
| `process.env` | ✅ (read-only) | ✅ (read/write) |
| `process.argv` | ✅ | ✅ |
| `process.cwd()` | ✅ | ✅ |
| `process.exit(code)` | ✅ | ✅ |
| `process.chdir(dir)` | ❌ | ✅ |
| `process.pid` | ❌ | ✅ |
| `process.platform` | ❌ | ✅ |
| `process.version` | ❌ | ✅ |
| `process.stdin/stdout/stderr` | ✅ (basic) | ✅ |
| Event emitters | ❌ | ✅ |

The `process.stdout`, `process.stderr`, and `process.stdin` are implemented as basic stream-like objects with `write()` and `read()` methods respectively. They support writing strings and Uint8Arrays, but don't include the full Node.js Stream API features.

For most common use cases (configuration, CLI arguments, working directory, basic I/O), jstime's Process API provides sufficient functionality.

## Buffer API

jstime provides a Node.js-compatible Buffer API through the `node:buffer` module. Buffer is used to work with binary data directly and is essential for tasks like file I/O, network communication, and cryptography.

### Usage

```javascript
import { Buffer } from 'node:buffer';
// or
import bufferModule from 'node:buffer';
const { Buffer } = bufferModule;
```

**Note:** `Buffer` is also available globally, so you can use it without importing.

### Creating Buffers

#### Buffer.alloc(size[, fill[, encoding]])

Allocates a new Buffer of the specified size. If `fill` is provided, the buffer will be filled with that value.

```javascript
// Create a 10-byte buffer filled with zeros
const buf1 = Buffer.alloc(10);
console.log(buf1); // <Buffer 00 00 00 00 00 00 00 00 00 00>

// Create a 10-byte buffer filled with 0x41 ('A')
const buf2 = Buffer.alloc(10, 0x41);
console.log(buf2.toString()); // 'AAAAAAAAAA'

// Create a buffer filled with a string pattern
const buf3 = Buffer.alloc(10, 'abc');
console.log(buf3.toString()); // 'abcabcabca'
```

#### Buffer.allocUnsafe(size)

Allocates a new Buffer of the specified size without initializing the memory.

```javascript
const buf = Buffer.allocUnsafe(10);
// Note: contents are uninitialized and may contain old data
```

#### Buffer.from(source[, encoding])

Creates a new Buffer from various sources:

```javascript
// From string (default UTF-8)
const buf1 = Buffer.from('Hello, World!');

// From string with encoding
const buf2 = Buffer.from('48656c6c6f', 'hex');  // 'Hello'
const buf3 = Buffer.from('SGVsbG8=', 'base64'); // 'Hello'

// From array of bytes
const buf4 = Buffer.from([0x48, 0x65, 0x6c, 0x6c, 0x6f]); // 'Hello'

// From another Buffer
const buf5 = Buffer.from(buf1);

// From ArrayBuffer
const arrayBuffer = new ArrayBuffer(4);
const buf6 = Buffer.from(arrayBuffer);
```

### Supported Encodings

- `utf8` / `utf-8` - Multi-byte encoded Unicode characters (default)
- `hex` - Encode each byte as two hexadecimal characters
- `base64` - Base64 encoding
- `base64url` - URL-safe Base64 encoding
- `latin1` / `binary` - Latin-1 encoding (ISO-8859-1)
- `ascii` - 7-bit ASCII

### Converting Buffers

#### toString([encoding[, start[, end]]])

Decodes a Buffer to a string using the specified encoding.

```javascript
const buf = Buffer.from('Hello, World!');

console.log(buf.toString());        // 'Hello, World!' (UTF-8 default)
console.log(buf.toString('hex'));   // '48656c6c6f2c20576f726c6421'
console.log(buf.toString('base64'));// 'SGVsbG8sIFdvcmxkIQ=='

// Partial conversion
console.log(buf.toString('utf8', 0, 5)); // 'Hello'
```

#### toJSON()

Returns a JSON representation of the Buffer.

```javascript
const buf = Buffer.from([1, 2, 3]);
console.log(buf.toJSON());
// { type: 'Buffer', data: [1, 2, 3] }
```

### Buffer Operations

#### concat(list[, totalLength])

Concatenates multiple Buffers into one.

```javascript
const buf1 = Buffer.from('Hello');
const buf2 = Buffer.from(' ');
const buf3 = Buffer.from('World');

const combined = Buffer.concat([buf1, buf2, buf3]);
console.log(combined.toString()); // 'Hello World'
```

#### compare(buf1, buf2)

Compares two Buffers, useful for sorting.

```javascript
const buf1 = Buffer.from('ABC');
const buf2 = Buffer.from('ABD');

console.log(Buffer.compare(buf1, buf1)); // 0  (equal)
console.log(Buffer.compare(buf1, buf2)); // -1 (buf1 < buf2)
console.log(Buffer.compare(buf2, buf1)); // 1  (buf2 > buf1)
```

#### copy(target[, targetStart[, sourceStart[, sourceEnd]]])

Copies data from one Buffer to another.

```javascript
const buf1 = Buffer.from([1, 2, 3, 4]);
const buf2 = Buffer.alloc(4);

buf1.copy(buf2);
console.log(buf2); // <Buffer 01 02 03 04>
```

#### fill(value[, offset[, end[, encoding]]])

Fills a Buffer with the specified value.

```javascript
const buf = Buffer.alloc(10);
buf.fill(0x42);
console.log(buf.toString()); // 'BBBBBBBBBB'

buf.fill('abc');
console.log(buf.toString()); // 'abcabcabca'
```

#### slice(start[, end]) / subarray(start[, end])

Returns a new Buffer that references the same memory.

```javascript
const buf = Buffer.from('Hello World');
const slice = buf.slice(0, 5);
console.log(slice.toString()); // 'Hello'
```

### Searching

#### indexOf(value[, byteOffset[, encoding]])

Returns the index of the first occurrence of value.

```javascript
const buf = Buffer.from('Hello World');
console.log(buf.indexOf('World')); // 6
console.log(buf.indexOf('xyz'));   // -1
```

#### lastIndexOf(value[, byteOffset[, encoding]])

Returns the index of the last occurrence of value.

```javascript
const buf = Buffer.from('Hello World World');
console.log(buf.lastIndexOf('World')); // 12
```

#### includes(value[, byteOffset[, encoding]])

Returns true if the Buffer contains the specified value.

```javascript
const buf = Buffer.from('Hello World');
console.log(buf.includes('World')); // true
console.log(buf.includes('xyz'));   // false
```

### Comparing

#### equals(otherBuffer)

Returns true if the two Buffers have exactly the same bytes.

```javascript
const buf1 = Buffer.from('ABC');
const buf2 = Buffer.from('ABC');
const buf3 = Buffer.from('ABD');

console.log(buf1.equals(buf2)); // true
console.log(buf1.equals(buf3)); // false
```

### Reading and Writing Numbers

Buffer provides methods for reading and writing integers and floating-point numbers in both little-endian and big-endian byte order.

```javascript
const buf = Buffer.alloc(8);

// Write unsigned integers
buf.writeUInt8(0x12, 0);
buf.writeUInt16LE(0x3456, 1);
buf.writeUInt32BE(0x789ABCDE, 3);

// Read unsigned integers
console.log(buf.readUInt8(0));       // 0x12
console.log(buf.readUInt16LE(1));    // 0x3456
console.log(buf.readUInt32BE(3));    // 0x789ABCDE

// Signed integers
buf.writeInt8(-128, 0);
buf.writeInt16LE(-1000, 1);
buf.writeInt32BE(-100000, 3);

console.log(buf.readInt8(0));        // -128
console.log(buf.readInt16LE(1));     // -1000
console.log(buf.readInt32BE(3));     // -100000

// Floating-point numbers
buf.writeFloatLE(3.14, 0);
buf.writeDoubleLE(3.14159265359, 0);

console.log(buf.readFloatLE(0));
console.log(buf.readDoubleLE(0));
```

### Byte Swapping

```javascript
const buf = Buffer.from([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);

buf.swap16(); // Swap pairs of bytes
buf.swap32(); // Swap groups of 4 bytes
buf.swap64(); // Swap groups of 8 bytes
```

### Static Methods

#### byteLength(string[, encoding])

Returns the byte length of a string in the specified encoding.

```javascript
console.log(Buffer.byteLength('Hello'));       // 5 (UTF-8)
console.log(Buffer.byteLength('Hello', 'hex')); // 2 (hex decodes to fewer bytes)
console.log(Buffer.byteLength('中文'));         // 6 (UTF-8 multi-byte)
```

#### isEncoding(encoding)

Returns true if the encoding is supported.

```javascript
console.log(Buffer.isEncoding('utf8'));    // true
console.log(Buffer.isEncoding('hex'));     // true
console.log(Buffer.isEncoding('invalid')); // false
```

#### isBuffer(obj)

Returns true if the object is a Buffer.

```javascript
const buf = Buffer.from('test');
const arr = new Uint8Array(4);

console.log(Buffer.isBuffer(buf)); // true
console.log(Buffer.isBuffer(arr)); // false
console.log(Buffer.isBuffer('test')); // false
```

### Constants

```javascript
import { Buffer, kMaxLength, constants } from 'node:buffer';

console.log(kMaxLength);           // Maximum buffer size (0xFFFFFFFF)
console.log(constants.MAX_LENGTH); // Same as kMaxLength
```

### Complete Example

```javascript
import { Buffer } from 'node:buffer';

// Create a buffer and write data
const buf = Buffer.alloc(256);

// Write a header
buf.writeUInt32BE(0x4A535449, 0);  // 'JSTI' magic number
buf.writeUInt16BE(1, 4);           // Version
buf.writeUInt32BE(Date.now() / 1000, 6); // Timestamp

// Write payload
const payload = 'Hello, World!';
const payloadLength = Buffer.byteLength(payload);
buf.writeUInt16BE(payloadLength, 10);
buf.write(payload, 12);

// Read back the data
console.log('Magic:', buf.readUInt32BE(0).toString(16));
console.log('Version:', buf.readUInt16BE(4));
console.log('Timestamp:', buf.readUInt32BE(6));
console.log('Payload length:', buf.readUInt16BE(10));
console.log('Payload:', buf.toString('utf8', 12, 12 + payloadLength));

// Convert to hex for debugging
console.log('Raw:', buf.slice(0, 12 + payloadLength).toString('hex'));
```

### Comparison with Node.js

jstime's Buffer API implements a subset of Node.js's Buffer:

| Feature | jstime | Node.js |
|---------|--------|---------|
| `Buffer.alloc()` | ✅ | ✅ |
| `Buffer.allocUnsafe()` | ✅ | ✅ |
| `Buffer.from()` | ✅ | ✅ |
| `Buffer.concat()` | ✅ | ✅ |
| `Buffer.compare()` | ✅ | ✅ |
| `Buffer.isBuffer()` | ✅ | ✅ |
| `Buffer.isEncoding()` | ✅ | ✅ |
| `Buffer.byteLength()` | ✅ | ✅ |
| `toString()` | ✅ | ✅ |
| `toJSON()` | ✅ | ✅ |
| `copy()` | ✅ | ✅ |
| `fill()` | ✅ | ✅ |
| `slice()` / `subarray()` | ✅ | ✅ |
| `indexOf()` / `lastIndexOf()` | ✅ | ✅ |
| `includes()` | ✅ | ✅ |
| `equals()` | ✅ | ✅ |
| `compare()` (instance) | ✅ | ✅ |
| `write()` | ✅ | ✅ |
| Read/Write methods | ✅ | ✅ |
| `swap16/32/64()` | ✅ | ✅ |
| `readBigInt64LE/BE()` | ❌ | ✅ |
| `writeBigInt64LE/BE()` | ❌ | ✅ |
| `Buffer.poolSize` | ✅ (getter only) | ✅ |
| `transcode()` | ❌ | ✅ |

## File System API

jstime provides a comprehensive Node.js-compatible file system API through the `node:fs/promises` module. This provides promise-based access to essential file operations.

**📁 Examples:** 
- Basic usage: [examples/fs-demo.js](../../examples/fs-demo.js)
- Complete API: [examples/fs-complete-demo.js](../../examples/fs-complete-demo.js)

### Supported APIs

**Primary (Essential):**
- `readFile(path, options?)` - Read the entire contents of a file
- `writeFile(path, data, options?)` - Write data to a file
- `appendFile(path, data, options?)` - Append data to a file
- `readdir(path, options?)` - Read the contents of a directory
- `mkdir(path, options?)` - Create a directory
- `rmdir(path, options?)` - Remove a directory
- `unlink(path)` - Delete a file
- `rename(oldPath, newPath)` - Rename a file or directory
- `copyFile(src, dest, mode?)` - Copy a file
- `stat(path, options?)` - Get file statistics
- `access(path, mode?)` - Test file accessibility
- `constants` - File system constants (F_OK, R_OK, W_OK, X_OK)

**Secondary (Additional):**
- `rm(path, options?)` - Remove files and directories (modern alternative)
- `truncate(path, len?)` - Truncate a file to a specified length
- `realpath(path, options?)` - Resolve path to an absolute path
- `chmod(path, mode)` - Change file permissions (Unix-like systems)
- `mkdtemp(prefix, options?)` - Create a unique temporary directory
- `readlink(path, options?)` - Read the target of a symbolic link
- `symlink(target, path, type?)` - Create a symbolic link
- `lstat(path, options?)` - Get file statistics without following symlinks
- `chown(path, uid, gid)` - Change file ownership (Unix-like systems)
- `utimes(path, atime, mtime)` - Change file access and modification times

### Usage

```javascript
import { readFile, writeFile, appendFile, mkdir, rm, stat, mkdtemp, symlink } from 'node:fs/promises';
// or
import * as fs from 'node:fs/promises';
```

### Reading Files

#### Read file as text

```javascript
import { readFile } from 'node:fs/promises';

// Simple string encoding
const text = await readFile('./README.md', 'utf-8');
console.log(text);

// Using options object
const content = await readFile('./file.txt', { encoding: 'utf-8' });
console.log(content);
```

#### Read file as buffer

```javascript
import { readFile } from 'node:fs/promises';

// Returns Uint8Array when no encoding is specified
const buffer = await readFile('./image.png');
console.log(buffer instanceof Uint8Array); // true
console.log(buffer.length); // file size in bytes
```

### Writing Files

```javascript
import { writeFile } from 'node:fs/promises';

// Write text
await writeFile('./output.txt', 'Hello, World!', 'utf-8');

// Write buffer
const buffer = new Uint8Array([72, 101, 108, 108, 111]);
await writeFile('./output.bin', buffer);
```

### Appending to Files

```javascript
import { appendFile } from 'node:fs/promises';

// Append text to a file
await appendFile('./log.txt', 'New log entry\n', 'utf-8');

// Append buffer
const buffer = new Uint8Array([72, 101, 108, 108, 111]);
await appendFile('./data.bin', buffer);

// Creates file if it doesn't exist
await appendFile('./new-file.txt', 'First line\n');
```

### Directory Operations

#### Creating directories

```javascript
import { mkdir } from 'node:fs/promises';

// Create single directory
await mkdir('./new-dir');

// Create nested directories (recursive)
await mkdir('./path/to/nested/dir', { recursive: true });
```

#### Listing directories

```javascript
import { readdir } from 'node:fs/promises';

// List directory contents
const files = await readdir('./src');
console.log('Files:', files); // Array of file/directory names

// Process files
for (const file of files) {
  console.log(file);
}
```

#### Removing directories

```javascript
import { rmdir } from 'node:fs/promises';

// Remove empty directory
await rmdir('./empty-dir');

// Remove directory and all contents (recursive)
await rmdir('./dir-with-files', { recursive: true });
```

### File Operations

#### Deleting files

```javascript
import { unlink, rm } from 'node:fs/promises';

// Delete a file with unlink
await unlink('./unwanted-file.txt');

// Or use modern rm() - works for files and directories
await rm('./unwanted-file.txt');

// Remove directory and all contents
await rm('./directory', { recursive: true });
```

#### Renaming files

```javascript
import { rename } from 'node:fs/promises';

await rename('./old-name.txt', './new-name.txt');
```

#### Copying files

```javascript
import { copyFile } from 'node:fs/promises';

await copyFile('./source.txt', './destination.txt');
```

#### Truncating files

```javascript
import { truncate } from 'node:fs/promises';

// Truncate file to 100 bytes
await truncate('./file.txt', 100);

// Truncate file to 0 bytes (empty the file)
await truncate('./file.txt');
```

### File Information

#### Getting file statistics

```javascript
import { stat } from 'node:fs/promises';

const stats = await stat('./file.txt');
console.log('Size:', stats.size);
console.log('Is file:', stats.isFile);
console.log('Is directory:', stats.isDirectory);
console.log('Is symlink:', stats.isSymbolicLink);
console.log('Modified time (ms):', stats.mtimeMs);
```

#### Resolving absolute paths

```javascript
import { realpath } from 'node:fs/promises';

// Resolve relative path to absolute path
const absolutePath = await realpath('./some/relative/path.txt');
console.log('Absolute path:', absolutePath);
```

#### Changing file permissions

```javascript
import { chmod } from 'node:fs/promises';

// Set file to read/write for owner, read-only for others
await chmod('./file.txt', 0o644);

// Set file to executable for owner
await chmod('./script.sh', 0o755);
```

**Note:** `chmod()` is only available on Unix-like systems (Linux, macOS).

#### Working with symbolic links

```javascript
import { symlink, readlink, lstat } from 'node:fs/promises';

// Create a symbolic link
await symlink('./target.txt', './link.txt');

// Read the link target
const target = await readlink('./link.txt');
console.log('Link points to:', target);

// Get stats without following the link
const stats = await lstat('./link.txt');
console.log('Is symlink:', stats.isSymbolicLink); // true
```

#### Creating temporary directories

```javascript
import { mkdtemp, writeFile, rmdir } from 'node:fs/promises';

// Create a unique temporary directory
const tmpDir = await mkdtemp('/tmp/myapp-');
console.log('Temp dir:', tmpDir); // e.g., /tmp/myapp-4a5b6c

// Use the directory
await writeFile(`${tmpDir}/data.txt`, 'temporary data');

// Clean up
await rmdir(tmpDir, { recursive: true });
```

#### Changing file ownership

```javascript
import { chown } from 'node:fs/promises';

// Change file ownership (Unix-like systems, requires permissions)
await chown('./file.txt', 1000, 1000);
```

**Note:** `chown()` is only available on Unix-like systems and typically requires root privileges.

#### Changing file timestamps

```javascript
import { utimes } from 'node:fs/promises';

// Set access and modification times
const now = Date.now();
const yesterday = now - 86400000; // 24 hours ago

await utimes('./file.txt', yesterday, yesterday);

// Or use Date objects
await utimes('./file.txt', new Date(), new Date());
```

#### Testing file accessibility

```javascript
import { access, constants } from 'node:fs/promises';

// Check if file exists
try {
  await access('./file.txt', constants.F_OK);
  console.log('File exists');
} catch (e) {
  console.log('File does not exist');
}

// Constants available
console.log(constants.F_OK); // 0 - File exists
console.log(constants.R_OK); // 4 - File is readable
console.log(constants.W_OK); // 2 - File is writable
console.log(constants.X_OK); // 1 - File is executable
```

### Error Handling

All file system operations can throw errors if the file or directory doesn't exist, or if there are permission issues:

```javascript
import { readFile, writeFile, mkdir } from 'node:fs/promises';

try {
  const data = await readFile('./nonexistent.txt', 'utf-8');
} catch (error) {
  console.error('Failed to read file:', error.message);
}

try {
  await writeFile('/root/protected.txt', 'data');
} catch (error) {
  console.error('Permission denied:', error.message);
}
```

### API Reference

#### `readFile(path, options?)`

Reads the entire contents of a file.

**Parameters:**
- `path` (string | Buffer | URL): The path to the file
- `options` (object | string, optional):
  - `encoding` (string): If specified, returns a string. Defaults to null (returns Buffer)
  - `flag` (string): File system flag. Defaults to 'r'

**Returns:** Promise<string | Uint8Array>

**Supported encodings:** 'utf-8', 'utf8'

#### `writeFile(path, data, options?)`

Writes data to a file, replacing the file if it already exists.

**Parameters:**
- `path` (string | Buffer | URL): The path to the file
- `data` (string | Uint8Array): Data to write
- `options` (object | string, optional):
  - `encoding` (string): Character encoding. Defaults to 'utf8' for strings
  - `flag` (string): File system flag. Defaults to 'w'

**Returns:** Promise<void>

#### `appendFile(path, data, options?)`

Appends data to a file, creating the file if it doesn't exist.

**Parameters:**
- `path` (string | Buffer | URL): The path to the file
- `data` (string | Uint8Array): Data to append
- `options` (object | string, optional):
  - `encoding` (string): Character encoding. Defaults to 'utf8' for strings
  - `flag` (string): File system flag. Defaults to 'a'

**Returns:** Promise<void>

#### `readdir(path, options?)`

Reads the contents of a directory.

**Parameters:**
- `path` (string | Buffer | URL): The path to the directory
- `options` (object | string, optional):
  - `encoding` (string): Character encoding for file names. Defaults to 'utf8'
  - `withFileTypes` (boolean): Not yet supported. Defaults to false

**Returns:** Promise<string[]>

Returns an array of filenames in the directory (excluding '.' and '..').

#### `mkdir(path, options?)`

Creates a directory.

**Parameters:**
- `path` (string | Buffer | URL): The path to create
- `options` (object, optional):
  - `recursive` (boolean): Create parent directories if needed. Defaults to false

**Returns:** Promise<void>

#### `rmdir(path, options?)`

Removes a directory.

**Parameters:**
- `path` (string | Buffer | URL): The path to remove
- `options` (object, optional):
  - `recursive` (boolean): Remove directory and all contents. Defaults to false

**Returns:** Promise<void>

#### `unlink(path)`

Deletes a file.

**Parameters:**
- `path` (string | Buffer | URL): The path to the file

**Returns:** Promise<void>

#### `rename(oldPath, newPath)`

Renames a file or directory.

**Parameters:**
- `oldPath` (string | Buffer | URL): The old path
- `newPath` (string | Buffer | URL): The new path

**Returns:** Promise<void>

#### `copyFile(src, dest, mode?)`

Copies a file.

**Parameters:**
- `src` (string | Buffer | URL): Source path
- `dest` (string | Buffer | URL): Destination path
- `mode` (number, optional): Copy mode flags

**Returns:** Promise<void>

#### `stat(path, options?)`

Gets file statistics.

**Parameters:**
- `path` (string | Buffer | URL): The path to stat
- `options` (object, optional): Options

**Returns:** Promise<Stats>

Returns a Stats object with properties:
- `isFile` (boolean): True if the path is a file
- `isDirectory` (boolean): True if the path is a directory
- `isSymbolicLink` (boolean): True if the path is a symbolic link
- `size` (number): File size in bytes
- `mtimeMs` (number): Last modified time in milliseconds since Unix epoch

#### `access(path, mode?)`

Tests file accessibility.

**Parameters:**
- `path` (string | Buffer | URL): The path to test
- `mode` (number, optional): Accessibility mode to check

**Returns:** Promise<void>

Throws an error if the file is not accessible.

#### `rm(path, options?)`

Removes files and directories (modern alternative to `unlink`/`rmdir`).

**Parameters:**
- `path` (string | Buffer | URL): The path to remove
- `options` (object, optional):
  - `recursive` (boolean): Remove directory and all contents. Defaults to false

**Returns:** Promise<void>

#### `truncate(path, len?)`

Truncates a file to a specified length.

**Parameters:**
- `path` (string | Buffer | URL): The path to the file
- `len` (number, optional): Target length in bytes. Defaults to 0

**Returns:** Promise<void>

#### `realpath(path, options?)`

Resolves a path to an absolute path, resolving symbolic links.

**Parameters:**
- `path` (string | Buffer | URL): The path to resolve
- `options` (object, optional): Options

**Returns:** Promise<string>

Returns the resolved absolute path.

#### `chmod(path, mode)`

Changes file permissions (Unix-like systems only).

**Parameters:**
- `path` (string | Buffer | URL): The path to the file
- `mode` (number): File mode (permissions) as octal number (e.g., 0o644)

**Returns:** Promise<void>

**Note:** Not supported on Windows. Will throw an error on non-Unix platforms.

#### `mkdtemp(prefix, options?)`

Creates a unique temporary directory.

**Parameters:**
- `prefix` (string): Directory name prefix
- `options` (object | string, optional):
  - `encoding` (string): Character encoding. Defaults to 'utf8'

**Returns:** Promise<string>

Returns the path to the created temporary directory.

#### `readlink(path, options?)`

Reads the target of a symbolic link.

**Parameters:**
- `path` (string | Buffer | URL): Path to the symbolic link
- `options` (object | string, optional):
  - `encoding` (string): Character encoding. Defaults to 'utf8'

**Returns:** Promise<string>

Returns the target path that the symbolic link points to.

#### `symlink(target, path, type?)`

Creates a symbolic link.

**Parameters:**
- `target` (string | Buffer | URL): Target path to link to
- `path` (string | Buffer | URL): Path of the symbolic link to create
- `type` (string, optional): Type of symlink ('file', 'dir', 'junction') - Windows only

**Returns:** Promise<void>

**Note:** On Windows, requires administrator privileges or Developer Mode.

#### `lstat(path, options?)`

Gets file statistics without following symbolic links.

**Parameters:**
- `path` (string | Buffer | URL): The path to stat
- `options` (object, optional): Options

**Returns:** Promise<Stats>

Returns a Stats object. Unlike `stat()`, if the path is a symbolic link, the stats are for the link itself, not the target.

#### `chown(path, uid, gid)`

Changes file ownership (Unix-like systems only).

**Parameters:**
- `path` (string | Buffer | URL): The path to the file
- `uid` (number): User ID
- `gid` (number): Group ID

**Returns:** Promise<void>

**Note:** Only supported on Unix-like systems. Requires appropriate permissions.

#### `utimes(path, atime, mtime)`

Changes file access and modification times.

**Parameters:**
- `path` (string | Buffer | URL): The path to the file
- `atime` (number | Date): Access time (milliseconds since epoch or Date object)
- `mtime` (number | Date): Modification time (milliseconds since epoch or Date object)

**Returns:** Promise<void>

#### `constants`

File system constants for use with `access()`:

- `F_OK` (0): File exists
- `R_OK` (4): File is readable  
- `W_OK` (2): File is writable
- `X_OK` (1): File is executable

### Example: Complete File Processing

```javascript
import { 
  readFile, 
  writeFile, 
  appendFile,
  readdir, 
  mkdir, 
  stat, 
  copyFile 
} from 'node:fs/promises';

// Create output directory
await mkdir('./output', { recursive: true });

// Create a summary log file
await writeFile('./output/summary.txt', 'Processing Summary\n==================\n\n');

// Read all JavaScript files in a directory
const files = await readdir('./src');
const jsFiles = files.filter(f => f.endsWith('.js'));

console.log(`Found ${jsFiles.length} JavaScript files`);

// Process each file
for (const file of jsFiles) {
  const inputPath = `./src/${file}`;
  const outputPath = `./output/${file}`;
  
  // Get file stats
  const stats = await stat(inputPath);
  console.log(`${file}: ${stats.size} bytes`);
  
  // Append to summary log
  await appendFile('./output/summary.txt', `${file}: ${stats.size} bytes\n`);
  
  // Read and transform content
  const content = await readFile(inputPath, 'utf-8');
  const transformed = content.toUpperCase();
  
  // Write to output
  await writeFile(outputPath, transformed, 'utf-8');
  console.log(`Processed ${file}`);
}

console.log('Processing complete!');
```

## UDP/Datagram Sockets API (dgram)

jstime provides a Node.js-compatible UDP datagram socket API through the `node:dgram` module. This provides access to UDP networking for sending and receiving datagrams.

### Supported APIs

- `createSocket(type, [callback])` - Create a UDP socket
- `Socket` class with:
  - `bind(port, [address], [callback])` - Bind to a port and address
  - `send(msg, [offset], [length], port, [address], [callback])` - Send data
  - `close([callback])` - Close the socket
  - `address()` - Get the bound address info
  - `setBroadcast(flag)` - Enable/disable broadcast
  - `setTTL(ttl)` - Set IP TTL
  - `setMulticastTTL(ttl)` - Set multicast TTL
  - `setMulticastLoopback(flag)` - Enable/disable multicast loopback
  - `addMembership(multicastAddress, [multicastInterface])` - Join multicast group
  - `dropMembership(multicastAddress, [multicastInterface])` - Leave multicast group
  - `getRecvBufferSize()` / `setRecvBufferSize(size)` - Receive buffer size
  - `getSendBufferSize()` / `setSendBufferSize(size)` - Send buffer size
  - `ref()` / `unref()` - Reference counting for event loop

### Usage

```javascript
import dgram from 'node:dgram';
// or
import { createSocket } from 'node:dgram';
```

### Creating a Socket

```javascript
import dgram from 'node:dgram';

// Create a UDP4 socket
const socket = dgram.createSocket('udp4');

// Create with options
const socket2 = dgram.createSocket({
  type: 'udp4',
  recvBufferSize: 1024 * 1024
});

// Create with message callback
const socket3 = dgram.createSocket('udp4', (msg, rinfo) => {
  console.log(`Received: ${msg} from ${rinfo.address}:${rinfo.port}`);
});
```

### Binding and Listening

```javascript
import dgram from 'node:dgram';

const socket = dgram.createSocket('udp4');

socket.on('listening', () => {
  const address = socket.address();
  console.log(`Server listening on ${address.address}:${address.port}`);
});

socket.on('message', (msg, rinfo) => {
  console.log(`Received ${msg.length} bytes from ${rinfo.address}:${rinfo.port}`);
});

// Bind to port 41234
socket.bind(41234);

// Or bind to specific address
socket.bind({
  port: 41234,
  address: '127.0.0.1'
});
```

### Sending Data

```javascript
import dgram from 'node:dgram';

const socket = dgram.createSocket('udp4');

// Send string data
socket.send('Hello, World!', 41234, '127.0.0.1', (err) => {
  if (err) console.error(err);
  else console.log('Message sent');
});

// Send buffer data
const buffer = new Uint8Array([0x48, 0x65, 0x6c, 0x6c, 0x6f]);
socket.send(buffer, 0, buffer.length, 41234, '127.0.0.1');

// Send array of buffers
socket.send(['Hello', ' ', 'World'], 41234, '127.0.0.1');
```

### Echo Server Example

```javascript
import dgram from 'node:dgram';

const server = dgram.createSocket('udp4');

server.on('error', (err) => {
  console.error(`Server error: ${err.message}`);
  server.close();
});

server.on('message', (msg, rinfo) => {
  console.log(`Server received: ${msg} from ${rinfo.address}:${rinfo.port}`);
  // Echo the message back
  server.send(msg, rinfo.port, rinfo.address);
});

server.on('listening', () => {
  const address = server.address();
  console.log(`Server listening on ${address.address}:${address.port}`);
});

server.bind(41234);
```

### Broadcast Example

```javascript
import dgram from 'node:dgram';

const socket = dgram.createSocket('udp4');

socket.bind(() => {
  socket.setBroadcast(true);
  
  const message = 'Broadcast message';
  socket.send(message, 41234, '255.255.255.255', (err) => {
    if (err) console.error(err);
    socket.close();
  });
});
```

### Multicast Example

```javascript
import dgram from 'node:dgram';

const socket = dgram.createSocket('udp4');
const MULTICAST_ADDR = '224.1.1.1';
const PORT = 5000;

socket.on('listening', () => {
  // Join multicast group
  socket.addMembership(MULTICAST_ADDR);
  console.log('Joined multicast group');
});

socket.on('message', (msg, rinfo) => {
  console.log(`Multicast message: ${msg} from ${rinfo.address}`);
});

socket.bind(PORT);

// Send to multicast group
setTimeout(() => {
  socket.send('Hello multicast!', PORT, MULTICAST_ADDR);
}, 1000);
```

### API Reference

#### `dgram.createSocket(options, [callback])`

Creates a dgram.Socket object.

**Parameters:**
- `options` (string | object): Socket type ('udp4' or 'udp6') or options object
  - `type` (string): 'udp4' or 'udp6'
  - `recvBufferSize` (number): Receive buffer size
  - `sendBufferSize` (number): Send buffer size
- `callback` (function, optional): Attached as listener for 'message' event

**Returns:** Socket

#### `socket.bind([port], [address], [callback])`

Binds the socket to a port and address.

**Parameters:**
- `port` (number, optional): Port to bind to. Default: 0 (OS assigns)
- `address` (string, optional): Address to bind to. Default: 0.0.0.0 (all interfaces)
- `callback` (function, optional): Called when binding is complete

**Returns:** Socket (for chaining)

#### `socket.send(msg, [offset], [length], port, [address], [callback])`

Sends a datagram through the socket.

**Parameters:**
- `msg` (string | Uint8Array | Array): Message to send
- `offset` (number, optional): Offset in buffer
- `length` (number, optional): Number of bytes to send
- `port` (number): Destination port
- `address` (string, optional): Destination address. Default: '127.0.0.1'
- `callback` (function, optional): Called when message is sent

#### `socket.close([callback])`

Closes the socket.

**Parameters:**
- `callback` (function, optional): Called when socket is closed

#### `socket.address()`

Returns the address information for the socket.

**Returns:** { address: string, family: string, port: number }

#### `socket.setBroadcast(flag)`

Sets or clears the SO_BROADCAST socket option.

**Parameters:**
- `flag` (boolean): Enable or disable broadcast

#### `socket.setTTL(ttl)`

Sets the IP_TTL socket option (time-to-live for outgoing packets).

**Parameters:**
- `ttl` (number): TTL value (1-255)

#### `socket.setMulticastTTL(ttl)`

Sets the IP_MULTICAST_TTL socket option.

**Parameters:**
- `ttl` (number): Multicast TTL value (0-255)

#### `socket.setMulticastLoopback(flag)`

Sets or clears the IP_MULTICAST_LOOP socket option.

**Parameters:**
- `flag` (boolean): Enable or disable multicast loopback

#### `socket.addMembership(multicastAddress, [multicastInterface])`

Tells the kernel to join a multicast group.

**Parameters:**
- `multicastAddress` (string): Multicast group address
- `multicastInterface` (string, optional): Interface address. Default: 0.0.0.0

#### `socket.dropMembership(multicastAddress, [multicastInterface])`

Instructs the kernel to leave a multicast group.

**Parameters:**
- `multicastAddress` (string): Multicast group address
- `multicastInterface` (string, optional): Interface address. Default: 0.0.0.0

### Events

The Socket class extends EventTarget and emits the following events:

- `'listening'` - Emitted when the socket is bound and ready to receive data
- `'message'` - Emitted when a datagram is received
  - Event properties: `data` (Uint8Array), `rinfo` ({ address, family, port, size })
- `'error'` - Emitted when an error occurs
  - Event property: `error` (Error)
- `'close'` - Emitted when the socket is closed

### Comparison with Node.js

jstime's dgram API implements a subset of Node.js's dgram module:

| Feature | jstime | Node.js |
|---------|--------|---------|
| `createSocket()` | ✅ | ✅ |
| `socket.bind()` | ✅ | ✅ |
| `socket.send()` | ✅ | ✅ |
| `socket.close()` | ✅ | ✅ |
| `socket.address()` | ✅ | ✅ |
| `socket.setBroadcast()` | ✅ | ✅ |
| `socket.setTTL()` | ✅ | ✅ |
| `socket.setMulticastTTL()` | ✅ | ✅ |
| `socket.setMulticastLoopback()` | ✅ | ✅ |
| `socket.addMembership()` | ✅ | ✅ |
| `socket.dropMembership()` | ✅ | ✅ |
| `socket.getRecvBufferSize()` | ✅ | ✅ |
| `socket.setRecvBufferSize()` | ✅ | ✅ |
| `socket.getSendBufferSize()` | ✅ | ✅ |
| `socket.setSendBufferSize()` | ✅ | ✅ |
| `socket.ref()` / `unref()` | ✅ (no-op) | ✅ |
| `socket.connect()` | ❌ | ✅ |
| `socket.disconnect()` | ❌ | ✅ |
| `socket.remoteAddress()` | ❌ | ✅ |
| IPv6 multicast | ❌ | ✅ |
| Source-specific multicast | ❌ | ✅ |

**Note:** The `ref()` and `unref()` methods are implemented as no-ops for compatibility. In jstime, sockets don't automatically keep the event loop running.

## WebAssembly
