use jstime_core as jstime;

mod common;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_events_module_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/events/test-module-exists.js");
        let result = jstime.run_script("globalThis.testEventsModuleExists", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_basic_emit() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/events/test-basic-emit.js");
        let result = jstime.run_script("globalThis.testBasicEmit", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_once() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/events/test-once.js");
        let result = jstime.run_script("globalThis.testOnce", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_remove_listener() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/events/test-remove-listener.js");
        let result = jstime.run_script("globalThis.testRemoveListener", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_named_imports() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/events/test-named-imports.js");
        let result = jstime.run_script("globalThis.testNamedImports", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_default_import() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/events/test-default-import.js");
        let result = jstime.run_script("globalThis.testDefaultImport", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_eventtarget_inheritance() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/events/test-eventtarget-inheritance.js");
        let result = jstime.run_script("globalThis.testEventTargetInheritance", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_listener_count() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/events/test-listener-count.js");
        let result = jstime.run_script("globalThis.testListenerCount", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_prepend_listener() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/events/test-prepend-listener.js");
        let result = jstime.run_script("globalThis.testPrependListener", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_remove_all_listeners() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/events/test-remove-all-listeners.js");
        let result = jstime.run_script("globalThis.testRemoveAllListeners", "test");
        assert_eq!(result.unwrap(), "true");
    }
}
