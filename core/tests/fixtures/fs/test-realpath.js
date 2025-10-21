import { realpath } from 'node:fs/promises';

const path = await realpath('./tests/fixtures/fs/test-readfile.txt');
globalThis.testRealpath = path.includes('test-readfile.txt') && path.startsWith('/');
