# JavaScript Runtime

This document describes jstime's JavaScript runtime capabilities, including language support, REPL, and script execution.

## Table of Contents

- [JavaScript Language Support](#javascript-language-support)
- [REPL](#repl)
- [Running Scripts](#running-scripts)
- [Limitations and Future Work](#limitations-and-future-work)

## JavaScript Language Support

jstime uses V8 as its JavaScript engine, providing full support for modern JavaScript features including:

- **ES2015+ (ES6+)**: All modern JavaScript syntax and features
- **Async/Await**: Asynchronous programming with async functions and await expressions
- **Top-level await**: Use await at the top level of ES modules
- **Promises**: Native Promise support for asynchronous operations
- **Classes**: ES6 class syntax
- **Arrow functions**: Concise function expressions
- **Template literals**: String interpolation and multi-line strings
- **Destructuring**: Object and array destructuring
- **Spread operator**: Spread syntax for arrays and objects
- **And more**: All standard JavaScript features supported by V8

### Example

```javascript
// Modern JavaScript features work out of the box
const greet = (name) => `Hello, ${name}!`;
console.log(greet('World'));

// Classes
class Person {
  constructor(name) {
    this.name = name;
  }
  
  greet() {
    return `Hello, I'm ${this.name}`;
  }
}

const person = new Person('Alice');
console.log(person.greet());

// Async/await
async function fetchData() {
  const result = await Promise.resolve('data');
  return result;
}
```


## REPL

jstime provides an interactive REPL (Read-Eval-Print Loop) for experimenting with JavaScript code.

### Features

- Interactive JavaScript shell
- Command history (saved to `~/.jstime_repl_history`)
- Tab completion for globals, built-in objects, and properties
- Multi-line input support
- Access to all jstime APIs

### Starting the REPL

```bash
# Start the REPL
$ jstime

Welcome to jstime v<version>!

>>
```

### REPL Examples

```javascript
>> 2 + 2
4

>> const name = 'Alice'
Alice

>> console.log(`Hello, ${name}!`)
Hello, Alice!
undefined

>> setTimeout(() => console.log('Delayed'), 1000)
1
Delayed

>> fetch('https://api.github.com')
  .then(r => r.json())
  .then(d => console.log(d))
Promise { <pending> }
>> // Result appears after promise resolves
```

### Tab Completion

The REPL supports tab completion:

- Type `cons` and press Tab → suggests `console`
- Type `console.` and press Tab → shows console methods
- Completion works for JavaScript built-ins and jstime APIs

### Exiting the REPL

Press `Ctrl+C` or `Ctrl+D` to exit the REPL.

## Running Scripts

jstime can execute JavaScript files directly:

```bash
# Run a JavaScript file
$ jstime script.js

# Run a module
$ jstime module.mjs
```

## Limitations and Future Work

While jstime provides a solid foundation for JavaScript execution with many web standard APIs, there are some limitations:

- **No WebSocket support**: WebSocket API not yet implemented
- **Limited DOM APIs**: No DOM APIs (this is a server-side runtime)
- **Limited Node.js compatibility**: Only a subset of Node.js APIs are available

Future enhancements being considered:

- **WASI (WebAssembly System Interface)**: Support for WASI to enable WebAssembly modules to access additional system APIs
- **Additional Web APIs**: More browser APIs as they become relevant
- **Expanded Node.js compatibility**: Additional Node.js APIs where applicable

## Additional Resources
