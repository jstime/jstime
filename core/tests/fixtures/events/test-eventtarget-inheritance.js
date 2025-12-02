// Test EventTarget inheritance
import { EventEmitter } from 'node:events';

const emitter = new EventEmitter();

const isInstanceOfEventTarget = emitter instanceof EventTarget;
const isInstanceOfEventEmitter = emitter instanceof EventEmitter;
const hasAddEventListener = typeof emitter.addEventListener === 'function';
const hasRemoveEventListener = typeof emitter.removeEventListener === 'function';
const hasDispatchEvent = typeof emitter.dispatchEvent === 'function';

globalThis.testEventTargetInheritance = isInstanceOfEventTarget && 
  isInstanceOfEventEmitter && 
  hasAddEventListener && 
  hasRemoveEventListener && 
  hasDispatchEvent;
