// Text encoding API
// https://encoding.spec.whatwg.org/

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ textEncoderEncode, textEncoderEncodeInto, textDecoderDecode }) => {
  // TextEncoder class
  class TextEncoder {
    constructor() {
      // TextEncoder only supports UTF-8
    }

    get encoding() {
      return 'utf-8';
    }

    encode(input = '') {
      return textEncoderEncode(String(input));
    }

    encodeInto(source, destination) {
      if (!(destination instanceof Uint8Array)) {
        throw new TypeError('Destination must be a Uint8Array');
      }
      return textEncoderEncodeInto(String(source), destination);
    }
  }

  // TextDecoder class
  class TextDecoder {
    #encoding;
    #fatal;
    #ignoreBOM;

    constructor(label = 'utf-8', options = {}) {
      // Normalize the encoding label
      const encoding = String(label).toLowerCase().trim();
      
      // Only UTF-8 is supported for now
      if (encoding !== 'utf-8' && encoding !== 'utf8' && encoding !== 'unicode-1-1-utf-8') {
        throw new RangeError(`The encoding label provided ('${label}') is invalid.`);
      }
      
      this.#encoding = 'utf-8';
      this.#fatal = Boolean(options.fatal);
      this.#ignoreBOM = Boolean(options.ignoreBOM);
    }

    get encoding() {
      return this.#encoding;
    }

    get fatal() {
      return this.#fatal;
    }

    get ignoreBOM() {
      return this.#ignoreBOM;
    }

    decode(input, options = {}) {
      // Allow undefined/null as empty input
      if (input === undefined || input === null) {
        return '';
      }
      
      return textDecoderDecode(input);
    }
  }

  // Make TextEncoder available globally
  Object.defineProperty(globalThis, 'TextEncoder', {
    value: TextEncoder,
    writable: true,
    enumerable: false,
    configurable: true,
  });

  // Make TextDecoder available globally
  Object.defineProperty(globalThis, 'TextDecoder', {
    value: TextDecoder,
    writable: true,
    enumerable: false,
    configurable: true,
  });
});
