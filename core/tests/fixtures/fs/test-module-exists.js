import * as fs from 'node:fs/promises';

globalThis.testFsModuleExists = typeof fs.readFile === 'function' && typeof fs.readdir === 'function';
