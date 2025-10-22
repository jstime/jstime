// Tests for common helper utilities

mod common;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_test_script_basic() {
        let result = common::run_test_script("1 + 1");
        assert_eq!(result.unwrap(), "2");
    }

    #[test]
    fn test_run_test_script_string() {
        let result = common::run_test_script("'Hello, World!'");
        assert_eq!(result.unwrap(), "Hello, World!");
    }

    #[test]
    fn test_run_test_script_error() {
        let result = common::run_test_script("undefinedVariable");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("ReferenceError"));
    }

    #[test]
    fn test_assert_api_exists_true() {
        assert!(common::assert_api_exists("console"));
        assert!(common::assert_api_exists("setTimeout"));
        assert!(common::assert_api_exists("queueMicrotask"));
    }

    #[test]
    fn test_assert_api_exists_false() {
        assert!(!common::assert_api_exists("nonExistentAPI"));
    }

    #[test]
    fn test_get_type_of_function() {
        let result = common::get_type_of("setTimeout");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn test_get_type_of_object() {
        let result = common::get_type_of("console");
        assert_eq!(result.unwrap(), "object");
    }

    #[test]
    fn test_get_type_of_undefined() {
        let result = common::get_type_of("nonExistentVariable");
        assert_eq!(result.unwrap(), "undefined");
    }

    #[test]
    fn test_multiple_calls_work() {
        // Test that multiple calls to run_test_script work correctly
        let result1 = common::run_test_script("1 + 1");
        let result2 = common::run_test_script("2 + 2");
        let result3 = common::run_test_script("3 + 3");

        assert_eq!(result1.unwrap(), "2");
        assert_eq!(result2.unwrap(), "4");
        assert_eq!(result3.unwrap(), "6");
    }
}
