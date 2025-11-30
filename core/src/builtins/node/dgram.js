// Node.js dgram API
// Minimal implementation compatible with Node.js
// https://nodejs.org/api/dgram.html

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ dgramCreateSocket, dgramBind, dgramSend, dgramClose, dgramAddress, dgramSetBroadcast, dgramSetTTL, dgramSetMulticastTTL, dgramSetMulticastLoopback, dgramAddMembership, dgramDropMembership, dgramRecv }) => {

  /**
   * Socket class - represents a UDP socket
   * Extends EventTarget to provide event-based API
   */
  class Socket extends EventTarget {
    #type;
    #socket = null;
    #bound = false;
    #closed = false;
    #recvBufferSize = 65536;
    #sendBufferSize = 65536;

    constructor(options) {
      super();

      if (typeof options === 'string') {
        this.#type = options;
      } else if (options && typeof options === 'object') {
        this.#type = options.type || 'udp4';
        if (options.recvBufferSize) {
          this.#recvBufferSize = options.recvBufferSize;
        }
        if (options.sendBufferSize) {
          this.#sendBufferSize = options.sendBufferSize;
        }
      } else {
        throw new TypeError('Invalid options');
      }

      if (this.#type !== 'udp4' && this.#type !== 'udp6') {
        throw new TypeError("Bad socket type specified. Valid types are: udp4, udp6");
      }

      // Create the underlying socket type identifier
      dgramCreateSocket(this.#type);
    }

    /**
     * Binds the socket to a port and optional address
     * @param {number} port - Port to bind to (0 for OS-assigned)
     * @param {string} [address] - Address to bind to
     * @param {Function} [callback] - Called when binding is complete
     * @returns {Socket} this
     */
    bind(port, address, callback) {
      if (this.#closed) {
        throw new Error('Socket is closed');
      }

      if (this.#bound) {
        throw new Error('Socket is already bound');
      }

      // Handle different argument patterns
      if (typeof port === 'object' && port !== null) {
        // bind({ port, address, exclusive }, callback)
        const options = port;
        callback = address;
        port = options.port || 0;
        address = options.address;
      } else if (typeof port === 'function') {
        // bind(callback)
        callback = port;
        port = 0;
        address = undefined;
      } else if (typeof address === 'function') {
        // bind(port, callback)
        callback = address;
        address = undefined;
      }

      try {
        this.#socket = dgramBind(this.#type, port || 0, address);
        this.#bound = true;

        // Emit listening event asynchronously
        queueMicrotask(() => {
          const event = new Event('listening');
          this.dispatchEvent(event);
          if (callback) {
            callback();
          }
        });
      } catch (err) {
        queueMicrotask(() => {
          const errorEvent = new Event('error');
          errorEvent.error = err;
          this.dispatchEvent(errorEvent);
          if (callback) {
            callback(err);
          }
        });
      }

      return this;
    }

    /**
     * Sends a message through the socket
     * @param {Buffer|string|Array} msg - Message to send
     * @param {number} [offset] - Offset in buffer
     * @param {number} [length] - Length of message
     * @param {number} port - Destination port
     * @param {string} [address] - Destination address
     * @param {Function} [callback] - Called when message is sent
     */
    send(msg, offset, length, port, address, callback) {
      if (this.#closed) {
        throw new Error('Socket is closed');
      }

      // Handle different argument patterns
      // send(msg, port, address, callback)
      if (typeof offset === 'number' && typeof length === 'string') {
        callback = port;
        address = length;
        port = offset;
        offset = 0;
        length = msg.length;
      }
      // send(msg, port, callback)
      else if (typeof offset === 'number' && typeof length === 'function') {
        callback = length;
        address = undefined;
        port = offset;
        offset = 0;
        length = msg.length;
      }
      // send(msg, port, address)
      else if (typeof offset === 'number' && typeof length === 'string' && typeof port === 'undefined') {
        address = length;
        port = offset;
        offset = 0;
        length = msg.length;
      }

      // Default address based on socket type
      if (!address) {
        address = this.#type === 'udp6' ? '::1' : '127.0.0.1';
      }

      // Convert message to appropriate format
      let buffer;
      if (typeof msg === 'string') {
        buffer = new TextEncoder().encode(msg);
      } else if (msg instanceof Uint8Array) {
        buffer = msg;
      } else if (Array.isArray(msg)) {
        // Concatenate array of buffers
        const totalLength = msg.reduce((acc, buf) => acc + buf.length, 0);
        buffer = new Uint8Array(totalLength);
        let pos = 0;
        for (const buf of msg) {
          const data = typeof buf === 'string' ? new TextEncoder().encode(buf) : buf;
          buffer.set(data, pos);
          pos += data.length;
        }
        offset = 0;
        length = buffer.length;
      } else {
        throw new TypeError('Invalid message type');
      }

      // Auto-bind if not bound
      if (!this.#bound) {
        this.#socket = dgramBind(this.#type, 0, undefined);
        this.#bound = true;
      }

      try {
        const bytesSent = dgramSend(this.#socket, buffer, offset || 0, length || buffer.length, port, address);
        if (callback) {
          queueMicrotask(() => callback(null, bytesSent));
        }
      } catch (err) {
        if (callback) {
          queueMicrotask(() => callback(err));
        } else {
          const errorEvent = new Event('error');
          errorEvent.error = err;
          this.dispatchEvent(errorEvent);
        }
      }
    }

    /**
     * Closes the socket
     * @param {Function} [callback] - Called when socket is closed
     */
    close(callback) {
      if (this.#closed) {
        if (callback) {
          queueMicrotask(() => callback());
        }
        return;
      }

      this.#closed = true;

      if (this.#socket) {
        try {
          dgramClose(this.#socket);
        } catch (err) {
          // Ignore close errors
        }
        this.#socket = null;
      }

      queueMicrotask(() => {
        const event = new Event('close');
        this.dispatchEvent(event);
        if (callback) {
          callback();
        }
      });
    }

    /**
     * Returns the address information of the socket
     * @returns {{ address: string, family: string, port: number }}
     */
    address() {
      if (!this.#bound) {
        throw new Error('Socket is not bound');
      }
      if (this.#closed) {
        throw new Error('Socket is closed');
      }
      return dgramAddress(this.#socket);
    }

    /**
     * Sets or clears the SO_BROADCAST socket option
     * @param {boolean} flag
     */
    setBroadcast(flag) {
      if (!this.#bound) {
        throw new Error('Socket is not bound');
      }
      if (this.#closed) {
        throw new Error('Socket is closed');
      }
      dgramSetBroadcast(this.#socket, flag);
    }

    /**
     * Sets the IP_TTL socket option
     * @param {number} ttl
     */
    setTTL(ttl) {
      if (!this.#bound) {
        throw new Error('Socket is not bound');
      }
      if (this.#closed) {
        throw new Error('Socket is closed');
      }
      if (typeof ttl !== 'number' || ttl < 1 || ttl > 255) {
        throw new RangeError('TTL must be between 1 and 255');
      }
      dgramSetTTL(this.#socket, ttl);
    }

    /**
     * Sets the IP_MULTICAST_TTL socket option
     * @param {number} ttl
     */
    setMulticastTTL(ttl) {
      if (!this.#bound) {
        throw new Error('Socket is not bound');
      }
      if (this.#closed) {
        throw new Error('Socket is closed');
      }
      if (typeof ttl !== 'number' || ttl < 0 || ttl > 255) {
        throw new RangeError('Multicast TTL must be between 0 and 255');
      }
      dgramSetMulticastTTL(this.#socket, ttl);
    }

    /**
     * Sets or clears the IP_MULTICAST_LOOP socket option
     * @param {boolean} flag
     */
    setMulticastLoopback(flag) {
      if (!this.#bound) {
        throw new Error('Socket is not bound');
      }
      if (this.#closed) {
        throw new Error('Socket is closed');
      }
      dgramSetMulticastLoopback(this.#socket, flag);
    }

    /**
     * Joins a multicast group
     * @param {string} multicastAddress - Multicast group address
     * @param {string} [multicastInterface] - Interface to use
     */
    addMembership(multicastAddress, multicastInterface) {
      if (!this.#bound) {
        throw new Error('Socket is not bound');
      }
      if (this.#closed) {
        throw new Error('Socket is closed');
      }
      dgramAddMembership(this.#socket, multicastAddress, multicastInterface);
    }

    /**
     * Leaves a multicast group
     * @param {string} multicastAddress - Multicast group address
     * @param {string} [multicastInterface] - Interface to use
     */
    dropMembership(multicastAddress, multicastInterface) {
      if (!this.#bound) {
        throw new Error('Socket is not bound');
      }
      if (this.#closed) {
        throw new Error('Socket is closed');
      }
      dgramDropMembership(this.#socket, multicastAddress, multicastInterface);
    }

    /**
     * Returns the receive buffer size
     * @returns {number}
     */
    getRecvBufferSize() {
      return this.#recvBufferSize;
    }

    /**
     * Sets the receive buffer size
     * @param {number} size
     */
    setRecvBufferSize(size) {
      if (typeof size !== 'number' || size < 1) {
        throw new RangeError('Buffer size must be a positive number');
      }
      this.#recvBufferSize = size;
    }

    /**
     * Returns the send buffer size
     * @returns {number}
     */
    getSendBufferSize() {
      return this.#sendBufferSize;
    }

    /**
     * Sets the send buffer size
     * @param {number} size
     */
    setSendBufferSize(size) {
      if (typeof size !== 'number' || size < 1) {
        throw new RangeError('Buffer size must be a positive number');
      }
      this.#sendBufferSize = size;
    }

    /**
     * Reference the socket (keeps event loop alive)
     * @returns {Socket} this
     */
    ref() {
      // In our implementation, sockets don't keep the event loop alive by default
      // This is a no-op for compatibility
      return this;
    }

    /**
     * Unreference the socket (allows event loop to exit)
     * @returns {Socket} this
     */
    unref() {
      // In our implementation, sockets don't keep the event loop alive by default
      // This is a no-op for compatibility
      return this;
    }

    /**
     * Receive data from the socket (non-blocking)
     * This is used internally for polling
     * @returns {{ data: Uint8Array, rinfo: { address, family, port, size } } | null}
     */
    _recv() {
      if (!this.#bound || this.#closed || !this.#socket) {
        return null;
      }
      return dgramRecv(this.#socket);
    }

    // Node.js style event methods for compatibility
    on(event, listener) {
      this.addEventListener(event, (e) => {
        if (event === 'message') {
          listener(e.data, e.rinfo);
        } else if (event === 'error') {
          listener(e.error);
        } else {
          listener(e);
        }
      });
      return this;
    }

    once(event, listener) {
      this.addEventListener(event, (e) => {
        if (event === 'message') {
          listener(e.data, e.rinfo);
        } else if (event === 'error') {
          listener(e.error);
        } else {
          listener(e);
        }
      }, { once: true });
      return this;
    }

    off(event, listener) {
      this.removeEventListener(event, listener);
      return this;
    }

    removeListener(event, listener) {
      return this.off(event, listener);
    }

    removeAllListeners(event) {
      // Note: This is a simplified implementation
      // Full implementation would track all listeners
      return this;
    }
  }

  /**
   * Creates a dgram.Socket
   * @param {string|Object} options - Socket type ('udp4' or 'udp6') or options object
   * @param {Function} [callback] - Listener for 'message' event
   * @returns {Socket}
   */
  function createSocket(options, callback) {
    const socket = new Socket(options);
    if (callback) {
      socket.on('message', callback);
    }
    return socket;
  }

  // Export the dgram module
  const dgram = {
    createSocket,
    Socket,
  };

  // Make it available via import
  globalThis.__node_modules = globalThis.__node_modules || {};
  globalThis.__node_modules['node:dgram'] = dgram;
  globalThis.__node_modules['dgram'] = dgram;
});
