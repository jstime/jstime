// Test listenerCount and eventNames
import { EventEmitter } from 'node:events';

const emitter = new EventEmitter();

emitter.on('foo', () => {});
emitter.on('foo', () => {});
emitter.on('bar', () => {});

const fooCount = emitter.listenerCount('foo') === 2;
const barCount = emitter.listenerCount('bar') === 1;
const eventNames = emitter.eventNames().sort().join(',') === 'bar,foo';

globalThis.testListenerCount = fooCount && barCount && eventNames;
