// setTimeout, setInterval, clearTimeout, clearInterval
// https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#timers

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ setTimeout: setTimeoutNative, setInterval: setIntervalNative, clearTimer }) => {
  globalThis.setTimeout = (callback, delay = 0, ...args) => {
    if (typeof callback !== 'function') {
      throw new TypeError('setTimeout requires a callback function');
    }
    // Clamp delay to 0 if negative
    const ms = Math.max(0, delay);
    return setTimeoutNative(() => callback(...args), ms);
  };

  globalThis.setInterval = (callback, delay = 0, ...args) => {
    if (typeof callback !== 'function') {
      throw new TypeError('setInterval requires a callback function');
    }
    // Clamp delay to 0 if negative
    const ms = Math.max(0, delay);
    return setIntervalNative(() => callback(...args), ms);
  };

  globalThis.clearTimeout = (id) => {
    if (id !== undefined && id !== null) {
      clearTimer(id);
    }
  };

  globalThis.clearInterval = (id) => {
    if (id !== undefined && id !== null) {
      clearTimer(id);
    }
  };
});
