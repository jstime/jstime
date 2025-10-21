import { access } from 'node:fs/promises';

// Test accessing an existing file
await access('./tests/fixtures/fs/test-readfile.txt');
globalThis.testAccess = true;
