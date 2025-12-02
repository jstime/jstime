// Test prependListener
import { EventEmitter } from 'node:events';

const emitter = new EventEmitter();
const order = [];

emitter.on('test', () => order.push(1));
emitter.prependListener('test', () => order.push(0));
emitter.emit('test');

globalThis.testPrependListener = order.join(',') === '0,1';
