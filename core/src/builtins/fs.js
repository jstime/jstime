// Node.js fs/promises API
// Minimal implementation compatible with Node.js

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ readFile: _readFile, readDir: _readDir, writeFile: _writeFile, appendFile: _appendFile, mkdir: _mkdir, rmdir: _rmdir, unlink: _unlink, rename: _rename, copyFile: _copyFile, stat: _stat, access: _access, rm: _rm, truncate: _truncate, realpath: _realpath, chmod: _chmod, mkdtemp: _mkdtemp, readlink: _readlink, symlink: _symlink, lstat: _lstat, chown: _chown, utimes: _utimes }) => {
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
   * Appends data to a file, creating the file if it doesn't exist.
   * @param {string | Buffer | URL} path - filename
   * @param {string | Buffer} data - data to append
   * @param {Object | string} options - encoding or options object
   * @returns {Promise<void>}
   */
  function appendFile(path, data, options) {
    return promisify(_appendFile, path, data);
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

  /**
   * Removes files and directories (modern alternative to unlink/rmdir).
   * @param {string | Buffer | URL} path - path to remove
   * @param {Object} options - options object
   * @returns {Promise<void>}
   */
  function rm(path, options) {
    return promisify(_rm, path, options);
  }

  /**
   * Truncates a file to a specified length.
   * @param {string | Buffer | URL} path - file path
   * @param {number} len - target length (defaults to 0)
   * @returns {Promise<void>}
   */
  function truncate(path, len) {
    return promisify(_truncate, path, len);
  }

  /**
   * Resolves path to an absolute path.
   * @param {string | Buffer | URL} path - path to resolve
   * @param {Object} options - options object
   * @returns {Promise<string>}
   */
  function realpath(path, options) {
    return promisify(_realpath, path, options);
  }

  /**
   * Changes file permissions (Unix-like systems only).
   * @param {string | Buffer | URL} path - file path
   * @param {number} mode - file mode (permissions)
   * @returns {Promise<void>}
   */
  function chmod(path, mode) {
    return promisify(_chmod, path, mode);
  }

  /**
   * Creates a unique temporary directory.
   * @param {string} prefix - directory name prefix
   * @param {Object | string} options - encoding or options object
   * @returns {Promise<string>}
   */
  function mkdtemp(prefix, options) {
    return promisify(_mkdtemp, prefix, options);
  }

  /**
   * Reads the target of a symbolic link.
   * @param {string | Buffer | URL} path - path to symlink
   * @param {Object | string} options - encoding or options object
   * @returns {Promise<string>}
   */
  function readlink(path, options) {
    return promisify(_readlink, path, options);
  }

  /**
   * Creates a symbolic link.
   * @param {string | Buffer | URL} target - target path
   * @param {string | Buffer | URL} path - symlink path
   * @param {string} type - type of symlink (optional, for Windows)
   * @returns {Promise<void>}
   */
  function symlink(target, path, type) {
    return promisify(_symlink, target, path, type);
  }

  /**
   * Gets file statistics without following symlinks.
   * @param {string | Buffer | URL} path - file path
   * @param {Object} options - options object
   * @returns {Promise<Stats>}
   */
  function lstat(path, options) {
    return promisify(_lstat, path, options);
  }

  /**
   * Changes file ownership (Unix-like systems only).
   * @param {string | Buffer | URL} path - file path
   * @param {number} uid - user ID
   * @param {number} gid - group ID
   * @returns {Promise<void>}
   */
  function chown(path, uid, gid) {
    return promisify(_chown, path, uid, gid);
  }

  /**
   * Changes file access and modification times.
   * @param {string | Buffer | URL} path - file path
   * @param {number | Date} atime - access time
   * @param {number | Date} mtime - modification time
   * @returns {Promise<void>}
   */
  function utimes(path, atime, mtime) {
    return promisify(_utimes, path, atime, mtime);
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
    appendFile,
    mkdir,
    rmdir,
    unlink,
    rename,
    copyFile,
    stat,
    access,
    rm,
    truncate,
    realpath,
    chmod,
    mkdtemp,
    readlink,
    symlink,
    lstat,
    chown,
    utimes,
    constants,
  };

  // Make it available via import
  globalThis.__node_modules = globalThis.__node_modules || {};
  globalThis.__node_modules['node:fs/promises'] = fsPromises;
  globalThis.__node_modules['fs/promises'] = fsPromises;
});
