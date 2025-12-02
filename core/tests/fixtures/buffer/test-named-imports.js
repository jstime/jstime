// Test named imports
import { Buffer, kMaxLength, constants } from 'node:buffer';

const bufferExists = typeof Buffer === 'function';
const kMaxLengthExists = typeof kMaxLength === 'number';
const constantsExist = typeof constants === 'object' && typeof constants.MAX_LENGTH === 'number';
globalThis.testNamedImports = bufferExists && kMaxLengthExists && constantsExist;
