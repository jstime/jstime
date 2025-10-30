// Error Handling Examples - Demonstrating jstime's enhanced error messages
//
// This file demonstrates various error types and color-coded output in jstime.
// Run with: jstime examples/error-handling-demo.js
//
// To see colored output, use: FORCE_COLOR=1 jstime examples/error-handling-demo.js
// To disable colors, use: NO_COLOR=1 jstime examples/error-handling-demo.js

console.log("=== jstime Error Handling Demo ===\n");

// Example 1: ReferenceError - Undefined variable
console.log("1. ReferenceError Example:");
console.log("   Uncomment the line below to see an undefined variable error:");
console.log("   // console.log(undefinedVariable);");
console.log("   Output: ReferenceError with file location and stack trace\n");

// Example 2: TypeError - Null/undefined access
console.log("2. TypeError Example:");
console.log("   Uncomment the lines below to see a null access error:");
console.log("   // const obj = null;");
console.log("   // console.log(obj.property);");
console.log("   Output: TypeError with error location\n");

// Example 3: TypeError - Not a function
console.log("3. TypeError Example (not a function):");
console.log("   Uncomment the lines below to see a 'not a function' error:");
console.log("   // const notAFunction = 42;");
console.log("   // notAFunction();");
console.log("   Output: TypeError explaining the issue\n");

// Example 4: SyntaxError - Missing operand
console.log("4. SyntaxError Example:");
console.log("   Uncomment the line below to see a syntax error:");
console.log("   // const result = 10 +;");
console.log("   Output: SyntaxError with source line and caret indicator\n");

// Example 5: Stack traces
console.log("5. Stack Trace Example:");
console.log("   Uncomment the code below to see a full stack trace:");
console.log("   // function outer() {");
console.log("   //     inner();");
console.log("   // }");
console.log("   // function inner() {");
console.log("   //     throw new Error('Something went wrong');");
console.log("   // }");
console.log("   // outer();");
console.log("   Stack trace shows: inner() -> outer() -> script\n");

// Example 6: Good error handling practices
console.log("6. Best Practices:");
console.log("   ✓ Check the stack trace to find where the error originated");
console.log("   ✓ Look at the source line with the ^^^ indicator");
console.log("   ✓ Use try-catch for expected errors");
console.log("   ✓ Enable colors for better readability (FORCE_COLOR=1)");

// Demonstrate proper error handling
try {
    // This would normally throw an error
    // throw new Error("Handled error");
    console.log("\n✓ All examples completed successfully!");
} catch (e) {
    console.error("Caught error:", e.message);
}

console.log("\n=== Demo Complete ===");
console.log("To see actual errors, uncomment the example code above.");
