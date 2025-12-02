// Test default import
import EventEmitter from 'node:events';

const emitter = new EventEmitter();
let called = false;

emitter.on('test', () => {
  called = true;
});

emitter.emit('test');

globalThis.testDefaultImport = called;
