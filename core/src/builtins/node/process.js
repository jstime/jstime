// Node.js Process API
// Minimal implementation compatible with Node.js

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ getEnv, getArgv, getCwd, exit, writeStdout, writeStderr, readStdin }) => {
  // Create a writable stream for stdout/stderr
  class ProcessWritableStream {
    #writeFunc;
    #isTTY;

    constructor(writeFunc, isTTY = false) {
      this.#writeFunc = writeFunc;
      this.#isTTY = isTTY;
    }

    write(chunk, encoding, callback) {
      // Handle different argument combinations
      if (typeof encoding === 'function') {
        callback = encoding;
        encoding = 'utf8';
      }

      try {
        // Convert chunk to string if needed
        let str;
        if (typeof chunk === 'string') {
          str = chunk;
        } else if (chunk instanceof Uint8Array) {
          // For Uint8Array, pass directly to native function
          this.#writeFunc(chunk);
          if (callback) callback();
          return true;
        } else if (chunk && typeof chunk === 'object' && chunk.buffer instanceof ArrayBuffer) {
          // Handle other typed arrays
          const uint8 = new Uint8Array(chunk.buffer, chunk.byteOffset, chunk.byteLength);
          this.#writeFunc(uint8);
          if (callback) callback();
          return true;
        } else {
          str = String(chunk);
        }

        this.#writeFunc(str);
        if (callback) callback();
        return true;
      } catch (e) {
        if (callback) callback(e);
        return false;
      }
    }

    end(chunk, encoding, callback) {
      if (typeof chunk === 'function') {
        callback = chunk;
        chunk = undefined;
        encoding = 'utf8';
      } else if (typeof encoding === 'function') {
        callback = encoding;
        encoding = 'utf8';
      }

      if (chunk !== undefined) {
        this.write(chunk, encoding);
      }
      if (callback) callback();
    }

    get isTTY() {
      return this.#isTTY;
    }

    // Stream compatibility methods
    setEncoding() { return this; }
    pause() { return this; }
    resume() { return this; }
    destroy() { return this; }
  }

  // Create a readable stream for stdin
  class ProcessReadableStream {
    #isTTY;
    #hasRead;
    #data;

    constructor(isTTY = false) {
      this.#isTTY = isTTY;
      this.#hasRead = false;
      this.#data = null;
    }

    read(size) {
      // Read all data from stdin (blocking operation)
      if (!this.#hasRead) {
        this.#data = readStdin();
        this.#hasRead = true;
      }
      return this.#data;
    }

    setEncoding() { return this; }
    pause() { return this; }
    resume() { return this; }
    destroy() { return this; }

    get isTTY() {
      return this.#isTTY;
    }
  }

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

    // Standard streams
    get stdout() {
      if (!this._stdout) {
        this._stdout = new ProcessWritableStream(writeStdout, false);
      }
      return this._stdout;
    },

    get stderr() {
      if (!this._stderr) {
        this._stderr = new ProcessWritableStream(writeStderr, false);
      }
      return this._stderr;
    },

    get stdin() {
      if (!this._stdin) {
        this._stdin = new ProcessReadableStream(false);
      }
      return this._stdin;
    },
  };

  // Make process available globally
  globalThis.process = process;
});
