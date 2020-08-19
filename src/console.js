// Cosole
// https://console.spec.whatwg.org/#console-namespace

const internalSymbol = Symbol('jstime');

globalThis[internalSymbol] = {};

globalThis[internalSymbol]._printer = globalThis.printer;

globalThis[internalSymbol].printer = function printer(loglevel, args) {
  if (args.length) {
    args = args.join(' ');
  }
  if (loglevel === 'log') {
    globalThis[internalSymbol]._printer(args)
  } else if (loglevel === 'error') {
    globalThis[internalSymbol]._printer(args, true)
  }
};

globalThis[internalSymbol].logger = function logger(loglevel, args) {
  if (!args.length) return;
  let first = args[0];
  let rest = args.slice(1);
  if (!rest.length) {
    globalThis[internalSymbol].printer(loglevel, first);
    return;
  }

  // TODO Handle Format Specifiers

  globalThis[internalSymbol].printer(loglevel, args);
};

globalThis.console.log = function log(...args) {
  globalThis[internalSymbol].logger('log', args);
};

globalThis.console.error = function error(...args) {
  globalThis[internalSymbol].logger('error', args);
};

globalThis.printer = undefined;
