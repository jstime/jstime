// Performance API
// https://w3c.github.io/hr-time/

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ performanceNow, performanceTimeOrigin }) => {
  // Create the performance object
  const performance = {
    now() {
      return performanceNow();
    },
    get timeOrigin() {
      return performanceTimeOrigin();
    },
    // toJSON for serialization
    toJSON() {
      return {
        timeOrigin: this.timeOrigin,
      };
    },
  };

  // Make performance available globally
  Object.defineProperty(globalThis, 'performance', {
    value: performance,
    writable: false,
    enumerable: true,
    configurable: false,
  });
});
