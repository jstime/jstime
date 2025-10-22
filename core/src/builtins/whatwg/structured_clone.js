// Structured Clone API
// https://html.spec.whatwg.org/multipage/structured-data.html#structured-cloning

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ structuredClone: _structuredClone }) => {
  // Make structuredClone available globally
  globalThis.structuredClone = _structuredClone;
});
