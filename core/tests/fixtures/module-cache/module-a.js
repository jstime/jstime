import { sharedValue, incrementCounter } from './shared.js';

export function useShared() {
  incrementCounter();
  return sharedValue;
}
