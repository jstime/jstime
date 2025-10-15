// URL
// https://url.spec.whatwg.org/

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ urlParse, urlGetHref, urlGetOrigin, urlGetProtocol, urlGetUsername, urlGetPassword, urlGetHost, urlGetHostname, urlGetPort, urlGetPathname, urlGetSearch, urlGetHash, urlSetHref, urlSetProtocol, urlSetUsername, urlSetPassword, urlSetHost, urlSetHostname, urlSetPort, urlSetPathname, urlSetSearch, urlSetHash, urlToJson, urlSearchParamsNew, urlSearchParamsAppend, urlSearchParamsDelete, urlSearchParamsGet, urlSearchParamsGetAll, urlSearchParamsHas, urlSearchParamsSet, urlSearchParamsSort, urlSearchParamsToString, urlSearchParamsEntries }) => {
  
  // URLSearchParams class
  class URLSearchParams {
    #params;

    constructor(init = '') {
      this.#params = [];
      
      if (init instanceof URLSearchParams) {
        for (const [key, value] of init) {
          this.#params.push([key, value]);
        }
      } else if (typeof init === 'object' && init !== null) {
        for (const key of Object.keys(init)) {
          const value = init[key];
          if (Array.isArray(value)) {
            for (const val of value) {
              this.#params.push([key, String(val)]);
            }
          } else {
            this.#params.push([key, String(value)]);
          }
        }
      } else {
        const str = String(init);
        // Remove leading '?' if present
        const query = str.startsWith('?') ? str.slice(1) : str;
        if (query) {
          this.#params = urlSearchParamsNew(query);
        }
      }
    }

    append(name, value) {
      this.#params.push([String(name), String(value)]);
    }

    delete(name) {
      const nameStr = String(name);
      this.#params = this.#params.filter(([key]) => key !== nameStr);
    }

    get(name) {
      const nameStr = String(name);
      for (const [key, value] of this.#params) {
        if (key === nameStr) {
          return value;
        }
      }
      return null;
    }

    getAll(name) {
      const nameStr = String(name);
      const results = [];
      for (const [key, value] of this.#params) {
        if (key === nameStr) {
          results.push(value);
        }
      }
      return results;
    }

    has(name) {
      const nameStr = String(name);
      for (const [key] of this.#params) {
        if (key === nameStr) {
          return true;
        }
      }
      return false;
    }

    set(name, value) {
      const nameStr = String(name);
      const valueStr = String(value);
      let found = false;
      const newParams = [];
      
      for (const [key, val] of this.#params) {
        if (key === nameStr) {
          if (!found) {
            newParams.push([key, valueStr]);
            found = true;
          }
          // Skip other occurrences
        } else {
          newParams.push([key, val]);
        }
      }
      
      if (!found) {
        newParams.push([nameStr, valueStr]);
      }
      
      this.#params = newParams;
    }

    sort() {
      this.#params.sort((a, b) => {
        if (a[0] < b[0]) return -1;
        if (a[0] > b[0]) return 1;
        return 0;
      });
    }

    toString() {
      return urlSearchParamsToString(this.#params);
    }

    *entries() {
      for (const entry of this.#params) {
        yield entry.slice();
      }
    }

    *keys() {
      for (const [key] of this.#params) {
        yield key;
      }
    }

    *values() {
      for (const [, value] of this.#params) {
        yield value;
      }
    }

    [Symbol.iterator]() {
      return this.entries();
    }

    forEach(callbackfn, thisArg) {
      for (const [key, value] of this.#params) {
        callbackfn.call(thisArg, value, key, this);
      }
    }
  }

  // URL class
  class URL {
    #handle;

    constructor(url, base) {
      if (base !== undefined) {
        this.#handle = urlParse(String(url), String(base));
      } else {
        this.#handle = urlParse(String(url));
      }
      
      if (this.#handle === null) {
        throw new TypeError('Invalid URL');
      }
    }

    get href() {
      return urlGetHref(this.#handle);
    }

    set href(value) {
      const newHandle = urlSetHref(this.#handle, String(value));
      if (newHandle === null) {
        throw new TypeError('Invalid URL');
      }
      this.#handle = newHandle;
    }

    get origin() {
      return urlGetOrigin(this.#handle);
    }

    get protocol() {
      return urlGetProtocol(this.#handle);
    }

    set protocol(value) {
      this.#handle = urlSetProtocol(this.#handle, String(value));
    }

    get username() {
      return urlGetUsername(this.#handle);
    }

    set username(value) {
      this.#handle = urlSetUsername(this.#handle, String(value));
    }

    get password() {
      return urlGetPassword(this.#handle);
    }

    set password(value) {
      this.#handle = urlSetPassword(this.#handle, String(value));
    }

    get host() {
      return urlGetHost(this.#handle);
    }

    set host(value) {
      this.#handle = urlSetHost(this.#handle, String(value));
    }

    get hostname() {
      return urlGetHostname(this.#handle);
    }

    set hostname(value) {
      this.#handle = urlSetHostname(this.#handle, String(value));
    }

    get port() {
      return urlGetPort(this.#handle);
    }

    set port(value) {
      this.#handle = urlSetPort(this.#handle, String(value));
    }

    get pathname() {
      return urlGetPathname(this.#handle);
    }

    set pathname(value) {
      this.#handle = urlSetPathname(this.#handle, String(value));
    }

    get search() {
      return urlGetSearch(this.#handle);
    }

    set search(value) {
      this.#handle = urlSetSearch(this.#handle, String(value));
    }

    get searchParams() {
      const search = this.search;
      const params = new URLSearchParams(search);
      
      // Override toString to sync back to URL
      const originalToString = params.toString.bind(params);
      const self = this;
      params.toString = function() {
        const result = originalToString();
        self.search = result;
        return result;
      };
      
      return params;
    }

    get hash() {
      return urlGetHash(this.#handle);
    }

    set hash(value) {
      this.#handle = urlSetHash(this.#handle, String(value));
    }

    toString() {
      return this.href;
    }

    toJSON() {
      return urlToJson(this.#handle);
    }
  }

  globalThis.URL = URL;
  globalThis.URLSearchParams = URLSearchParams;
});
