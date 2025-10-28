// Streams API
// https://streams.spec.whatwg.org/

'use strict';

// eslint-disable-next-line no-unused-expressions
(() => {
  // ReadableStream implementation
  class ReadableStream {
    #reader = null;
    #state = 'readable'; // 'readable', 'closed', 'errored'
    #queue = [];
    #error = null;
    #controller = null;

    constructor(underlyingSource = {}, strategy = {}) {
      const controller = {
        enqueue: (chunk) => {
          if (this.#state === 'closed') {
            throw new TypeError('Cannot enqueue on a closed stream');
          }
          if (this.#state === 'errored') {
            throw new TypeError('Cannot enqueue on an errored stream');
          }
          this.#queue.push(chunk);
        },
        close: () => {
          if (this.#state === 'closed') {
            throw new TypeError('Stream is already closed');
          }
          if (this.#state === 'errored') {
            throw new TypeError('Cannot close an errored stream');
          }
          this.#state = 'closed';
        },
        error: (e) => {
          if (this.#state === 'closed' || this.#state === 'errored') {
            return;
          }
          this.#error = e;
          this.#state = 'errored';
          this.#queue = [];
        }
      };

      this.#controller = controller;

      // Call start if provided
      if (typeof underlyingSource.start === 'function') {
        try {
          const startResult = underlyingSource.start.call(underlyingSource, controller);
          if (startResult && typeof startResult.then === 'function') {
            startResult.catch(e => controller.error(e));
          }
        } catch (e) {
          controller.error(e);
        }
      }
    }

    get locked() {
      return this.#reader !== null;
    }

    getReader() {
      if (this.#reader !== null) {
        throw new TypeError('ReadableStream is already locked to a reader');
      }

      const reader = new ReadableStreamDefaultReader(this);
      this.#reader = reader;
      return reader;
    }

    cancel(reason) {
      if (this.#reader !== null) {
        return Promise.reject(new TypeError('Cannot cancel a locked stream'));
      }
      this.#state = 'closed';
      this.#queue = [];
      return Promise.resolve();
    }

    // Internal method for reader to get chunks
    _pullChunk() {
      if (this.#queue.length > 0) {
        return { value: this.#queue.shift(), done: false };
      }
      if (this.#state === 'closed') {
        return { value: undefined, done: true };
      }
      if (this.#state === 'errored') {
        throw this.#error;
      }
      return null; // No chunks available yet
    }

    // Internal method to release reader
    _releaseReader() {
      this.#reader = null;
    }

    _getState() {
      return this.#state;
    }

    _getError() {
      return this.#error;
    }
  }

  // ReadableStreamDefaultReader implementation
  class ReadableStreamDefaultReader {
    #stream;
    #closed = false;

    constructor(stream) {
      if (!(stream instanceof ReadableStream)) {
        throw new TypeError('ReadableStreamDefaultReader can only be constructed with a ReadableStream');
      }
      this.#stream = stream;
    }

    read() {
      if (this.#closed) {
        return Promise.reject(new TypeError('Reader is closed'));
      }

      const chunk = this.#stream._pullChunk();
      if (chunk !== null) {
        return Promise.resolve(chunk);
      }

      // Stream state check
      const state = this.#stream._getState();
      if (state === 'closed') {
        return Promise.resolve({ value: undefined, done: true });
      }
      if (state === 'errored') {
        return Promise.reject(this.#stream._getError());
      }

      // No chunks available, return a pending promise
      // In a full implementation, this would wait for chunks
      return Promise.resolve({ value: undefined, done: true });
    }

    releaseLock() {
      if (this.#closed) {
        return;
      }
      this.#stream._releaseReader();
      this.#closed = true;
    }

    get closed() {
      if (this.#closed || this.#stream._getState() === 'closed') {
        return Promise.resolve();
      }
      if (this.#stream._getState() === 'errored') {
        return Promise.reject(this.#stream._getError());
      }
      // In a full implementation, this would be a promise that resolves when closed
      return new Promise(() => {});
    }

    cancel(reason) {
      if (this.#closed) {
        return Promise.resolve();
      }
      this.#closed = true;
      return this.#stream.cancel(reason);
    }
  }

  // WritableStream implementation
  class WritableStream {
    #writer = null;
    #state = 'writable'; // 'writable', 'closed', 'errored'
    #error = null;
    #underlyingSink = null;

    constructor(underlyingSink = {}, strategy = {}) {
      this.#underlyingSink = underlyingSink;

      const controller = {
        error: (e) => {
          if (this.#state === 'closed' || this.#state === 'errored') {
            return;
          }
          this.#error = e;
          this.#state = 'errored';
        }
      };

      // Call start if provided
      if (typeof underlyingSink.start === 'function') {
        try {
          const startResult = underlyingSink.start.call(underlyingSink, controller);
          if (startResult && typeof startResult.then === 'function') {
            startResult.catch(e => controller.error(e));
          }
        } catch (e) {
          controller.error(e);
        }
      }
    }

    get locked() {
      return this.#writer !== null;
    }

    getWriter() {
      if (this.#writer !== null) {
        throw new TypeError('WritableStream is already locked to a writer');
      }

      const writer = new WritableStreamDefaultWriter(this);
      this.#writer = writer;
      return writer;
    }

    abort(reason) {
      if (this.#writer !== null) {
        return Promise.reject(new TypeError('Cannot abort a locked stream'));
      }
      this.#state = 'errored';
      this.#error = reason;
      return Promise.resolve();
    }

    // Internal methods
    _write(chunk) {
      if (this.#state === 'errored') {
        return Promise.reject(this.#error);
      }
      if (this.#state === 'closed') {
        return Promise.reject(new TypeError('Stream is closed'));
      }

      if (typeof this.#underlyingSink.write === 'function') {
        try {
          const result = this.#underlyingSink.write.call(this.#underlyingSink, chunk);
          if (result && typeof result.then === 'function') {
            return result;
          }
          return Promise.resolve();
        } catch (e) {
          this.#state = 'errored';
          this.#error = e;
          return Promise.reject(e);
        }
      }
      return Promise.resolve();
    }

    _close() {
      if (this.#state === 'closed' || this.#state === 'errored') {
        return Promise.reject(new TypeError('Stream is already closed or errored'));
      }

      this.#state = 'closed';

      if (typeof this.#underlyingSink.close === 'function') {
        try {
          const result = this.#underlyingSink.close.call(this.#underlyingSink);
          if (result && typeof result.then === 'function') {
            return result;
          }
          return Promise.resolve();
        } catch (e) {
          return Promise.reject(e);
        }
      }
      return Promise.resolve();
    }

    _releaseWriter() {
      this.#writer = null;
    }

    _getState() {
      return this.#state;
    }

    _getError() {
      return this.#error;
    }
  }

  // WritableStreamDefaultWriter implementation
  class WritableStreamDefaultWriter {
    #stream;
    #closed = false;

    constructor(stream) {
      if (!(stream instanceof WritableStream)) {
        throw new TypeError('WritableStreamDefaultWriter can only be constructed with a WritableStream');
      }
      this.#stream = stream;
    }

    write(chunk) {
      if (this.#closed) {
        return Promise.reject(new TypeError('Writer is closed'));
      }
      return this.#stream._write(chunk);
    }

    close() {
      if (this.#closed) {
        return Promise.reject(new TypeError('Writer is already closed'));
      }
      this.#closed = true;
      return this.#stream._close();
    }

    releaseLock() {
      if (this.#closed) {
        return;
      }
      this.#stream._releaseWriter();
      this.#closed = true;
    }

    get closed() {
      if (this.#closed || this.#stream._getState() === 'closed') {
        return Promise.resolve();
      }
      if (this.#stream._getState() === 'errored') {
        return Promise.reject(this.#stream._getError());
      }
      // In a full implementation, this would be a promise that resolves when closed
      return new Promise(() => {});
    }

    get ready() {
      return Promise.resolve();
    }

    abort(reason) {
      if (this.#closed) {
        return Promise.resolve();
      }
      this.#closed = true;
      return this.#stream.abort(reason);
    }

    get desiredSize() {
      return 1;
    }
  }

  // TransformStream implementation
  class TransformStream {
    #readable;
    #writable;
    #readableController;

    constructor(transformer = {}, writableStrategy = {}, readableStrategy = {}) {
      const readableController = {
        enqueue: (chunk) => {
          if (this.#readableController) {
            this.#readableController.enqueue(chunk);
          }
        },
        close: () => {
          if (this.#readableController) {
            this.#readableController.close();
          }
        },
        error: (e) => {
          if (this.#readableController) {
            this.#readableController.error(e);
          }
        }
      };

      this.#readable = new ReadableStream({
        start: (controller) => {
          this.#readableController = controller;
        }
      });

      this.#writable = new WritableStream({
        write: async (chunk) => {
          if (typeof transformer.transform === 'function') {
            try {
              await transformer.transform.call(transformer, chunk, readableController);
            } catch (e) {
              readableController.error(e);
              throw e;
            }
          } else {
            readableController.enqueue(chunk);
          }
        },
        close: () => {
          if (typeof transformer.flush === 'function') {
            try {
              const result = transformer.flush.call(transformer, readableController);
              if (result && typeof result.then === 'function') {
                return result.then(() => readableController.close());
              }
            } catch (e) {
              readableController.error(e);
              throw e;
            }
          }
          readableController.close();
        },
        abort: (reason) => {
          readableController.error(reason);
        }
      });
    }

    get readable() {
      return this.#readable;
    }

    get writable() {
      return this.#writable;
    }
  }

  // Make classes available globally
  Object.defineProperty(globalThis, 'ReadableStream', {
    value: ReadableStream,
    writable: true,
    enumerable: false,
    configurable: true,
  });

  Object.defineProperty(globalThis, 'ReadableStreamDefaultReader', {
    value: ReadableStreamDefaultReader,
    writable: true,
    enumerable: false,
    configurable: true,
  });

  Object.defineProperty(globalThis, 'WritableStream', {
    value: WritableStream,
    writable: true,
    enumerable: false,
    configurable: true,
  });

  Object.defineProperty(globalThis, 'WritableStreamDefaultWriter', {
    value: WritableStreamDefaultWriter,
    writable: true,
    enumerable: false,
    configurable: true,
  });

  Object.defineProperty(globalThis, 'TransformStream', {
    value: TransformStream,
    writable: true,
    enumerable: false,
    configurable: true,
  });
});
