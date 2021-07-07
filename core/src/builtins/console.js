// Console
// https://console.spec.whatwg.org/#console-namespace

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ printer: _printer }) => {
  function printer(loglevel, args) {
    if (Array.isArray(args) && args.length) {
      args = args.join(' ');
    }
    if (loglevel === 'log' || loglevel === 'info' || loglevel === 'debug') {
      _printer(args);
    } else if (loglevel === 'error' || loglevel === 'warn') {
      _printer(args, true);
    }
  }

  function formatter(args) {
    let target = args[0];
    let current = args[1];

    let i = target.indexOf(' %')
    if (i === -1) {
      return args;
    }

    let converted;
    const specifier = target[i + 2];
    if (specifier === 's') {
      converted = String(current);
    } else if (specifier === 'd' || specifier === 'i') {
      if (typeof current === 'symbol') {
        converted = NaN;
      } else {
        converted = parseInt(current, 10);
      }
    } else if (specifier === 'f') {
      if (typeof current === 'symbol') {
        converted = NaN;
      } else {
        converted = parseFloat(current);
      }
      // TODO: %o, %O
    }

    if (converted) {
      target = target.substring(0, i + 1) + converted + target.substring(i + 3, target.length);
    }

    const result = [target, ...args.slice(2)];
    if (i === target.length - 3 || result.length === 1) {
      return result;
    }

    return (formatter(result));
  }

  function logger(loglevel, args) {
    if (!args.length) {
      return;
    }
    const first = args[0];
    const rest = args.slice(1);
    if (!rest.length) {
      printer(loglevel, first);
      return;
    }

    printer(loglevel, formatter(args));
  }

  let inConsoleCall = false;
  function consoleCall(inspectorMethod, level, ...args) {
    if (!inConsoleCall) {
      inConsoleCall = true;
      try {
        inspectorMethod(...args);
      } finally {
        inConsoleCall = false;
      }
    }
    logger(level, args);
  }

  [
    'log', 'info',
    'debug', 'error',
    'warn',
  ].forEach((level) => {
    const inspectorMethod = globalThis.console[level];
    // Combine V8 inspector console methods and jstime methods
    if (inspectorMethod) {
      globalThis.console[level] = consoleCall.bind(undefined, inspectorMethod, level);
    } else {
      globalThis.console[level] = (...args) => {
        logger(level, args);
      };
    }
    Object.defineProperty(globalThis.console[level], 'name', {
      value: level,
    });
  });
});
