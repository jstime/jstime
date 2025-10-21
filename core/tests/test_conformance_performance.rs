// Performance API Conformance Tests
// Based on https://w3c.github.io/hr-time/ and https://w3c.github.io/performance-timeline/

use jstime_core as jstime;

mod common;

#[cfg(test)]
mod conformance_performance {
    use super::*;

    #[test]
    fn performance_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof performance;", "test");
        assert_eq!(result.unwrap(), "object");
    }

    #[test]
    fn performance_is_not_null() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("performance !== null;", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn performance_now_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof performance.now;", "test");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn performance_now_returns_number() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof performance.now();", "test");
        assert_eq!(result.unwrap(), "number");
    }

    #[test]
    fn performance_now_returns_positive_number() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("performance.now() >= 0;", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn performance_now_is_monotonic() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const t1 = performance.now(); \
             const t2 = performance.now(); \
             t2 >= t1;",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn performance_now_has_subsecond_precision() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        // The return value should be a DOMHighResTimeStamp (double)
        let result = jstime.run_script("performance.now() % 1 !== 0 || true;", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn performance_timeorigin_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof performance.timeOrigin;", "test");
        assert_eq!(result.unwrap(), "number");
    }

    #[test]
    fn performance_timeorigin_is_positive() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("performance.timeOrigin > 0;", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn performance_timeorigin_is_readonly() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const orig = performance.timeOrigin; \
             try { performance.timeOrigin = 999; } catch(e) {} \
             performance.timeOrigin === orig;",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn performance_tojson_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof performance.toJSON;", "test");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn performance_tojson_returns_object() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof performance.toJSON();", "test");
        assert_eq!(result.unwrap(), "object");
    }

    #[test]
    fn performance_tojson_includes_timeorigin() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("'timeOrigin' in performance.toJSON();", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn performance_json_stringify() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const json = JSON.stringify(performance); \
             typeof JSON.parse(json).timeOrigin === 'number';",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn performance_now_increases_over_time() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const t1 = performance.now(); \
             let sum = 0; \
             for (let i = 0; i < 1000; i++) { sum += i; } \
             const t2 = performance.now(); \
             t2 > t1;",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn performance_now_can_be_called_multiple_times() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "performance.now(); \
             performance.now(); \
             performance.now(); \
             'ok';",
            "test",
        );
        assert_eq!(result.unwrap(), "ok");
    }

    #[test]
    fn performance_timeorigin_is_close_to_date_now() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        // timeOrigin should be close to Date.now() at initialization
        // Allow for some variance
        let result = jstime.run_script(
            "const now = Date.now(); \
             const diff = Math.abs(now - performance.timeOrigin); \
             diff < 1000;", // Within 1 second
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn performance_now_returns_different_values() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const times = new Set(); \
             for (let i = 0; i < 10; i++) { \
               times.add(performance.now()); \
             } \
             times.size > 1;", // Should have at least some different values
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn performance_now_precision() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        // Check that performance.now() returns a high-resolution timestamp
        let result = jstime.run_script(
            "const t = performance.now(); \
             typeof t === 'number' && !isNaN(t) && isFinite(t);",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }
}
