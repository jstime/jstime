//! Consistent error handling utilities for builtins.
//!
//! This module provides helper functions to create V8 exceptions with consistent
//! error messages across all builtin implementations, as well as formatting
//! exceptions with source information and stack traces.

/// Throws a TypeError with the given message.
///
/// # Examples
/// ```ignore
/// throw_type_error(scope, "Expected at least 1 argument");
/// ```
#[inline]
pub(crate) fn throw_type_error(scope: &mut v8::PinScope, message: &str) {
    let message = v8::String::new(scope, message).unwrap();
    let exception = v8::Exception::type_error(scope, message);
    scope.throw_exception(exception);
}

/// Throws a generic Error with the given message.
///
/// # Examples
/// ```ignore
/// throw_error(scope, "Failed to read file");
/// ```
#[inline]
pub(crate) fn throw_error(scope: &mut v8::PinScope, message: &str) {
    let message = v8::String::new(scope, message).unwrap();
    let exception = v8::Exception::error(scope, message);
    scope.throw_exception(exception);
}

/// Throws a RangeError with the given message.
///
/// # Examples
/// ```ignore
/// throw_range_error(scope, "Index out of bounds");
/// ```
#[allow(dead_code)] // Not currently used but provided for consistency
#[inline]
pub(crate) fn throw_range_error(scope: &mut v8::PinScope, message: &str) {
    let message = v8::String::new(scope, message).unwrap();
    let exception = v8::Exception::range_error(scope, message);
    scope.throw_exception(exception);
}

/// Attempts to convert a v8::Value to a Rust string.
/// Returns an error message if conversion fails.
///
/// # Examples
/// ```ignore
/// let s = try_to_rust_string(scope, value, "argument");
/// if let Err(msg) = s {
///     throw_type_error(scope, &msg);
///     return;
/// }
/// ```
pub(crate) fn try_to_rust_string(
    scope: &mut v8::PinScope,
    value: v8::Local<v8::Value>,
    param_name: &str,
) -> Result<String, String> {
    v8::tc_scope!(let tc, scope);
    match value.to_string(tc) {
        Some(s) => Ok(s.to_rust_string_lossy(tc)),
        None => Err(format!("Failed to convert {} to string", param_name)),
    }
}

/// Helper to convert v8::Value to a Rust string, throwing an error if it fails.
/// Returns None if an error was thrown.
///
/// # Examples
/// ```ignore
/// let Some(s) = to_rust_string_or_throw(scope, value, "argument") else {
///     return;
/// };
/// ```
pub(crate) fn to_rust_string_or_throw(
    scope: &mut v8::PinScope,
    value: v8::Local<v8::Value>,
    param_name: &str,
) -> Option<String> {
    match try_to_rust_string(scope, value, param_name) {
        Ok(s) => Some(s),
        Err(msg) => {
            throw_type_error(scope, &msg);
            None
        }
    }
}

/// Checks if the required number of arguments are present.
/// Throws a TypeError if not enough arguments are provided.
///
/// # Examples
/// ```ignore
/// if !check_arg_count(scope, args, 2, "myFunction") {
///     return;
/// }
/// ```
pub(crate) fn check_arg_count(
    scope: &mut v8::PinScope,
    args: &v8::FunctionCallbackArguments,
    min_count: usize,
    function_name: &str,
) -> bool {
    if args.length() < min_count as i32 {
        let msg = format!(
            "{} requires at least {} argument{}",
            function_name,
            min_count,
            if min_count == 1 { "" } else { "s" }
        );
        throw_type_error(scope, &msg);
        false
    } else {
        true
    }
}

/// Attempts to get a function from a v8::Value.
/// Returns Err with an error message if the value is not a function.
///
/// # Examples
/// ```ignore
/// let callback = match try_get_function_result(value) {
///     Ok(f) => f,
///     Err(msg) => {
///         crate::error::throw_type_error(scope, &msg);
///         return;
///     }
/// };
/// ```
pub(crate) fn try_get_function_result<'s>(
    value: v8::Local<'s, v8::Value>,
) -> Result<v8::Local<'s, v8::Function>, &'static str> {
    v8::Local::<v8::Function>::try_from(value).map_err(|_| "Value must be a function")
}

/// Attempts to get an object from a v8::Value.
/// Returns Err with an error message if the value is not an object.
///
/// # Examples
/// ```ignore
/// let obj = match try_get_object_result(value) {
///     Ok(o) => o,
///     Err(msg) => {
///         crate::error::throw_type_error(scope, msg);
///         return;
///     }
/// };
/// ```
#[allow(dead_code)] // Not currently used but provided for consistency
pub(crate) fn try_get_object_result<'s>(
    value: v8::Local<'s, v8::Value>,
) -> Result<v8::Local<'s, v8::Object>, &'static str> {
    v8::Local::<v8::Object>::try_from(value).map_err(|_| "Value must be an object")
}

