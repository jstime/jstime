import { sharedValue, counter } from './shared.js';

export function getShared() {
  return { value: sharedValue, count: counter };
}
