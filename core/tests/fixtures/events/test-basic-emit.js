// Test basic emit and on functionality
import { EventEmitter } from 'node:events';

const emitter = new EventEmitter();
let result = '';

emitter.on('test', (msg) => {
  result += msg;
});

emitter.emit('test', 'Hello');
emitter.emit('test', 'World');

globalThis.testBasicEmit = result === 'HelloWorld';
