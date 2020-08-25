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

    // TODO Handle Format Specifiers

    printer(loglevel, args);
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
