// Web Cryptography API
// https://w3c.github.io/webcrypto/

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ cryptoGetRandomValues, cryptoRandomUUID, cryptoSubtleDigest }) => {
  // SubtleCrypto object
  const subtle = {
    async digest(algorithm, data) {
      // Normalize algorithm to string
      let alg;
      if (typeof algorithm === 'string') {
        alg = algorithm;
      } else if (algorithm && typeof algorithm === 'object' && algorithm.name) {
        alg = algorithm.name;
      } else {
        throw new TypeError('Invalid algorithm');
      }

      // Normalize data to ArrayBuffer or ArrayBufferView
      if (data === null || data === undefined) {
        throw new TypeError('data is required');
      }

      // Call the native implementation (returns a Promise)
      return cryptoSubtleDigest(alg, data);
    },
  };

  // Crypto object
  const crypto = {
    getRandomValues(typedArray) {
      if (typedArray === null || typedArray === undefined) {
        throw new TypeError('getRandomValues: argument 1 is required');
      }

      // Call native implementation
      return cryptoGetRandomValues(typedArray);
    },

    randomUUID() {
      return cryptoRandomUUID();
    },

    // SubtleCrypto interface
    get subtle() {
      return subtle;
    },
  };

  // Make crypto available globally
  Object.defineProperty(globalThis, 'crypto', {
    value: crypto,
    writable: false,
    enumerable: true,
    configurable: false,
  });

  // Also expose SubtleCrypto constructor (for instanceof checks)
  Object.defineProperty(globalThis, 'SubtleCrypto', {
    value: function SubtleCrypto() {
      throw new TypeError('Illegal constructor');
    },
    writable: true,
    enumerable: false,
    configurable: true,
  });

  // Also expose Crypto constructor (for instanceof checks)
  Object.defineProperty(globalThis, 'Crypto', {
    value: function Crypto() {
      throw new TypeError('Illegal constructor');
    },
    writable: true,
    enumerable: false,
    configurable: true,
  });
});
