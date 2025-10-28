# System APIs

This document describes the system-level APIs implemented in jstime for interacting with the operating system and file system.

## Table of Contents

- [Process API](#process-api)
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

## WebAssembly
