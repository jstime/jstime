// Node.js fs/promises API
// Minimal implementation compatible with Node.js

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ readFile: _readFile, readDir: _readDir }) => {
  // Helper to convert synchronous operations to promises
  function promisify(fn, ...args) {
    return new Promise((resolve, reject) => {
      try {
        const result = fn(...args);
        resolve(result);
      } catch (error) {
        reject(error);
      }
    });
  }

  /**
   * Reads the entire contents of a file.
   * @param {string | Buffer | URL} path - filename or file descriptor
   * @param {Object | string} options - encoding or options object
   * @returns {Promise<string | Buffer>}
   */
  function readFile(path, options) {
    let encoding = null;
    let flag = 'r';

    // Handle options parameter
    if (typeof options === 'string') {
      encoding = options;
    } else if (options && typeof options === 'object') {
      encoding = options.encoding || null;
      flag = options.flag || 'r';
    }

    return promisify(_readFile, path, encoding);
  }

  /**
   * Reads the contents of a directory.
   * @param {string | Buffer | URL} path - path to directory
   * @param {Object | string} options - encoding or options object
   * @returns {Promise<string[]>}
   */
  function readdir(path, options) {
    let encoding = 'utf8';
    let withFileTypes = false;

    // Handle options parameter
    if (typeof options === 'string') {
      encoding = options;
    } else if (options && typeof options === 'object') {
      encoding = options.encoding || 'utf8';
      withFileTypes = options.withFileTypes || false;
    }

    if (withFileTypes) {
      return Promise.reject(new Error('withFileTypes option is not yet supported'));
    }

    return promisify(_readDir, path);
  }

  // Export the fs/promises API
  const fsPromises = {
    readFile,
    readdir,
  };

  // Make it available via import
  globalThis.__node_modules = globalThis.__node_modules || {};
  globalThis.__node_modules['node:fs/promises'] = fsPromises;
  globalThis.__node_modules['fs/promises'] = fsPromises;
});
