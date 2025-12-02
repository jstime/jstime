// Test named imports
import { EventEmitter, once, listenerCount, getEventListeners } from 'node:events';

const hasEventEmitter = typeof EventEmitter === 'function';
const hasOnce = typeof once === 'function';
const hasListenerCount = typeof listenerCount === 'function';
const hasGetEventListeners = typeof getEventListeners === 'function';

globalThis.testNamedImports = hasEventEmitter && hasOnce && hasListenerCount && hasGetEventListeners;
