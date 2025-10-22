// Node.js Process API
// Minimal implementation compatible with Node.js

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ getEnv, getArgv, getCwd, exit }) => {
  // Create the process object
  const process = {
    // Environment variables (lazy loaded)
    get env() {
      if (!this._env) {
        this._env = getEnv();
      }
      return this._env;
    },

    // Command-line arguments (lazy loaded)
    get argv() {
      if (!this._argv) {
        this._argv = getArgv();
      }
      return this._argv;
    },

    // Current working directory
    cwd() {
      return getCwd();
    },

    // Exit the process
    exit(code) {
      exit(code || 0);
    },
  };

  // Make process available globally
  globalThis.process = process;
});
