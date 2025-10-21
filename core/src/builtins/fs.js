// Node.js fs/promises API
// Minimal implementation compatible with Node.js

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ readFile: _readFile, readDir: _readDir, writeFile: _writeFile, mkdir: _mkdir, rmdir: _rmdir, unlink: _unlink, rename: _rename, copyFile: _copyFile, stat: _stat, access: _access }) => {
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

  /**
   * Writes data to a file.
   * @param {string | Buffer | URL} path - filename
   * @param {string | Buffer} data - data to write
   * @param {Object | string} options - encoding or options object
   * @returns {Promise<void>}
   */
  function writeFile(path, data, options) {
    // Simply pass the data to the native function
    // If it's a string, Rust will handle it
    // If it's a Uint8Array, Rust will handle it too
    return promisify(_writeFile, path, data);
  }

  /**
   * Creates a directory.
   * @param {string | Buffer | URL} path - directory path
   * @param {Object} options - options object
   * @returns {Promise<void>}
   */
  function mkdir(path, options) {
    return promisify(_mkdir, path, options);
  }

  /**
   * Removes a directory.
   * @param {string | Buffer | URL} path - directory path
   * @param {Object} options - options object
   * @returns {Promise<void>}
   */
  function rmdir(path, options) {
    return promisify(_rmdir, path, options);
  }

  /**
   * Deletes a file.
   * @param {string | Buffer | URL} path - file path
   * @returns {Promise<void>}
   */
  function unlink(path) {
    return promisify(_unlink, path);
  }

  /**
   * Renames a file or directory.
   * @param {string | Buffer | URL} oldPath - old path
   * @param {string | Buffer | URL} newPath - new path
   * @returns {Promise<void>}
   */
  function rename(oldPath, newPath) {
    return promisify(_rename, oldPath, newPath);
  }

  /**
   * Copies a file.
   * @param {string | Buffer | URL} src - source path
   * @param {string | Buffer | URL} dest - destination path
   * @param {number} mode - copy mode (optional)
   * @returns {Promise<void>}
   */
  function copyFile(src, dest, mode) {
    return promisify(_copyFile, src, dest, mode);
  }

  /**
   * Gets file statistics.
   * @param {string | Buffer | URL} path - file path
   * @param {Object} options - options object
   * @returns {Promise<Stats>}
   */
  function stat(path, options) {
    return promisify(_stat, path, options);
  }

  /**
   * Tests file accessibility.
   * @param {string | Buffer | URL} path - file path
   * @param {number} mode - accessibility mode
   * @returns {Promise<void>}
   */
  function access(path, mode) {
    return promisify(_access, path, mode);
  }

  // Constants
  const constants = {
    F_OK: 0,  // File exists
    R_OK: 4,  // File is readable
    W_OK: 2,  // File is writable
    X_OK: 1,  // File is executable
  };

  // Export the fs/promises API
  const fsPromises = {
    readFile,
    readdir,
    writeFile,
    mkdir,
    rmdir,
    unlink,
    rename,
    copyFile,
    stat,
    access,
    constants,
  };

  // Make it available via import
  globalThis.__node_modules = globalThis.__node_modules || {};
  globalThis.__node_modules['node:fs/promises'] = fsPromises;
  globalThis.__node_modules['fs/promises'] = fsPromises;
});
