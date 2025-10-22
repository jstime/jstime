use jstime_core as jstime;

mod common;

#[cfg(test)]
mod api {
    use super::*;
    #[test]
    fn run_script() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("\"Hello, World!\"", "jstime");
        assert_eq!(result.unwrap(), "Hello, World!");
        let result = jstime.run_script("1 + 1", "jstime");
        assert_eq!(result.unwrap(), "2");
        let result = jstime.run_script("const a = 123; const b = 456; a + b;", "jstime");
        assert_eq!(result.unwrap(), "579");
        let result = jstime.run_script("a", "jstime");
        assert_eq!(result.unwrap(), "123");
    }
    #[test]
    fn run_script_error() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let err = match jstime.run_script("a", "jstime") {
            Ok(_result) => panic!(),
            Err(e) => e,
        };
        assert_eq!(err, "ReferenceError: a is not defined\n    at jstime:1:1");
        let err = match jstime.run_script("}", "jstime") {
            Ok(_result) => panic!(),
            Err(e) => e,
        };
        assert_eq!(err, "SyntaxError: Unexpected token \'}\'");
    }
    #[test]
    fn import() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let hello_path = "./tests/fixtures/modules/hello-world.js";
        let _result = jstime.import(hello_path);
        let result = jstime.run_script("globalThis.hello", "jstime");
        assert_eq!(result.unwrap(), "hello world");
    }

    #[test]
    fn import_json_simple() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create temporary test files
        let temp_dir = std::env::temp_dir();
        let json_file = temp_dir.join("test_simple.json");
        let test_file = temp_dir.join("test_json_simple.js");

        std::fs::write(&json_file, r#"{"message": "hello from json"}"#).unwrap();
        std::fs::write(
            &test_file,
            format!(
                "import data from '{}';\nglobalThis.jsonData = data;",
                json_file.to_str().unwrap()
            ),
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Failed to import JSON: {:?}", result);

        let result = jstime.run_script("globalThis.jsonData.message", "jstime");
        assert_eq!(result.unwrap(), "hello from json");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&json_file).ok();
    }

    #[test]
    fn import_json_complex() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create temporary test files
        let temp_dir = std::env::temp_dir();
        let json_file = temp_dir.join("test_data.json");
        let test_file = temp_dir.join("test_json_complex.js");

        std::fs::write(
            &json_file,
            r#"{
  "name": "jstime",
  "version": "1.0.0",
  "features": ["modules", "timers", "fetch"],
  "nested": {
    "key": "value",
    "number": 42
  }
}"#,
        )
        .unwrap();
        std::fs::write(
            &test_file,
            format!(
                "import data from '{}';\nglobalThis.jsonData = data;",
                json_file.to_str().unwrap()
            ),
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Failed to import JSON: {:?}", result);

        let result = jstime.run_script("globalThis.jsonData.name", "jstime");
        assert_eq!(result.unwrap(), "jstime");

        let result = jstime.run_script("globalThis.jsonData.version", "jstime");
        assert_eq!(result.unwrap(), "1.0.0");

        let result = jstime.run_script("globalThis.jsonData.nested.number", "jstime");
        assert_eq!(result.unwrap(), "42");

        let result = jstime.run_script("Array.isArray(globalThis.jsonData.features)", "jstime");
        assert_eq!(result.unwrap(), "true");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&json_file).ok();
    }

    #[test]
    fn import_json_default_export() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test that JSON is imported as default export
        let temp_dir = std::env::temp_dir();
        let json_file = temp_dir.join("test_default.json");
        let test_file = temp_dir.join("test_json_default.js");

        std::fs::write(&json_file, r#"{"message": "hello from json"}"#).unwrap();
        std::fs::write(
            &test_file,
            format!(
                "import json from '{}';\nglobalThis.checkResult = json.message === 'hello from json';",
                json_file.to_str().unwrap()
            ),
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Failed to import JSON: {:?}", result);

        let result = jstime.run_script("globalThis.checkResult", "jstime");
        assert_eq!(result.unwrap(), "true");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&json_file).ok();
    }
}
