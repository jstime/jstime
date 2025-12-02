// Node.js Events API
// https://nodejs.org/api/events.html
// Wrapper around WHATWG EventTarget for Node.js compatibility

'use strict';

// eslint-disable-next-line no-unused-expressions
(({}) => {
  // Default maximum listeners (can be changed per-instance or globally)
  let defaultMaxListeners = 10;

  /**
   * EventEmitter class - Node.js compatible wrapper around EventTarget
   * Provides Node.js-style event API using the underlying WHATWG EventTarget
   */
  class EventEmitter extends EventTarget {
    constructor() {
      super();
      this._maxListeners = undefined;
      this._wrapperMap = new Map(); // Maps original listeners to wrapped listeners
    }

    /**
     * Get the default maximum number of listeners
     */
    static get defaultMaxListeners() {
      return defaultMaxListeners;
    }

    /**
     * Set the default maximum number of listeners
     */
    static set defaultMaxListeners(n) {
      if (typeof n !== 'number' || n < 0 || Number.isNaN(n)) {
        throw new RangeError('The value of "defaultMaxListeners" is out of range. It must be a non-negative number. Received ' + n);
      }
      defaultMaxListeners = n;
    }

    /**
     * Set the maximum number of listeners for this emitter
     */
    setMaxListeners(n) {
      if (typeof n !== 'number' || n < 0 || Number.isNaN(n)) {
        throw new RangeError('The value of "n" is out of range. It must be a non-negative number. Received ' + n);
      }
      this._maxListeners = n;
      return this;
    }

    /**
     * Get the maximum number of listeners for this emitter
     */
    getMaxListeners() {
      return this._maxListeners === undefined ? defaultMaxListeners : this._maxListeners;
    }

    /**
     * Synchronously calls each of the listeners registered for the event
     */
    emit(eventName, ...args) {
      const listeners = this.__listeners__.get(String(eventName));
      
      if (!listeners || listeners.length === 0) {
        // Special case: 'error' event with no listener throws
        if (eventName === 'error') {
          const err = args[0];
          if (err instanceof Error) {
            throw err;
          }
          const error = new Error('Unhandled error.' + (err ? ' (' + err + ')' : ''));
          error.context = err;
          throw error;
        }
        return false;
      }

      // Call listeners with args directly (Node.js style - no Event object)
      const listenersCopy = listeners.slice();
      for (let i = 0; i < listenersCopy.length; i++) {
        listenersCopy[i].apply(this, args);
      }
      return true;
    }

    /**
     * Adds a listener function to the end of the listeners array
     */
    addListener(eventName, listener) {
      return this._addListener(eventName, listener, false);
    }

    /**
     * Alias for addListener
     */
    on(eventName, listener) {
      return this.addListener(eventName, listener);
    }

    /**
     * Adds a one-time listener function for the event
     */
    once(eventName, listener) {
      if (typeof listener !== 'function') {
        throw new TypeError('The "listener" argument must be of type Function. Received type ' + typeof listener);
      }

      const wrapped = (...args) => {
        this.removeListener(eventName, wrapped);
        listener.apply(this, args);
      };
      wrapped.listener = listener;
      
      return this._addListener(eventName, wrapped, false);
    }

    /**
     * Adds a listener function to the beginning of the listeners array
     */
    prependListener(eventName, listener) {
      return this._addListener(eventName, listener, true);
    }

    /**
     * Adds a one-time listener to the beginning of the listeners array
     */
    prependOnceListener(eventName, listener) {
      if (typeof listener !== 'function') {
        throw new TypeError('The "listener" argument must be of type Function. Received type ' + typeof listener);
      }

      const wrapped = (...args) => {
        this.removeListener(eventName, wrapped);
        listener.apply(this, args);
      };
      wrapped.listener = listener;
      
      return this._addListener(eventName, wrapped, true);
    }

    /**
     * Internal method to add a listener using EventTarget's __listeners__ directly
     */
    _addListener(eventName, listener, prepend) {
      if (typeof listener !== 'function') {
        throw new TypeError('The "listener" argument must be of type Function. Received type ' + typeof listener);
      }

      const typeStr = String(eventName);

      // Emit 'newListener' event if there are listeners for it
      const newListenerListeners = this.__listeners__.get('newListener');
      if (newListenerListeners && newListenerListeners.length > 0) {
        this.emit('newListener', eventName, listener.listener || listener);
      }

      // Get or create array for this event type (using EventTarget's internal map)
      let listeners = this.__listeners__.get(typeStr);
      if (!listeners) {
        listeners = [];
        this.__listeners__.set(typeStr, listeners);
      }

      // Add listener at beginning or end
      if (prepend) {
        listeners.unshift(listener);
      } else {
        listeners.push(listener);
      }

      // Check for listener leak
      const maxListeners = this.getMaxListeners();
      if (maxListeners > 0 && listeners.length > maxListeners && !listeners._warned) {
        console.error(
          'MaxListenersExceededWarning: Possible EventEmitter memory leak detected. ' +
          listeners.length + ' ' + typeStr + ' listeners added. ' +
          'Use emitter.setMaxListeners() to increase limit'
        );
        listeners._warned = true;
      }

      return this;
    }

    /**
     * Removes the specified listener from the listener array
     */
    removeListener(eventName, listener) {
      if (typeof listener !== 'function') {
        throw new TypeError('The "listener" argument must be of type Function. Received type ' + typeof listener);
      }

      const typeStr = String(eventName);
      const listeners = this.__listeners__.get(typeStr);
      if (!listeners) {
        return this;
      }

      // Find and remove listener (check both direct match and .listener for once())
      for (let i = listeners.length - 1; i >= 0; i--) {
        if (listeners[i] === listener || listeners[i].listener === listener) {
          listeners.splice(i, 1);
          
          // Emit 'removeListener' event
          const removeListenerListeners = this.__listeners__.get('removeListener');
          if (removeListenerListeners && removeListenerListeners.length > 0) {
            this.emit('removeListener', eventName, listener);
          }
          break;
        }
      }

      return this;
    }

    /**
     * Alias for removeListener
     */
    off(eventName, listener) {
      return this.removeListener(eventName, listener);
    }

    /**
     * Removes all listeners, or those of the specified eventName
     */
    removeAllListeners(eventName) {
      if (arguments.length === 0) {
        // Remove all listeners for all events
        const eventNames = Array.from(this.__listeners__.keys());
        for (const name of eventNames) {
          if (name !== 'removeListener') {
            this.removeAllListeners(name);
          }
        }
        this.__listeners__.delete('removeListener');
        return this;
      }

      const typeStr = String(eventName);
      const listeners = this.__listeners__.get(typeStr);
      if (!listeners) {
        return this;
      }

      // Emit 'removeListener' for each listener
      const removeListenerListeners = this.__listeners__.get('removeListener');
      if (removeListenerListeners && removeListenerListeners.length > 0) {
        for (let i = listeners.length - 1; i >= 0; i--) {
          this.emit('removeListener', eventName, listeners[i].listener || listeners[i]);
        }
      }

      this.__listeners__.delete(typeStr);
      return this;
    }

    /**
     * Returns a copy of the array of listeners for the event
     */
    listeners(eventName) {
      const listeners = this.__listeners__.get(String(eventName));
      if (!listeners) {
        return [];
      }
      return listeners.map(listener => listener.listener || listener);
    }

    /**
     * Returns a copy of the array of listeners, including wrappers
     */
    rawListeners(eventName) {
      const listeners = this.__listeners__.get(String(eventName));
      if (!listeners) {
        return [];
      }
      return listeners.slice();
    }

    /**
     * Returns the number of listeners listening to the event
     */
    listenerCount(eventName) {
      const listeners = this.__listeners__.get(String(eventName));
      return listeners ? listeners.length : 0;
    }

    /**
     * Returns an array listing the events for which the emitter has listeners
     */
    eventNames() {
      return Array.from(this.__listeners__.keys());
    }
  }

  /**
   * Creates a Promise that is fulfilled when the emitter emits the given event
   */
  function once(emitter, eventName) {
    return new Promise((resolve, reject) => {
      const eventListener = (...args) => {
        if (errorListener) {
          emitter.removeListener('error', errorListener);
        }
        resolve(args);
      };
      let errorListener;

      if (eventName !== 'error') {
        errorListener = (err) => {
          emitter.removeListener(eventName, eventListener);
          reject(err);
        };
        emitter.once('error', errorListener);
      }

      emitter.once(eventName, eventListener);
    });
  }

  /**
   * Returns the number of listeners listening to eventName on emitter
   * @deprecated Use emitter.listenerCount() instead
   */
  function listenerCount(emitter, eventName) {
    return emitter.listenerCount(eventName);
  }

  /**
   * Returns a copy of the array of listeners for the event
   */
  function getEventListeners(emitter, eventName) {
    if (typeof emitter.listeners === 'function') {
      return emitter.listeners(eventName);
    }
    return [];
  }

  // Export the events module
  const events = {
    EventEmitter,
    once,
    listenerCount,
    getEventListeners,
    default: EventEmitter,
  };

  events.EventEmitter = EventEmitter;

  // Make it available via import
  globalThis.__node_modules = globalThis.__node_modules || {};
  globalThis.__node_modules['node:events'] = events;
  globalThis.__node_modules['events'] = events;
});
