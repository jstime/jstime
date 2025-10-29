((bindings) => {
  'use strict';

  // Headers class
  class Headers {
    constructor(init) {
      this._headers = new Map();
      if (init) {
        if (init instanceof Headers) {
          for (const [key, value] of init.entries()) {
            this.append(key, value);
          }
        } else if (Array.isArray(init)) {
          for (const [key, value] of init) {
            this.append(key, value);
          }
        } else if (typeof init === 'object') {
          for (const key in init) {
            if (init.hasOwnProperty(key)) {
              this.append(key, init[key]);
            }
          }
        }
      }
    }

    append(name, value) {
      const normalizedName = String(name).toLowerCase();
      const normalizedValue = String(value);
      
      if (this._headers.has(normalizedName)) {
        this._headers.set(normalizedName, this._headers.get(normalizedName) + ', ' + normalizedValue);
      } else {
        this._headers.set(normalizedName, normalizedValue);
      }
    }

    delete(name) {
      this._headers.delete(String(name).toLowerCase());
    }

    get(name) {
      return this._headers.get(String(name).toLowerCase()) || null;
    }

    has(name) {
      return this._headers.has(String(name).toLowerCase());
    }

    set(name, value) {
      this._headers.set(String(name).toLowerCase(), String(value));
    }

    entries() {
      return this._headers.entries();
    }

    keys() {
      return this._headers.keys();
    }

    values() {
      return this._headers.values();
    }

    forEach(callback, thisArg) {
      this._headers.forEach((value, key) => {
        callback.call(thisArg, value, key, this);
      });
    }

    [Symbol.iterator]() {
      return this._headers.entries();
    }

    _toArray() {
      const result = [];
      for (const [key, value] of this._headers) {
        result.push([key, value]);
      }
      return result;
    }
  }

  // Response class
  class Response {
    constructor(body, init = {}) {
      this._body = body;
      this._streamId = init.streamId;
      this.status = init.status || 200;
      this.statusText = init.statusText || '';
      this.headers = new Headers(init.headers || {});
      this.ok = this.status >= 200 && this.status < 300;
      this.redirected = false;
      this.type = 'basic';
      this.url = init.url || '';
      this._bodyUsed = false;
      this._bodyStream = null;
    }

    get body() {
      if (this._bodyStream) {
        return this._bodyStream;
      }
      
      // If we have a stream ID, create a streaming ReadableStream
      if (this._streamId !== undefined && this._streamId !== null) {
        const streamId = this._streamId;
        this._bodyStream = new globalThis.ReadableStream({
          async pull(controller) {
            try {
              const result = await bindings.fetchReadChunk(streamId);
              if (result.done) {
                controller.close();
              } else {
                controller.enqueue(result.value);
              }
            } catch (error) {
              controller.error(error);
            }
          }
        });
      } else {
        // Fallback: Create a ReadableStream from the body string
        this._bodyStream = new globalThis.ReadableStream({
          start: (controller) => {
            if (this._body !== null && this._body !== undefined) {
              const encoder = new TextEncoder();
              controller.enqueue(encoder.encode(String(this._body)));
            }
            controller.close();
          }
        });
      }
      
      return this._bodyStream;
    }

    get bodyUsed() {
      return this._bodyUsed;
    }

    async text() {
      if (this._bodyUsed) {
        return Promise.reject(new TypeError('Body has already been consumed'));
      }
      this._bodyUsed = true;
      
      // If we have a stream ID, read from the stream
      if (this._streamId !== undefined && this._streamId !== null) {
        const reader = this.body.getReader();
        const chunks = [];
        const decoder = new TextDecoder();
        
        while (true) {
          const { done, value } = await reader.read();
          if (done) break;
          chunks.push(decoder.decode(value, { stream: true }));
        }
        chunks.push(decoder.decode()); // Final decode without stream flag
        
        return chunks.join('');
      }
      
      // Fallback for non-streaming responses
      return String(this._body || '');
    }

    json() {
      return this.text().then(text => JSON.parse(text));
    }

    clone() {
      if (this._bodyUsed) {
        throw new TypeError('Body has already been consumed');
      }
      return new Response(this._body, {
        status: this.status,
        statusText: this.statusText,
        headers: this.headers,
        url: this.url,
        streamId: this._streamId
      });
    }
  }

  // Request class
  class Request {
    constructor(input, init = {}) {
      if (input instanceof Request) {
        this.url = input.url;
        this.method = init.method || input.method;
        this.headers = new Headers(init.headers || input.headers);
        this.body = init.body !== undefined ? init.body : input.body;
      } else {
        this.url = String(input);
        this.method = (init.method || 'GET').toUpperCase();
        this.headers = new Headers(init.headers || {});
        this.body = init.body !== undefined ? init.body : null;
      }

      this.mode = init.mode || 'cors';
      this.credentials = init.credentials || 'same-origin';
      this.cache = init.cache || 'default';
      this.redirect = init.redirect || 'follow';
      this.referrer = init.referrer || 'about:client';
      this.integrity = init.integrity || '';
    }

    clone() {
      return new Request(this.url, {
        method: this.method,
        headers: this.headers,
        body: this.body,
        mode: this.mode,
        credentials: this.credentials,
        cache: this.cache,
        redirect: this.redirect,
        referrer: this.referrer,
        integrity: this.integrity
      });
    }
  }

  // fetch function
  function fetch(resource, init = {}) {
    let request;
    
    if (resource instanceof Request) {
      request = resource;
      if (init && Object.keys(init).length > 0) {
        request = new Request(resource, init);
      }
    } else {
      request = new Request(resource, init);
    }

    // Prepare headers as an array for native binding
    const headersArray = request.headers._toArray();
    
    // Call native fetch binding and return the promise
    return bindings.fetchSend(
      request.url,
      request.method,
      headersArray,
      request.body || null
    ).then(responseData => {
      // Parse response - now includes streamId for streaming
      return new Response(null, {
        status: responseData.status,
        statusText: responseData.statusText,
        headers: responseData.headers,
        url: request.url,
        streamId: responseData.streamId
      });
    });
  }

  // Export to global
  globalThis.Headers = Headers;
  globalThis.Request = Request;
  globalThis.Response = Response;
  globalThis.fetch = fetch;
});
