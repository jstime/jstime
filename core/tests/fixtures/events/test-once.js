// Test once functionality
import { EventEmitter } from 'node:events';

const emitter = new EventEmitter();
let count = 0;

emitter.once('test', () => {
  count++;
});

emitter.emit('test');
emitter.emit('test');
emitter.emit('test');

globalThis.testOnce = count === 1;
