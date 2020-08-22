// Cosole
// https://console.spec.whatwg.org/#console-namespace

((global) => {
  function printer(loglevel, args) {
    if (Array.isArray(args) && args.length) {
      args = args.join(' ');
    }
    if (loglevel === 'log' || loglevel === 'info' || loglevel === 'debug') {
      _printer(args);
    } else if (loglevel === 'error' || loglevel === 'warn') {
      _printer(args, true);
    }
  };

  function logger(loglevel, args) {
    if (!args.length) return;
    let first = args[0];
    let rest = args.slice(1);
    if (!rest.length) {
      printer(loglevel, first);
      return;
    }

    // TODO Handle Format Specifiers

    printer(loglevel, args);
  };

  function log(...args) {
    logger('log', args);
  };

  function error(...args) {
    logger('error', args);
  };

  function info(...args) {
    logger('info', args)
  }

  function debug(...args) {
    logger('debug', args);
  }

  function warn(...args) {
    logger('warn', args);
  }

  const _printer = globalThis.printer;
  global.console.log = log;
  global.console.error = error;
  global.console.info = info;
  global.console.debug = debug;
  global.console.warn = warn;
  delete global.printer;

  const queueMicrotask = global.queueMicrotask;
  global.queueMicrotask = cb => {
    if (typeof cb !== 'function') {
      throw new TypeError('queueMicrotask requires a callback function');
    }
    return queueMicrotask(cb);
  };
})(globalThis);
