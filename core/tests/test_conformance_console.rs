// Console API Conformance Tests
// Based on https://console.spec.whatwg.org/

use jstime_core as jstime;

mod common;

#[cfg(test)]
mod conformance_console {
    use super::*;

    #[test]
    fn console_exists_on_global() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("globalThis.hasOwnProperty('console');", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn console_is_not_enumerable() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "Object.getOwnPropertyDescriptor(globalThis, 'console').enumerable;",
            "test",
        );
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn console_is_writable() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "Object.getOwnPropertyDescriptor(globalThis, 'console').writable;",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn console_is_configurable() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "Object.getOwnPropertyDescriptor(globalThis, 'console').configurable;",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn console_constructor_does_not_exist() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("'Console' in globalThis;", "test");
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn console_has_required_methods() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test that all required methods exist
        let methods = vec![
            "assert", "clear", "count", "countReset", "debug", "dir", "dirxml", "error",
            "group", "groupCollapsed", "groupEnd", "info", "log", "table", "time", "timeEnd",
            "timeLog", "trace", "warn",
        ];

        for method in methods {
            let script = format!("typeof console.{};", method);
            let result = jstime.run_script(&script, "test");
            assert_eq!(
                result.unwrap(),
                "function",
                "console.{} should be a function",
                method
            );
        }
    }

    #[test]
    fn console_log_accepts_multiple_arguments() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        // Should not throw
        let result = jstime.run_script("console.log('a', 'b', 'c'); 'ok';", "test");
        assert_eq!(result.unwrap(), "ok");
    }

    #[test]
    fn console_methods_return_undefined() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("console.log('test');", "test");
        assert_eq!(result.unwrap(), "undefined");
    }

    #[test]
    fn console_count_increments() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        // count should work without throwing
        let result = jstime.run_script(
            "console.count('label'); console.count('label'); 'ok';",
            "test",
        );
        assert_eq!(result.unwrap(), "ok");
    }

    #[test]
    fn console_time_and_timeend() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "console.time('timer'); console.timeEnd('timer'); 'ok';",
            "test",
        );
        assert_eq!(result.unwrap(), "ok");
    }

    #[test]
    fn console_group_methods() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "console.group('g1'); console.groupCollapsed('g2'); console.groupEnd(); console.groupEnd(); 'ok';",
            "test",
        );
        assert_eq!(result.unwrap(), "ok");
    }

    #[test]
    fn console_assert_with_falsy_value() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        // assert with false should not throw
        let result = jstime.run_script("console.assert(false, 'message'); 'ok';", "test");
        assert_eq!(result.unwrap(), "ok");
    }

    #[test]
    fn console_assert_with_truthy_value() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("console.assert(true, 'message'); 'ok';", "test");
        assert_eq!(result.unwrap(), "ok");
    }
}
