// Test that EventEmitter exists and can be imported
import { EventEmitter } from 'node:events';

globalThis.testEventsModuleExists = typeof EventEmitter === 'function';
