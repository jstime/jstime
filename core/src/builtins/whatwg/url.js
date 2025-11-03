// High-performance URL implementation with Rust-side storage
// Property access is lazy - only calls Rust when property is accessed
(function(bindings) {
  'use strict';
  
  const { urlParse, urlGetProperty, urlSetProperty } = bindings;
  
  class URL {
    #id;
    #searchParams;
    
    constructor(url, base) {
      const urlStr = String(url);
      const baseStr = base !== undefined ? String(base) : undefined;
      
      const id = urlParse(urlStr, baseStr);
      if (id === null) {
        throw new TypeError('Invalid URL');
      }
      
      this.#id = id;
      this.#searchParams = null;
    }
    
    get href() {
      return urlGetProperty(this.#id, 'href');
    }
    
    set href(value) {
      const newId = urlParse(String(value));
      if (newId !== null) {
        this.#id = newId;
        this.#searchParams = null; // Invalidate cached searchParams
      }
    }
    
    get origin() {
      return urlGetProperty(this.#id, 'origin');
    }
    
    get protocol() {
      return urlGetProperty(this.#id, 'protocol');
    }
    
    set protocol(value) {
      urlSetProperty(this.#id, 'protocol', String(value));
    }
    
    get username() {
      return urlGetProperty(this.#id, 'username');
    }
    
    get password() {
      return urlGetProperty(this.#id, 'password');
    }
    
    get host() {
      return urlGetProperty(this.#id, 'host');
    }
    
    get hostname() {
      return urlGetProperty(this.#id, 'hostname');
    }
    
    get port() {
      return urlGetProperty(this.#id, 'port');
    }
    
    get pathname() {
      return urlGetProperty(this.#id, 'pathname');
    }
    
    set pathname(value) {
      urlSetProperty(this.#id, 'pathname', String(value));
    }
    
    get search() {
      return urlGetProperty(this.#id, 'search');
    }
    
    set search(value) {
      urlSetProperty(this.#id, 'search', String(value));
      this.#searchParams = null; // Invalidate cached searchParams
    }
    
    get hash() {
      return urlGetProperty(this.#id, 'hash');
    }
    
    set hash(value) {
      urlSetProperty(this.#id, 'hash', String(value));
    }
    
    get searchParams() {
      if (!this.#searchParams) {
        this.#searchParams = new URLSearchParams(this.search, this);
      }
      return this.#searchParams;
    }
    
    // Internal method called by URLSearchParams to update search
    _updateSearch(newSearch) {
      urlSetProperty(this.#id, 'search', newSearch);
    }
    
    toString() {
      return this.href;
    }
    
    toJSON() {
      return this.href;
    }
  }
  
  // Simple URLSearchParams implementation
  class URLSearchParams {
    #params;
    #url;
    
    constructor(init, url) {
      this.#params = new Map();
      this.#url = url || null;
      
      if (typeof init === 'string') {
        const search = init.startsWith('?') ? init.slice(1) : init;
        if (search) {
          search.split('&').forEach(pair => {
            const [key, value = ''] = pair.split('=').map(decodeURIComponent);
            this.append(key, value);
          });
        }
      } else if (init && typeof init === 'object') {
        // Support object initialization
        for (const key in init) {
          if (Object.prototype.hasOwnProperty.call(init, key)) {
            this.append(key, init[key]);
          }
        }
      }
    }
    
    #updateURL() {
      if (this.#url) {
        const search = this.toString();
        this.#url._updateSearch(search ? '?' + search : '');
      }
    }
    
    append(name, value) {
      const key = String(name);
      const val = String(value);
      if (!this.#params.has(key)) {
        this.#params.set(key, []);
      }
      this.#params.get(key).push(val);
      this.#updateURL();
    }
    
    delete(name) {
      this.#params.delete(String(name));
      this.#updateURL();
    }
    
    get(name) {
      const values = this.#params.get(String(name));
      return values && values.length > 0 ? values[0] : null;
    }
    
    getAll(name) {
      return this.#params.get(String(name)) || [];
    }
    
    has(name) {
      return this.#params.has(String(name));
    }
    
    set(name, value) {
      this.#params.set(String(name), [String(value)]);
      this.#updateURL();
    }
    
    sort() {
      const sorted = new Map([...this.#params.entries()].sort());
      this.#params = sorted;
      this.#updateURL();
    }
    
    toString() {
      const parts = [];
      for (const [key, values] of this.#params) {
        for (const value of values) {
          parts.push(`${encodeURIComponent(key)}=${encodeURIComponent(value)}`);
        }
      }
      return parts.join('&');
    }
    
    *entries() {
      for (const [key, values] of this.#params) {
        for (const value of values) {
          yield [key, value];
        }
      }
    }
    
    *keys() {
      for (const [key] of this.entries()) {
        yield key;
      }
    }
    
    *values() {
      for (const [, value] of this.entries()) {
        yield value;
      }
    }
    
    [Symbol.iterator]() {
      return this.entries();
    }
  }
  
  globalThis.URL = URL;
  globalThis.URLSearchParams = URLSearchParams;
});
