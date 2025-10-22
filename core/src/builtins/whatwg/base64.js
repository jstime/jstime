// Base64 encoding/decoding
// https://html.spec.whatwg.org/multipage/webappapis.html#atob
// https://html.spec.whatwg.org/multipage/webappapis.html#btoa

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ atob: _atob, btoa: _btoa }) => {
  // Make atob available globally
  Object.defineProperty(globalThis, 'atob', {
    value: _atob,
    writable: true,
    enumerable: false,
    configurable: true,
  });

  // Make btoa available globally
  Object.defineProperty(globalThis, 'btoa', {
    value: _btoa,
    writable: true,
    enumerable: false,
    configurable: true,
  });
});
