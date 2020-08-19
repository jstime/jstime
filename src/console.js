// Cosole
// https://console.spec.whatwg.org/#console-namespace

const kConsole = Symbol('console');

globalThis[kConsole] = {};

globalThis[kConsole]._printer = globalThis.printer;

globalThis[kConsole].printer = function printer(loglevel, args) {
  if (args.length) {
    args = args.join(' ');
  }
  if (loglevel === 'log') {
    globalThis[kConsole]._printer(args)
  } else if (loglevel === 'error') {
    globalThis[kConsole]._printer(args, true)
  }
};

globalThis[kConsole].logger = function logger(loglevel, args) {
  if (!args.length) return;
  let first = args[0];
  let rest = args.slice(1);
  if (!rest.length) {
    globalThis[kConsole].printer(loglevel, first);
    return;
  }

  // TODO Handle Format Specifiers

  globalThis[kConsole].printer(loglevel, args);
};

globalThis.console.log = function log(...args) {
  globalThis[kConsole].logger('log', args);
};

globalThis.console.error = function error(...args) {
  globalThis[kConsole].logger('error', args);
};

delete globalThis.printer;
