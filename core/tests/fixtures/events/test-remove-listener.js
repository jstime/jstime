// Test removeListener/off functionality
import { EventEmitter } from 'node:events';

const emitter = new EventEmitter();
let count = 0;

const listener = () => {
  count++;
};

emitter.on('test', listener);
emitter.emit('test');
emitter.off('test', listener);
emitter.emit('test');

globalThis.testRemoveListener = count === 1;
