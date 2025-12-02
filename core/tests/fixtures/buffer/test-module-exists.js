// Test that node:buffer module is available
import bufferModule from 'node:buffer';

globalThis.testBufferModuleExists = typeof bufferModule === 'object' && typeof bufferModule.Buffer === 'function';
