// Node.js Buffer API
// https://nodejs.org/api/buffer.html
// Minimal implementation compatible with Node.js

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ bufferAlloc, bufferFrom, bufferConcat, bufferByteLength, bufferCompare, bufferIsEncoding }) => {
  // Constants for encoding
  const kMaxLength = 0xFFFFFFFF;

  // List of supported encodings
  const ENCODINGS = ['utf8', 'utf-8', 'hex', 'base64', 'base64url', 'latin1', 'binary', 'ascii'];

  /**
   * Buffer class - extends Uint8Array
   * Note: In Node.js, Buffer extends Uint8Array. We simulate this by adding
   * Buffer-specific methods to Uint8Array instances created by our native functions.
   */
  class Buffer extends Uint8Array {
    /**
     * Allocates a new Buffer of size bytes.
     * @param {number} size - The desired length of the new Buffer.
     * @param {number|string|Buffer} [fill] - A value to pre-fill the Buffer with.
     * @param {string} [encoding='utf8'] - If fill is a string, this is its encoding.
     * @returns {Buffer}
     */
    static alloc(size, fill, encoding) {
      if (typeof size !== 'number') {
        throw new TypeError('The "size" argument must be of type number');
      }
      if (size < 0 || size > kMaxLength) {
        throw new RangeError(`The value "${size}" is invalid for option "size"`);
      }
      const buf = bufferAlloc(size, fill, encoding);
      return Buffer._wrapUint8Array(buf);
    }

    /**
     * Allocates a new Buffer of size bytes (unsafe, not zeroed).
     * @param {number} size - The desired length of the new Buffer.
     * @returns {Buffer}
     */
    static allocUnsafe(size) {
      if (typeof size !== 'number') {
        throw new TypeError('The "size" argument must be of type number');
      }
      if (size < 0 || size > kMaxLength) {
        throw new RangeError(`The value "${size}" is invalid for option "size"`);
      }
      // In our implementation, allocUnsafe is the same as alloc since we always
      // create new backing stores
      const buf = bufferAlloc(size);
      return Buffer._wrapUint8Array(buf);
    }

    /**
     * Allocates a new Buffer of size bytes (unsafe, slow, not zeroed).
     * @param {number} size - The desired length of the new Buffer.
     * @returns {Buffer}
     */
    static allocUnsafeSlow(size) {
      return Buffer.allocUnsafe(size);
    }

    /**
     * Creates a new Buffer from various sources.
     * @param {string|Array|Buffer|ArrayBuffer|Object} value - Source value.
     * @param {string|number} [encodingOrOffset] - Encoding for strings, or offset for ArrayBuffer.
     * @param {number} [length] - Length for ArrayBuffer slice.
     * @returns {Buffer}
     */
    static from(value, encodingOrOffset, length) {
      if (value === null || value === undefined) {
        throw new TypeError('The first argument must be of type string or an instance of Buffer, ArrayBuffer, or Array');
      }

      // Handle string
      if (typeof value === 'string') {
        const buf = bufferFrom(value, encodingOrOffset || 'utf8');
        return Buffer._wrapUint8Array(buf);
      }

      // Handle ArrayBuffer with offset and length
      if (value instanceof ArrayBuffer) {
        const offset = typeof encodingOrOffset === 'number' ? encodingOrOffset : 0;
        const len = typeof length === 'number' ? length : value.byteLength - offset;
        const view = new Uint8Array(value, offset, len);
        const buf = bufferFrom(view);
        return Buffer._wrapUint8Array(buf);
      }

      // Handle array-like, Buffer, TypedArray
      const buf = bufferFrom(value);
      return Buffer._wrapUint8Array(buf);
    }

    /**
     * Returns the concatenation of the Buffer instances in list.
     * @param {Array<Buffer|Uint8Array>} list - List of Buffers to concatenate.
     * @param {number} [totalLength] - Total length of the resulting Buffer.
     * @returns {Buffer}
     */
    static concat(list, totalLength) {
      if (!Array.isArray(list)) {
        throw new TypeError('The "list" argument must be an instance of Array');
      }
      if (list.length === 0) {
        return Buffer.alloc(0);
      }
      const buf = bufferConcat(list, totalLength);
      return Buffer._wrapUint8Array(buf);
    }

    /**
     * Returns the byte length of a string when encoded.
     * @param {string|Buffer|ArrayBuffer} string - The value to get byte length of.
     * @param {string} [encoding='utf8'] - The encoding.
     * @returns {number}
     */
    static byteLength(string, encoding) {
      return bufferByteLength(string, encoding || 'utf8');
    }

    /**
     * Compares two Buffer instances.
     * @param {Buffer} buf1 - First Buffer.
     * @param {Buffer} buf2 - Second Buffer.
     * @returns {number} -1, 0, or 1.
     */
    static compare(buf1, buf2) {
      if (!(buf1 instanceof Uint8Array) || !(buf2 instanceof Uint8Array)) {
        throw new TypeError('The "buf1" and "buf2" arguments must be one of type Buffer or Uint8Array');
      }
      return bufferCompare(buf1, buf2);
    }

    /**
     * Returns true if encoding is a supported character encoding.
     * @param {string} encoding - The encoding to test.
     * @returns {boolean}
     */
    static isEncoding(encoding) {
      return bufferIsEncoding(encoding);
    }

    /**
     * Returns true if obj is a Buffer.
     * @param {*} obj - Object to test.
     * @returns {boolean}
     */
    static isBuffer(obj) {
      return obj instanceof Buffer || (obj instanceof Uint8Array && obj._isBuffer === true);
    }

    /**
     * Wraps a Uint8Array with Buffer methods.
     * @private
     */
    static _wrapUint8Array(arr) {
      // Mark as buffer
      Object.defineProperty(arr, '_isBuffer', { value: true, writable: false, enumerable: false });

      // Add Buffer prototype methods
      Object.setPrototypeOf(arr, Buffer.prototype);

      return arr;
    }

    /**
     * Returns the maximum number of bytes allowed for a Buffer.
     */
    static get poolSize() {
      return 8192;
    }

    /**
     * Writes string to buf at offset according to encoding.
     * @param {string} string - String to write.
     * @param {number} [offset=0] - Offset to start writing at.
     * @param {number} [length] - Maximum number of bytes to write.
     * @param {string} [encoding='utf8'] - The encoding.
     * @returns {number} Number of bytes written.
     */
    write(string, offset, length, encoding) {
      // Handle various argument combinations
      if (typeof offset === 'string') {
        encoding = offset;
        offset = 0;
        length = this.length;
      } else if (typeof length === 'string') {
        encoding = length;
        length = this.length - offset;
      }

      offset = offset || 0;
      length = length !== undefined ? length : this.length - offset;
      encoding = encoding || 'utf8';

      if (typeof string !== 'string') {
        throw new TypeError('Argument must be a string');
      }

      const encoded = Buffer.from(string, encoding);
      const bytesToWrite = Math.min(length, encoded.length, this.length - offset);

      for (let i = 0; i < bytesToWrite; i++) {
        this[offset + i] = encoded[i];
      }

      return bytesToWrite;
    }

    /**
     * Decodes buf to a string according to encoding.
     * @param {string} [encoding='utf8'] - The encoding.
     * @param {number} [start=0] - Byte offset to start decoding at.
     * @param {number} [end] - Byte offset to stop decoding at.
     * @returns {string}
     */
    toString(encoding, start, end) {
      encoding = encoding || 'utf8';
      start = start || 0;
      end = end !== undefined ? end : this.length;

      if (start >= end) {
        return '';
      }

      const slice = this.subarray(start, end);
      const enc = encoding.toLowerCase();

      switch (enc) {
        case 'utf8':
        case 'utf-8':
          return new TextDecoder('utf-8').decode(slice);

        case 'hex':
          let hex = '';
          for (let i = 0; i < slice.length; i++) {
            hex += slice[i].toString(16).padStart(2, '0');
          }
          return hex;

        case 'base64': {
          // Use btoa for base64 encoding
          let binary = '';
          for (let i = 0; i < slice.length; i++) {
            binary += String.fromCharCode(slice[i]);
          }
          return btoa(binary);
        }

        case 'base64url': {
          // URL-safe base64
          let binary = '';
          for (let i = 0; i < slice.length; i++) {
            binary += String.fromCharCode(slice[i]);
          }
          return btoa(binary).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
        }

        case 'latin1':
        case 'binary':
        case 'ascii': {
          let str = '';
          for (let i = 0; i < slice.length; i++) {
            str += String.fromCharCode(slice[i]);
          }
          return str;
        }

        default:
          return new TextDecoder('utf-8').decode(slice);
      }
    }

    /**
     * Returns a JSON representation of buf.
     * @returns {Object}
     */
    toJSON() {
      return {
        type: 'Buffer',
        data: Array.from(this)
      };
    }

    /**
     * Compares buf with target.
     * @param {Buffer|Uint8Array} target - Buffer to compare to.
     * @param {number} [targetStart=0] - Offset in target to start comparison.
     * @param {number} [targetEnd=target.length] - Offset in target to end comparison.
     * @param {number} [sourceStart=0] - Offset in buf to start comparison.
     * @param {number} [sourceEnd=buf.length] - Offset in buf to end comparison.
     * @returns {number} -1, 0, or 1.
     */
    compare(target, targetStart, targetEnd, sourceStart, sourceEnd) {
      if (!(target instanceof Uint8Array)) {
        throw new TypeError('The "target" argument must be one of type Buffer or Uint8Array');
      }

      targetStart = targetStart || 0;
      targetEnd = targetEnd !== undefined ? targetEnd : target.length;
      sourceStart = sourceStart || 0;
      sourceEnd = sourceEnd !== undefined ? sourceEnd : this.length;

      const source = this.subarray(sourceStart, sourceEnd);
      const dest = target.subarray(targetStart, targetEnd);

      return Buffer.compare(source, dest);
    }

    /**
     * Returns true if buf and otherBuffer have exactly the same bytes.
     * @param {Buffer|Uint8Array} otherBuffer - Buffer to compare to.
     * @returns {boolean}
     */
    equals(otherBuffer) {
      if (!(otherBuffer instanceof Uint8Array)) {
        throw new TypeError('The "otherBuffer" argument must be one of type Buffer or Uint8Array');
      }
      return this.compare(otherBuffer) === 0;
    }

    /**
     * Copies data from a region of buf to a region in target.
     * @param {Buffer|Uint8Array} target - Target Buffer.
     * @param {number} [targetStart=0] - Offset in target to start writing.
     * @param {number} [sourceStart=0] - Offset in buf to start copying from.
     * @param {number} [sourceEnd=buf.length] - Offset in buf to stop copying.
     * @returns {number} Number of bytes copied.
     */
    copy(target, targetStart, sourceStart, sourceEnd) {
      if (!(target instanceof Uint8Array)) {
        throw new TypeError('The "target" argument must be one of type Buffer or Uint8Array');
      }

      targetStart = targetStart || 0;
      sourceStart = sourceStart || 0;
      sourceEnd = sourceEnd !== undefined ? sourceEnd : this.length;

      const bytesToCopy = Math.min(sourceEnd - sourceStart, target.length - targetStart);

      for (let i = 0; i < bytesToCopy; i++) {
        target[targetStart + i] = this[sourceStart + i];
      }

      return bytesToCopy;
    }

    /**
     * Fills buf with the specified value.
     * @param {string|Buffer|Uint8Array|number} value - Value to fill with.
     * @param {number} [offset=0] - Offset to start filling.
     * @param {number} [end=buf.length] - Offset to stop filling.
     * @param {string} [encoding='utf8'] - The encoding.
     * @returns {Buffer} A reference to buf.
     */
    fill(value, offset, end, encoding) {
      if (typeof offset === 'string') {
        encoding = offset;
        offset = 0;
        end = this.length;
      } else if (typeof end === 'string') {
        encoding = end;
        end = this.length;
      }

      offset = offset || 0;
      end = end !== undefined ? end : this.length;
      encoding = encoding || 'utf8';

      let fillValue;
      if (typeof value === 'number') {
        fillValue = [value & 0xFF];
      } else if (typeof value === 'string') {
        const encoded = Buffer.from(value, encoding);
        fillValue = Array.from(encoded);
      } else if (value instanceof Uint8Array) {
        fillValue = Array.from(value);
      } else {
        fillValue = [0];
      }

      if (fillValue.length === 0) {
        fillValue = [0];
      }

      let fillIndex = 0;
      for (let i = offset; i < end; i++) {
        this[i] = fillValue[fillIndex % fillValue.length];
        fillIndex++;
      }

      return this;
    }

    /**
     * Returns the index of the first occurrence of value in buf.
     * @param {string|Buffer|Uint8Array|number} value - Value to search for.
     * @param {number} [byteOffset=0] - Offset to begin searching.
     * @param {string} [encoding='utf8'] - The encoding.
     * @returns {number} Index of first occurrence, or -1.
     */
    indexOf(value, byteOffset, encoding) {
      return this._indexOf(value, byteOffset, encoding, true);
    }

    /**
     * Returns the index of the last occurrence of value in buf.
     * @param {string|Buffer|Uint8Array|number} value - Value to search for.
     * @param {number} [byteOffset] - Offset to begin searching.
     * @param {string} [encoding='utf8'] - The encoding.
     * @returns {number} Index of last occurrence, or -1.
     */
    lastIndexOf(value, byteOffset, encoding) {
      return this._indexOf(value, byteOffset, encoding, false);
    }

    /**
     * Internal indexOf implementation.
     * @private
     */
    _indexOf(value, byteOffset, encoding, first) {
      if (typeof byteOffset === 'string') {
        encoding = byteOffset;
        byteOffset = first ? 0 : this.length;
      }

      byteOffset = byteOffset || (first ? 0 : this.length);
      encoding = encoding || 'utf8';

      let searchBytes;
      if (typeof value === 'number') {
        searchBytes = [value & 0xFF];
      } else if (typeof value === 'string') {
        const encoded = Buffer.from(value, encoding);
        searchBytes = Array.from(encoded);
      } else if (value instanceof Uint8Array) {
        searchBytes = Array.from(value);
      } else {
        return -1;
      }

      if (searchBytes.length === 0) {
        return first ? 0 : this.length;
      }

      if (first) {
        // Forward search
        for (let i = byteOffset; i <= this.length - searchBytes.length; i++) {
          let found = true;
          for (let j = 0; j < searchBytes.length; j++) {
            if (this[i + j] !== searchBytes[j]) {
              found = false;
              break;
            }
          }
          if (found) {
            return i;
          }
        }
      } else {
        // Backward search
        const startIndex = Math.min(byteOffset, this.length - searchBytes.length);
        for (let i = startIndex; i >= 0; i--) {
          let found = true;
          for (let j = 0; j < searchBytes.length; j++) {
            if (this[i + j] !== searchBytes[j]) {
              found = false;
              break;
            }
          }
          if (found) {
            return i;
          }
        }
      }

      return -1;
    }

    /**
     * Returns true if value is found in buf.
     * @param {string|Buffer|Uint8Array|number} value - Value to search for.
     * @param {number} [byteOffset=0] - Offset to begin searching.
     * @param {string} [encoding='utf8'] - The encoding.
     * @returns {boolean}
     */
    includes(value, byteOffset, encoding) {
      return this.indexOf(value, byteOffset, encoding) !== -1;
    }

    /**
     * Returns a new Buffer that references the same memory as the original.
     * @param {number} [start=0] - Start offset.
     * @param {number} [end=buf.length] - End offset.
     * @returns {Buffer}
     */
    slice(start, end) {
      const slice = super.subarray(start, end);
      return Buffer._wrapUint8Array(slice);
    }

    /**
     * Returns a new Buffer that references the same memory as the original.
     * @param {number} [start=0] - Start offset.
     * @param {number} [end=buf.length] - End offset.
     * @returns {Buffer}
     */
    subarray(start, end) {
      const sub = super.subarray(start, end);
      return Buffer._wrapUint8Array(sub);
    }

    // Read methods
    readUInt8(offset = 0) {
      return this[offset];
    }

    readUInt16LE(offset = 0) {
      return this[offset] | (this[offset + 1] << 8);
    }

    readUInt16BE(offset = 0) {
      return (this[offset] << 8) | this[offset + 1];
    }

    readUInt32LE(offset = 0) {
      return (this[offset] | (this[offset + 1] << 8) | (this[offset + 2] << 16)) + this[offset + 3] * 0x1000000;
    }

    readUInt32BE(offset = 0) {
      return this[offset] * 0x1000000 + ((this[offset + 1] << 16) | (this[offset + 2] << 8) | this[offset + 3]);
    }

    readInt8(offset = 0) {
      const val = this[offset];
      return val & 0x80 ? val - 0x100 : val;
    }

    readInt16LE(offset = 0) {
      const val = this[offset] | (this[offset + 1] << 8);
      return val & 0x8000 ? val - 0x10000 : val;
    }

    readInt16BE(offset = 0) {
      const val = (this[offset] << 8) | this[offset + 1];
      return val & 0x8000 ? val - 0x10000 : val;
    }

    readInt32LE(offset = 0) {
      return this[offset] | (this[offset + 1] << 8) | (this[offset + 2] << 16) | (this[offset + 3] << 24);
    }

    readInt32BE(offset = 0) {
      return (this[offset] << 24) | (this[offset + 1] << 16) | (this[offset + 2] << 8) | this[offset + 3];
    }

    readFloatLE(offset = 0) {
      const view = new DataView(this.buffer, this.byteOffset + offset, 4);
      return view.getFloat32(0, true);
    }

    readFloatBE(offset = 0) {
      const view = new DataView(this.buffer, this.byteOffset + offset, 4);
      return view.getFloat32(0, false);
    }

    readDoubleLE(offset = 0) {
      const view = new DataView(this.buffer, this.byteOffset + offset, 8);
      return view.getFloat64(0, true);
    }

    readDoubleBE(offset = 0) {
      const view = new DataView(this.buffer, this.byteOffset + offset, 8);
      return view.getFloat64(0, false);
    }

    // Write methods
    writeUInt8(value, offset = 0) {
      this[offset] = value & 0xFF;
      return offset + 1;
    }

    writeUInt16LE(value, offset = 0) {
      this[offset] = value & 0xFF;
      this[offset + 1] = (value >>> 8) & 0xFF;
      return offset + 2;
    }

    writeUInt16BE(value, offset = 0) {
      this[offset] = (value >>> 8) & 0xFF;
      this[offset + 1] = value & 0xFF;
      return offset + 2;
    }

    writeUInt32LE(value, offset = 0) {
      this[offset] = value & 0xFF;
      this[offset + 1] = (value >>> 8) & 0xFF;
      this[offset + 2] = (value >>> 16) & 0xFF;
      this[offset + 3] = (value >>> 24) & 0xFF;
      return offset + 4;
    }

    writeUInt32BE(value, offset = 0) {
      this[offset] = (value >>> 24) & 0xFF;
      this[offset + 1] = (value >>> 16) & 0xFF;
      this[offset + 2] = (value >>> 8) & 0xFF;
      this[offset + 3] = value & 0xFF;
      return offset + 4;
    }

    writeInt8(value, offset = 0) {
      if (value < 0) value = 0x100 + value;
      this[offset] = value & 0xFF;
      return offset + 1;
    }

    writeInt16LE(value, offset = 0) {
      if (value < 0) value = 0x10000 + value;
      this[offset] = value & 0xFF;
      this[offset + 1] = (value >>> 8) & 0xFF;
      return offset + 2;
    }

    writeInt16BE(value, offset = 0) {
      if (value < 0) value = 0x10000 + value;
      this[offset] = (value >>> 8) & 0xFF;
      this[offset + 1] = value & 0xFF;
      return offset + 2;
    }

    writeInt32LE(value, offset = 0) {
      this[offset] = value & 0xFF;
      this[offset + 1] = (value >>> 8) & 0xFF;
      this[offset + 2] = (value >>> 16) & 0xFF;
      this[offset + 3] = (value >>> 24) & 0xFF;
      return offset + 4;
    }

    writeInt32BE(value, offset = 0) {
      this[offset] = (value >>> 24) & 0xFF;
      this[offset + 1] = (value >>> 16) & 0xFF;
      this[offset + 2] = (value >>> 8) & 0xFF;
      this[offset + 3] = value & 0xFF;
      return offset + 4;
    }

    writeFloatLE(value, offset = 0) {
      const view = new DataView(this.buffer, this.byteOffset + offset, 4);
      view.setFloat32(0, value, true);
      return offset + 4;
    }

    writeFloatBE(value, offset = 0) {
      const view = new DataView(this.buffer, this.byteOffset + offset, 4);
      view.setFloat32(0, value, false);
      return offset + 4;
    }

    writeDoubleLE(value, offset = 0) {
      const view = new DataView(this.buffer, this.byteOffset + offset, 8);
      view.setFloat64(0, value, true);
      return offset + 8;
    }

    writeDoubleBE(value, offset = 0) {
      const view = new DataView(this.buffer, this.byteOffset + offset, 8);
      view.setFloat64(0, value, false);
      return offset + 8;
    }

    /**
     * Swaps byte order in-place (16-bit).
     * @returns {Buffer}
     */
    swap16() {
      if (this.length % 2 !== 0) {
        throw new RangeError('Buffer size must be a multiple of 16-bits');
      }
      for (let i = 0; i < this.length; i += 2) {
        const t = this[i];
        this[i] = this[i + 1];
        this[i + 1] = t;
      }
      return this;
    }

    /**
     * Swaps byte order in-place (32-bit).
     * @returns {Buffer}
     */
    swap32() {
      if (this.length % 4 !== 0) {
        throw new RangeError('Buffer size must be a multiple of 32-bits');
      }
      for (let i = 0; i < this.length; i += 4) {
        const t0 = this[i];
        const t1 = this[i + 1];
        this[i] = this[i + 3];
        this[i + 1] = this[i + 2];
        this[i + 2] = t1;
        this[i + 3] = t0;
      }
      return this;
    }

    /**
     * Swaps byte order in-place (64-bit).
     * @returns {Buffer}
     */
    swap64() {
      if (this.length % 8 !== 0) {
        throw new RangeError('Buffer size must be a multiple of 64-bits');
      }
      for (let i = 0; i < this.length; i += 8) {
        const t0 = this[i];
        const t1 = this[i + 1];
        const t2 = this[i + 2];
        const t3 = this[i + 3];
        this[i] = this[i + 7];
        this[i + 1] = this[i + 6];
        this[i + 2] = this[i + 5];
        this[i + 3] = this[i + 4];
        this[i + 4] = t3;
        this[i + 5] = t2;
        this[i + 6] = t1;
        this[i + 7] = t0;
      }
      return this;
    }
  }

  // Constants
  Object.defineProperty(Buffer, 'kMaxLength', {
    value: kMaxLength,
    writable: false,
    enumerable: false,
    configurable: false
  });

  // Export the buffer module
  const bufferModule = {
    Buffer,
    kMaxLength,
    constants: {
      MAX_LENGTH: kMaxLength,
      MAX_STRING_LENGTH: kMaxLength
    },
    // For compatibility
    INSPECT_MAX_BYTES: 50,
    // SlowBuffer is deprecated but included for compatibility
    SlowBuffer: Buffer
  };

  // Make it available via import
  globalThis.__node_modules = globalThis.__node_modules || {};
  globalThis.__node_modules['node:buffer'] = bufferModule;
  globalThis.__node_modules['buffer'] = bufferModule;

  // Also expose Buffer globally (like Node.js)
  globalThis.Buffer = Buffer;
});
