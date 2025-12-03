use jstime_core as jstime;

mod common;

#[cfg(test)]
mod completions {
    use super::*;

    #[test]
    fn get_global_names_returns_builtin_apis() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let globals = jstime.get_global_names();

        // Should include jstime built-in APIs
        assert!(globals.contains(&"console".to_string()));
        assert!(globals.contains(&"setTimeout".to_string()));
        assert!(globals.contains(&"setInterval".to_string()));
        assert!(globals.contains(&"clearTimeout".to_string()));
        assert!(globals.contains(&"clearInterval".to_string()));
        assert!(globals.contains(&"fetch".to_string()));
        assert!(globals.contains(&"URL".to_string()));
        assert!(globals.contains(&"URLSearchParams".to_string()));
        assert!(globals.contains(&"crypto".to_string()));
        assert!(globals.contains(&"performance".to_string()));
        assert!(globals.contains(&"process".to_string()));
        assert!(globals.contains(&"atob".to_string()));
        assert!(globals.contains(&"btoa".to_string()));
        assert!(globals.contains(&"TextEncoder".to_string()));
        assert!(globals.contains(&"TextDecoder".to_string()));
        assert!(globals.contains(&"Event".to_string()));
        assert!(globals.contains(&"EventTarget".to_string()));
        assert!(globals.contains(&"structuredClone".to_string()));
        assert!(globals.contains(&"queueMicrotask".to_string()));

        // Should include standard JS built-ins
        assert!(globals.contains(&"Math".to_string()));
        assert!(globals.contains(&"Array".to_string()));
        assert!(globals.contains(&"Object".to_string()));
        assert!(globals.contains(&"String".to_string()));
        assert!(globals.contains(&"Number".to_string()));
        assert!(globals.contains(&"Boolean".to_string()));
        assert!(globals.contains(&"Date".to_string()));
        assert!(globals.contains(&"JSON".to_string()));
        assert!(globals.contains(&"Promise".to_string()));
        assert!(globals.contains(&"RegExp".to_string()));
        assert!(globals.contains(&"Error".to_string()));
        assert!(globals.contains(&"WebAssembly".to_string()));
    }

    #[test]
    fn get_global_names_includes_user_defined() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Define a global variable
        jstime
            .run_script("globalThis.myCustomGlobal = 42;", "test")
            .unwrap();

        let globals = jstime.get_global_names();
        assert!(globals.contains(&"myCustomGlobal".to_string()));
    }

    #[test]
    fn get_property_names_for_console() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let props = jstime.get_property_names("console");

        assert!(props.contains(&"log".to_string()));
        assert!(props.contains(&"error".to_string()));
        assert!(props.contains(&"warn".to_string()));
        assert!(props.contains(&"info".to_string()));
        assert!(props.contains(&"debug".to_string()));
    }

    #[test]
    fn get_property_names_for_math() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let props = jstime.get_property_names("Math");

        // Static properties
        assert!(props.contains(&"PI".to_string()));
        assert!(props.contains(&"E".to_string()));
        // Methods
        assert!(props.contains(&"sin".to_string()));
        assert!(props.contains(&"cos".to_string()));
        assert!(props.contains(&"abs".to_string()));
        assert!(props.contains(&"max".to_string()));
        assert!(props.contains(&"min".to_string()));
        assert!(props.contains(&"random".to_string()));
    }

    #[test]
    fn get_property_names_for_process() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let props = jstime.get_property_names("process");

        assert!(props.contains(&"env".to_string()));
        assert!(props.contains(&"argv".to_string()));
        assert!(props.contains(&"cwd".to_string()));
        assert!(props.contains(&"exit".to_string()));
        assert!(props.contains(&"stdout".to_string()));
        assert!(props.contains(&"stderr".to_string()));
        assert!(props.contains(&"stdin".to_string()));
    }

    #[test]
    fn get_property_names_for_crypto() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let props = jstime.get_property_names("crypto");

        assert!(props.contains(&"getRandomValues".to_string()));
        assert!(props.contains(&"randomUUID".to_string()));
        assert!(props.contains(&"subtle".to_string()));
    }

    #[test]
    fn get_property_names_for_nonexistent_object() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let props = jstime.get_property_names("nonExistentObject");
        assert!(props.is_empty());
    }

    #[test]
    fn get_property_names_for_null() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let props = jstime.get_property_names("null");
        assert!(props.is_empty());
    }

    #[test]
    fn get_property_names_for_undefined() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let props = jstime.get_property_names("undefined");
        assert!(props.is_empty());
    }

    #[test]
    fn get_property_names_for_user_defined_object() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Define a custom object
        jstime
            .run_script("globalThis.myObj = { foo: 1, bar: 2, baz: 3 };", "test")
            .unwrap();

        let props = jstime.get_property_names("myObj");
        assert!(props.contains(&"foo".to_string()));
        assert!(props.contains(&"bar".to_string()));
        assert!(props.contains(&"baz".to_string()));
    }

    #[test]
    fn get_property_names_handles_nested_properties() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // crypto.subtle is a nested object
        let props = jstime.get_property_names("crypto.subtle");

        assert!(props.contains(&"digest".to_string()));
    }
}
