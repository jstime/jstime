# Error Handling and Debugging

jstime provides enhanced error messages with helpful hints and color-coded output to make debugging easier.

## Enhanced Error Formatting

Errors in jstime include:
- **File path and line number** (in cyan)
- **Source code line** where the error occurred
- **Error indicator** (^^^) pointing to the exact location (in red)
- **Error message** with type and description (in red/bold)
- **Helpful hints** for common errors (in yellow)
- **Stack trace** showing the call stack (in gray)

### Example Error Output

```javascript
// example.js
const x = 5;
console.log(undefinedVar);
```

Running this produces:
```
example.js:2
console.log(undefinedVar);
            ^^^^^^^^^^^^
^

ReferenceError: undefinedVar is not defined
    at example.js:2:13
💡 Hint: 'undefinedVar' is not defined. Did you forget to declare it with 'const', 'let', or 'var'?
```

## Color Control

Colors can be controlled via environment variables:

- **`NO_COLOR=1`** - Disable colored output
- **`FORCE_COLOR=1`** - Force colored output even in non-TTY environments

Examples:
```bash
# Disable colors
NO_COLOR=1 jstime script.js

# Force colors
FORCE_COLOR=1 jstime script.js
```

## Error Hints

jstime provides helpful hints for common JavaScript errors:

### ReferenceError: Variable Not Defined

```javascript
console.log(myVar);
// ReferenceError: myVar is not defined
// 💡 Hint: 'myVar' is not defined. Did you forget to declare it with 'const', 'let', or 'var'?
```

### TypeError: Cannot Read Properties

```javascript
const obj = null;
console.log(obj.property);
// TypeError: Cannot read properties of null (reading 'property')
// 💡 Hint: You're trying to access a property on an undefined or null value. Check that the object exists before accessing its properties.
```

### TypeError: Not a Function

```javascript
const notAFunction = 5;
notAFunction();
// TypeError: notAFunction is not a function
// 💡 Hint: You're trying to call something that isn't a function. Check the type of the value you're calling.
```

### SyntaxError: Missing Operand

```javascript
const result = 10 +;
// SyntaxError: Unexpected token ';'
// 💡 Hint: The line has an operator without a right-hand side. Check if you're missing the second operand.
```

### SyntaxError: Mismatched Parentheses/Brackets/Braces

```javascript
const arr = [1, 2, 3;
// SyntaxError: Unexpected token ';'
// 💡 Hint: Check for matching brackets. Each opening '[' needs a closing ']'.
```

### Async/Await Errors

```javascript
function regularFunction() {
    await fetch('https://example.com');
}
// SyntaxError: await is only valid in async function
// 💡 Hint: You can only use 'await' inside an async function. Add 'async' before the function keyword.
```

## Stack Traces

Stack traces show the call chain leading to the error:

```javascript
function foo() {
    bar();
}

function bar() {
    throw new Error("Something went wrong");
}

foo();
```

Output:
```
Error: Something went wrong
    at bar (script.js:6:11)
    at foo (script.js:2:5)
    at script.js:9:1
```

## REPL Error Handling

The REPL provides the same enhanced error messages with proper formatting:

```
>> undefinedVar
REPL:1
undefinedVar
^^^^^^^^^^^^
^

ReferenceError: undefinedVar is not defined
    at REPL:1:1
💡 Hint: 'undefinedVar' is not defined. Did you forget to declare it with 'const', 'let', or 'var'?
```

## Source Maps (Future Feature)

Source maps support is planned for future releases. This will allow mapping errors in transpiled/bundled code back to the original source for easier debugging.

When implemented, source maps will:
- Automatically detect inline and external source maps
- Map error locations to original source files
- Display original source code in error messages
- Support standard source map format (v3)

The infrastructure is in place in `core/src/sourcemap.rs` for future implementation.

## Best Practices

### 1. Read the Hints

Pay attention to the helpful hints provided with errors. They often suggest the fix:

```javascript
// ❌ Bad
console.log(result);  // ReferenceError

// ✅ Good - follow the hint
const result = 42;
console.log(result);
```

### 2. Check the Stack Trace

The stack trace shows where the error originated:

```
Error: File not found
    at readConfig (config.js:15:11)  ← Error thrown here
    at main (app.js:5:3)              ← Called from here
    at app.js:10:1                    ← Called from here
```

### 3. Use the REPL for Quick Testing

Test small code snippets in the REPL to understand errors:

```bash
$ jstime
>> const x = 5
5
>> x +
REPL:1
x +
  ^

SyntaxError: Unexpected token
💡 Hint: The line has an operator without a right-hand side...
```

### 4. Enable Colors for Better Readability

Colored output makes it easier to spot important information:
- **Cyan** = File location
- **Red** = Error and indicators
- **Yellow** = Helpful hints
- **Gray** = Stack traces

## Error Types

### JavaScript Standard Errors

jstime supports all standard JavaScript error types:

- **Error** - Generic error
- **ReferenceError** - Variable not defined
- **TypeError** - Type mismatch or invalid operation
- **SyntaxError** - Invalid syntax
- **RangeError** - Value out of valid range

### Custom Errors

You can create custom errors in your code:

```javascript
class ValidationError extends Error {
    constructor(message) {
        super(message);
        this.name = 'ValidationError';
    }
}

throw new ValidationError('Invalid input');
// ValidationError: Invalid input
//     at script.js:7:7
```

## See Also

- [Runtime Features](../runtime.md) - JavaScript runtime capabilities
- [Web APIs](web-apis.md) - Available Web APIs
- [Module System](modules.md) - ES modules support