/// Attempts to get an array from a v8::Value.
/// Returns Err with an error message if the value is not an array.
///
/// # Examples
/// ```ignore
/// let arr = match try_get_array_result(value) {
///     Ok(a) => a,
///     Err(msg) => {
///         crate::error::throw_type_error(scope, msg);
///         return;
///     }
/// };
/// ```
pub(crate) fn try_get_array_result<'s>(
    value: v8::Local<'s, v8::Value>,
) -> Result<v8::Local<'s, v8::Array>, &'static str> {
    v8::Local::<v8::Array>::try_from(value).map_err(|_| "Value must be an array")
}

/// Format an exception with file name, line number, source code, and stack trace.
/// This provides detailed error information similar to Node.js.
///
/// Returns a formatted error string that includes:
/// - File path and line number
/// - Source code line with error
/// - Caret (^^^) pointing to error location
/// - Error message
/// - Stack trace (when available)
pub(crate) fn format_exception(
    tc: &mut v8::PinnedRef<'_, v8::TryCatch<v8::HandleScope>>,
) -> String {
    // Get the exception value
    let exception = match tc.exception() {
        Some(e) => e,
        None => return "Unknown error".to_string(),
    };

    // Get the error message from the exception
    let exception_string = {
        let isolate: &v8::Isolate = tc;
        exception
            .to_string(tc)
            .map(|s| s.to_rust_string_lossy(isolate))
            .unwrap_or_else(|| "Unknown error".to_string())
    };

    // Try to get the Message object for detailed error information
    if let Some(message) = tc.message() {
        let mut output = String::new();

        // Get file name and line number
        let resource_name = {
            let isolate: &v8::Isolate = tc;
            message
                .get_script_resource_name(tc)
                .and_then(|v| v.to_string(tc))
                .map(|s| s.to_rust_string_lossy(isolate))
        };

        let line_number = message.get_line_number(tc);

        // Get source line if available
        let source_line = {
            let isolate: &v8::Isolate = tc;
            message
                .get_source_line(tc)
                .map(|s| s.to_string(tc).unwrap().to_rust_string_lossy(isolate))
        };

        // Get column information
        let start_column = message.get_start_column();
        let end_column = message.get_end_column();

        // Format the output similar to Node.js
        if let (Some(file), Some(line)) = (resource_name, line_number) {
            output.push_str(&format!("{}:{}\n", file, line));

            // Add source line if available
            if let Some(source) = source_line {
                output.push_str(&source);
                output.push('\n');

                // Add caret indicator
                // Add spaces for indentation
                for _ in 0..start_column {
                    output.push(' ');
                }

                // Add carets
                let caret_count = (end_column - start_column).max(1);

                for _ in 0..caret_count {
                    output.push('^');
                }
                output.push('\n');
            }
        }

        // Add the error message
        output.push('\n');
        output.push_str(&exception_string);

        // Try to get stack trace - check if the exception has a stack property
        if let Ok(exception_obj) = v8::Local::<v8::Object>::try_from(exception) {
            // Get cached "stack" string  
            let stack_key = {
                let isolate_for_state: &mut v8::Isolate = tc;
                let state = crate::IsolateState::get(isolate_for_state);
                let cache = state.borrow().string_cache.clone();
                let mut cache_borrow = cache.borrow_mut();
                crate::get_cached_string!(cache_borrow, tc, stack, "stack")
            };
            if let Some(stack_val) = exception_obj.get(tc, stack_key.into())
                && let Some(stack_str) = stack_val.to_string(tc)
            {
                let isolate: &v8::Isolate = tc;
                let stack = stack_str.to_rust_string_lossy(isolate);
                // Only add stack if it's different from the exception string
                // and contains actual stack information
                if !stack.is_empty() && stack != exception_string && stack.contains('\n') {
                    // The stack already includes the error message in most cases,
                    // so we'll use it as-is if it contains the error message,
                    // otherwise append it
                    if stack.starts_with(&exception_string) || stack.contains(&exception_string) {
                        output = String::new();
                        if let (Some(file), Some(line)) = (
                            message
                                .get_script_resource_name(tc)
                                .and_then(|v| v.to_string(tc))
                                .map(|s| s.to_rust_string_lossy(isolate)),
                            message.get_line_number(tc),
                        ) {
                            output.push_str(&format!("{}:{}\n", file, line));

                            if let Some(source) = message
                                .get_source_line(tc)
                                .map(|s| s.to_string(tc).unwrap().to_rust_string_lossy(isolate))
                            {
                                output.push_str(&source);
                                output.push('\n');

                                // Add spaces for indentation
                                for _ in 0..start_column {
                                    output.push(' ');
                                }

                                let caret_count = (end_column - start_column).max(1);

                                for _ in 0..caret_count {
                                    output.push('^');
                                }
                                output.push('\n');
                            }
                            output.push('\n');
                        }
                        output.push_str(&stack);
                    } else {
                        output.push('\n');
                        output.push_str(&stack);
                    }
                }
            }
        }

        output
    } else {
        // If no message object, fall back to just the exception string
        exception_string
    }
}
