import { useShared } from './module-a.js';
import { getShared } from './module-b.js';

// Use module A
useShared();
useShared();

// Check that module B sees the same counter state
const result = getShared();
globalThis.testResult = result;
