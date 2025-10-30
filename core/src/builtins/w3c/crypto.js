// Web Cryptography API
// https://w3c.github.io/webcrypto/

'use strict';

// eslint-disable-next-line no-unused-expressions
(({
  cryptoGetRandomValues,
  cryptoRandomUUID,
  cryptoSubtleDigest,
  cryptoSubtleSign,
  cryptoSubtleVerify,
  cryptoSubtleEncrypt,
  cryptoSubtleDecrypt,
  cryptoSubtleGenerateKey,
  cryptoSubtleImportKey,
  cryptoSubtleExportKey,
}) => {
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

    async sign(algorithm, key, data) {
      if (!algorithm) {
        throw new TypeError('algorithm is required');
      }
      if (!key) {
        throw new TypeError('key is required');
      }
      if (data === null || data === undefined) {
        throw new TypeError('data is required');
      }

      // Call the native implementation (returns a Promise)
      return cryptoSubtleSign(algorithm, key, data);
    },

    async verify(algorithm, key, signature, data) {
      if (!algorithm) {
        throw new TypeError('algorithm is required');
      }
      if (!key) {
        throw new TypeError('key is required');
      }
      if (signature === null || signature === undefined) {
        throw new TypeError('signature is required');
      }
      if (data === null || data === undefined) {
        throw new TypeError('data is required');
      }

      // Call the native implementation (returns a Promise)
      return cryptoSubtleVerify(algorithm, key, signature, data);
    },

    async encrypt(algorithm, key, data) {
      if (!algorithm) {
        throw new TypeError('algorithm is required');
      }
      if (!key) {
        throw new TypeError('key is required');
      }
      if (data === null || data === undefined) {
        throw new TypeError('data is required');
      }

      // Call the native implementation (returns a Promise)
      return cryptoSubtleEncrypt(algorithm, key, data);
    },

    async decrypt(algorithm, key, data) {
      if (!algorithm) {
        throw new TypeError('algorithm is required');
      }
      if (!key) {
        throw new TypeError('key is required');
      }
      if (data === null || data === undefined) {
        throw new TypeError('data is required');
      }

      // Call the native implementation (returns a Promise)
      return cryptoSubtleDecrypt(algorithm, key, data);
    },

    async generateKey(algorithm, extractable, keyUsages) {
      if (!algorithm) {
        throw new TypeError('algorithm is required');
      }
      if (typeof extractable !== 'boolean') {
        throw new TypeError('extractable must be a boolean');
      }
      if (!Array.isArray(keyUsages)) {
        throw new TypeError('keyUsages must be an array');
      }

      // Call the native implementation (returns a Promise)
      return cryptoSubtleGenerateKey(algorithm, extractable, keyUsages);
    },

    async importKey(format, keyData, algorithm, extractable, keyUsages) {
      if (!format) {
        throw new TypeError('format is required');
      }
      if (keyData === null || keyData === undefined) {
        throw new TypeError('keyData is required');
      }
      if (!algorithm) {
        throw new TypeError('algorithm is required');
      }
      if (typeof extractable !== 'boolean') {
        throw new TypeError('extractable must be a boolean');
      }
      if (!Array.isArray(keyUsages)) {
        throw new TypeError('keyUsages must be an array');
      }

      // Call the native implementation (returns a Promise)
      return cryptoSubtleImportKey(format, keyData, algorithm, extractable, keyUsages);
    },

    async exportKey(format, key) {
      if (!format) {
        throw new TypeError('format is required');
      }
      if (!key) {
        throw new TypeError('key is required');
      }

      // Call the native implementation (returns a Promise)
      return cryptoSubtleExportKey(format, key);
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
