import { readFile, readdir } from 'node:fs/promises';

globalThis.testNamedImports = typeof readFile === 'function' && typeof readdir === 'function';
