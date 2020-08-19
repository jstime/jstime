// Cosole
// https://console.spec.whatwg.org/#console-namespace

((global) => {
  function printer(loglevel, args) {
    if (Array.isArray(args) && args.length) {
      args = args.join(' ');
    }
    if (loglevel === 'log') {
      _printer(args);
    } else if (loglevel === 'error') {
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

  const _printer = globalThis.printer;
  global.console.log = log;
  global.console.error = error;
  delete global.printer;
})(globalThis);
