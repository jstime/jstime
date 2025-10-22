// queueMicrotask
// https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#microtask-queuing

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ queueMicrotask }) => {
  globalThis.queueMicrotask = (cb) => {
    if (typeof cb !== 'function') {
      throw new TypeError('queueMicrotask requires a callback function');
    }
    return queueMicrotask(cb);
  };
});
