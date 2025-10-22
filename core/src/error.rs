//! Consistent error handling utilities for builtins.
//!
//! This module provides helper functions to create V8 exceptions with consistent
//! error messages across all builtin implementations.

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
