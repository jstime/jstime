use jstime_core as jstime;

mod common;

#[cfg(test)]
mod warmup {
    use super::*;

    #[test]
    fn test_warmup_with_zero_iterations() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default().with_warmup(0);
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("1 + 1", "test_warmup.js");
        assert_eq!(result.unwrap(), "2");
    }

    #[test]
    fn test_warmup_with_script() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default().with_warmup(5);
        let mut jstime = jstime::JSTime::new(options);

        // Test with simple arithmetic
        let result = jstime.run_script("10 * 5", "test_warmup.js");
        assert_eq!(result.unwrap(), "50");

        // Test with function
        let result = jstime.run_script(
            "function fib(n) { return n <= 1 ? n : fib(n-1) + fib(n-2); } fib(10)",
            "test_warmup.js",
        );
        assert_eq!(result.unwrap(), "55");
    }

    #[test]
    fn test_warmup_with_state() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default().with_warmup(3);
        let mut jstime = jstime::JSTime::new(options);

        // First script defines a variable using var (which allows redeclaration)
        let result = jstime.run_script("var counter = 0; counter", "test_warmup.js");
        assert_eq!(result.unwrap(), "0");

        // Second script increments it - warmup will run this 3 times, then once more for real
        // So counter will be incremented 4 times total (3 warmup + 1 actual)
        let result = jstime.run_script("counter++; counter", "test_warmup.js");
        assert_eq!(result.unwrap(), "4");

        // Third script reads it
        let result = jstime.run_script("counter", "test_warmup.js");
        assert_eq!(result.unwrap(), "4");
    }

    #[test]
    fn test_warmup_with_builtin_apis() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default().with_warmup(5);
        let mut jstime = jstime::JSTime::new(options);

        // Test with console API
        let result = jstime.run_script("console.log('test'); 'ok'", "test_warmup.js");
        assert_eq!(result.unwrap(), "ok");

        // Test with Math API
        let result = jstime.run_script("Math.max(1, 2, 3, 4, 5)", "test_warmup.js");
        assert_eq!(result.unwrap(), "5");
    }

    #[test]
    fn test_warmup_with_error() {
        let _setup_guard = common::setup();
        // Disable colors for consistent test output
        unsafe {
            std::env::set_var("NO_COLOR", "1");
        }

        let options = jstime::Options::default().with_warmup(3);
        let mut jstime = jstime::JSTime::new(options);

        // Warmup should fail on error
        let result = jstime.run_script("throw new Error('test error')", "test_warmup.js");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Error: test error"));
    }

    #[test]
    fn test_warmup_preserves_optimization() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default().with_warmup(10);
        let mut jstime = jstime::JSTime::new(options);

        // Complex script that benefits from JIT optimization
        let script = r#"
            function complexCalculation(n) {
                let result = 0;
                for (let i = 0; i < n; i++) {
                    result += Math.sqrt(i) * Math.sin(i);
                }
                return Math.floor(result);
            }
            complexCalculation(100)
        "#;

        let result = jstime.run_script(script, "test_warmup.js");
        assert!(result.is_ok());
    }
}
