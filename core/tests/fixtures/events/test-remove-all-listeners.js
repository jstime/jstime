// Test removeAllListeners
import { EventEmitter } from 'node:events';

const emitter = new EventEmitter();

emitter.on('foo', () => {});
emitter.on('foo', () => {});
emitter.on('bar', () => {});

emitter.removeAllListeners('foo');

const fooRemoved = emitter.listenerCount('foo') === 0;
const barStillThere = emitter.listenerCount('bar') === 1;

globalThis.testRemoveAllListeners = fooRemoved && barStillThere;
