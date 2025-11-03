// URL
// https://url.spec.whatwg.org/

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ urlParse, urlSetHref, urlSetProtocol, urlSetUsername, urlSetPassword, urlSetHost, urlSetHostname, urlSetPort, urlSetPathname, urlSetSearch, urlSetHash, urlSearchParamsNew, urlSearchParamsToString }) => {
  
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
    #href;
    #origin;
    #protocol;
    #username;
    #password;
    #host;
    #hostname;
    #port;
    #pathname;
    #search;
    #hash;
    #searchParams;

    constructor(url, base) {
      const parsed = base !== undefined 
        ? urlParse(String(url), String(base))
        : urlParse(String(url));
      
      if (parsed === null) {
        throw new TypeError('Invalid URL');
      }
      
      // Cache all URL components
      this.#href = parsed.href;
      this.#origin = parsed.origin;
      this.#protocol = parsed.protocol;
      this.#username = parsed.username;
      this.#password = parsed.password;
      this.#host = parsed.host;
      this.#hostname = parsed.hostname;
      this.#port = parsed.port;
      this.#pathname = parsed.pathname;
      this.#search = parsed.search;
      this.#hash = parsed.hash;
      this.#searchParams = null;
    }

    get href() {
      return this.#href;
    }

    set href(value) {
      const parsed = urlSetHref(this.#href, String(value));
      if (parsed === null) {
        throw new TypeError('Invalid URL');
      }
      this.#href = parsed.href;
      this.#origin = parsed.origin;
      this.#protocol = parsed.protocol;
      this.#username = parsed.username;
      this.#password = parsed.password;
      this.#host = parsed.host;
      this.#hostname = parsed.hostname;
      this.#port = parsed.port;
      this.#pathname = parsed.pathname;
      this.#search = parsed.search;
      this.#hash = parsed.hash;
      this.#searchParams = null; // Reset cached searchParams
    }

    get origin() {
      return this.#origin;
    }

    get protocol() {
      return this.#protocol;
    }

    set protocol(value) {
      const parsed = urlSetProtocol(this.#href, String(value));
      if (parsed !== null) {
        this.#href = parsed.href;
        this.#protocol = parsed.protocol;
      }
    }

    get username() {
      return this.#username;
    }

    set username(value) {
      const parsed = urlSetUsername(this.#href, String(value));
      if (parsed !== null) {
        this.#href = parsed.href;
        this.#username = parsed.username;
      }
    }

    get password() {
      return this.#password;
    }

    set password(value) {
      const parsed = urlSetPassword(this.#href, String(value));
      if (parsed !== null) {
        this.#href = parsed.href;
        this.#password = parsed.password;
      }
    }

    get host() {
      return this.#host;
    }

    set host(value) {
      const parsed = urlSetHost(this.#href, String(value));
      if (parsed !== null) {
        this.#href = parsed.href;
        this.#host = parsed.host;
        this.#hostname = parsed.hostname;
        this.#port = parsed.port;
      }
    }

    get hostname() {
      return this.#hostname;
    }

    set hostname(value) {
      const parsed = urlSetHostname(this.#href, String(value));
      if (parsed !== null) {
        this.#href = parsed.href;
        this.#hostname = parsed.hostname;
        this.#host = parsed.host;
      }
    }

    get port() {
      return this.#port;
    }

    set port(value) {
      const parsed = urlSetPort(this.#href, String(value));
      if (parsed !== null) {
        this.#href = parsed.href;
        this.#port = parsed.port;
        this.#host = parsed.host;
      }
    }

    get pathname() {
      return this.#pathname;
    }

    set pathname(value) {
      const parsed = urlSetPathname(this.#href, String(value));
      if (parsed !== null) {
        this.#href = parsed.href;
        this.#pathname = parsed.pathname;
      }
    }

    get search() {
      return this.#search;
    }

    set search(value) {
      const parsed = urlSetSearch(this.#href, String(value));
      if (parsed !== null) {
        this.#href = parsed.href;
        this.#search = parsed.search;
        this.#searchParams = null; // Reset cached searchParams
      }
    }

    get searchParams() {
      if (!this.#searchParams) {
        const search = this.#search;
        this.#searchParams = new URLSearchParams(search);
        
        // Create a wrapper that updates the URL when the params change
        const self = this;
        const params = this.#searchParams;
        
        // Wrap methods that modify the params
        const wrapMethod = (method) => {
          const original = params[method].bind(params);
          return function(...args) {
            const result = original(...args);
            const parsed = urlSetSearch(self.#href, params.toString());
            if (parsed !== null) {
              self.#href = parsed.href;
              self.#search = parsed.search;
            }
            return result;
          };
        };
        
        params.append = wrapMethod('append');
        params.delete = wrapMethod('delete');
        params.set = wrapMethod('set');
        params.sort = wrapMethod('sort');
      }
      
      return this.#searchParams;
    }

    get hash() {
      return this.#hash;
    }

    set hash(value) {
      const parsed = urlSetHash(this.#href, String(value));
      if (parsed !== null) {
        this.#href = parsed.href;
        this.#hash = parsed.hash;
      }
    }

    toString() {
      return this.#href;
    }

    toJSON() {
      return this.#href;
    }
  }

  globalThis.URL = URL;
  globalThis.URLSearchParams = URLSearchParams;
});
