// Cosole
// https://console.spec.whatwg.org/#console-namespace
// Logging
// void assert(optional boolean condition = false, any... data);
// void clear();
// void debug(any... data);
// void error(any... data);
// void info(any... data);
// void log(any... data);
// void table(optional any tabularData, optional sequence<DOMString> properties);
// void trace(any... data);
// void warn(any... data);
// void dir(optional any item, optional object? options);
// void dirxml(any... data);

// log(...data)
// Perform Logger("log", data).

// void error(any... data);
// Perform Logger("error", data).

// Logger(logLevel, args)
// https://console.spec.whatwg.org/#Logger
// 1.  If args is empty, return.
//
// 2. Let first be args[0].
//
// 3. Let rest be all elements following first in args.
//
// 4. If rest is empty, perform Printer(logLevel, « first ») and return.
//
// 5. If first does not contain any format specifiers, perform Printer(logLevel, args).
//
// 6.Otherwise, perform Printer(logLevel, Formatter(args)).
//
// 7. Return undefined.

// Printer(logLevel, args[, options])
// https://console.spec.whatwg.org/#printer

globalThis.console.log = globalThis.printer;
globalThis.printer = undefined;

// printer('🎶 BOUND 🎶');
