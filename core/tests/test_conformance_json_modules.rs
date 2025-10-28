// JSON Modules Conformance Tests
// Based on TC39 proposal: https://github.com/tc39/proposal-json-modules
// And Node.js JSON module behavior

use jstime_core as jstime;

mod common;

#[cfg(test)]
mod conformance_json_modules {
    use super::*;

    #[test]
    fn json_module_imports_as_default() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create a test JSON file
        let temp_dir = std::env::temp_dir();
        let json_file = temp_dir.join("conformance_test.json");
        let test_file = temp_dir.join("conformance_test_import.js");

        std::fs::write(&json_file, r#"{"test": "value"}"#).unwrap();
        std::fs::write(
            &test_file,
            format!(
                "import data from '{}';\nglobalThis.result = data;",
                json_file.to_str().unwrap()
            ),
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok());

        // JSON should be imported as default export
        let result = jstime.run_script("typeof globalThis.result", "test");
        assert_eq!(result.unwrap(), "object");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&json_file).ok();
    }

    #[test]
    fn json_module_preserves_primitives() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let json_file = temp_dir.join("primitives.json");
        let test_file = temp_dir.join("test_primitives.js");

        std::fs::write(
            &json_file,
            r#"{"string": "text", "number": 42, "boolean": true, "null": null}"#,
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

        jstime.import(test_file.to_str().unwrap()).unwrap();

        // Check string type
        let result = jstime.run_script("typeof globalThis.jsonData.string", "test");
        assert_eq!(result.unwrap(), "string");

        // Check number type
        let result = jstime.run_script("typeof globalThis.jsonData.number", "test");
        assert_eq!(result.unwrap(), "number");

        // Check boolean type
        let result = jstime.run_script("typeof globalThis.jsonData.boolean", "test");
        assert_eq!(result.unwrap(), "boolean");

        // Check null (typeof null is 'object' in JavaScript)
        let result = jstime.run_script("globalThis.jsonData.null === null", "test");
        assert_eq!(result.unwrap(), "true");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&json_file).ok();
    }

    #[test]
    fn json_module_preserves_arrays() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let json_file = temp_dir.join("array.json");
        let test_file = temp_dir.join("test_array.js");

        std::fs::write(&json_file, r#"{"items": [1, 2, 3]}"#).unwrap();
        std::fs::write(
            &test_file,
            format!(
                "import data from '{}';\nglobalThis.arr = data.items;",
                json_file.to_str().unwrap()
            ),
        )
        .unwrap();

        jstime.import(test_file.to_str().unwrap()).unwrap();

        // Check that arrays are preserved
        let result = jstime.run_script("Array.isArray(globalThis.arr)", "test");
        assert_eq!(result.unwrap(), "true");

        // Check array length
        let result = jstime.run_script("globalThis.arr.length", "test");
        assert_eq!(result.unwrap(), "3");

        // Check array elements
        let result = jstime.run_script("globalThis.arr[0]", "test");
        assert_eq!(result.unwrap(), "1");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&json_file).ok();
    }

    #[test]
    fn json_module_preserves_nested_objects() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let json_file = temp_dir.join("nested.json");
        let test_file = temp_dir.join("test_nested.js");

        std::fs::write(&json_file, r#"{"outer": {"inner": {"value": 42}}}"#).unwrap();
        std::fs::write(
            &test_file,
            format!(
                "import data from '{}';\nglobalThis.nested = data;",
                json_file.to_str().unwrap()
            ),
        )
        .unwrap();

        jstime.import(test_file.to_str().unwrap()).unwrap();

        // Check nested access
        let result = jstime.run_script("globalThis.nested.outer.inner.value", "test");
        assert_eq!(result.unwrap(), "42");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&json_file).ok();
    }

    #[test]
    fn json_module_supports_reexport() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let json_file = temp_dir.join("reexport_data.json");
        let reexport_file = temp_dir.join("reexport.js");
        let test_file = temp_dir.join("test_reexport.js");

        std::fs::write(&json_file, r#"{"reexported": true}"#).unwrap();
        std::fs::write(
            &reexport_file,
            format!(
                "export {{ default as data }} from '{}';",
                json_file.to_str().unwrap()
            ),
        )
        .unwrap();
        std::fs::write(
            &test_file,
            format!(
                "import {{ data }} from '{}';\nglobalThis.reexportedData = data;",
                reexport_file.to_str().unwrap()
            ),
        )
        .unwrap();

        jstime.import(test_file.to_str().unwrap()).unwrap();

        let result = jstime.run_script("globalThis.reexportedData.reexported", "test");
        assert_eq!(result.unwrap(), "true");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&reexport_file).ok();
        std::fs::remove_file(&json_file).ok();
    }

    #[test]
    fn json_module_empty_object() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let json_file = temp_dir.join("empty.json");
        let test_file = temp_dir.join("test_empty.js");

        std::fs::write(&json_file, r#"{}"#).unwrap();
        std::fs::write(
            &test_file,
            format!(
                "import data from '{}';\nglobalThis.empty = data;",
                json_file.to_str().unwrap()
            ),
        )
        .unwrap();

        jstime.import(test_file.to_str().unwrap()).unwrap();

        let result = jstime.run_script("typeof globalThis.empty", "test");
        assert_eq!(result.unwrap(), "object");

        let result = jstime.run_script("Object.keys(globalThis.empty).length", "test");
        assert_eq!(result.unwrap(), "0");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&json_file).ok();
    }

    #[test]
    fn json_module_empty_array() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let json_file = temp_dir.join("empty_array.json");
        let test_file = temp_dir.join("test_empty_array.js");

        std::fs::write(&json_file, r#"{"arr": []}"#).unwrap();
        std::fs::write(
            &test_file,
            format!(
                "import data from '{}';\nglobalThis.emptyArr = data.arr;",
                json_file.to_str().unwrap()
            ),
        )
        .unwrap();

        jstime.import(test_file.to_str().unwrap()).unwrap();

        let result = jstime.run_script("Array.isArray(globalThis.emptyArr)", "test");
        assert_eq!(result.unwrap(), "true");

        let result = jstime.run_script("globalThis.emptyArr.length", "test");
        assert_eq!(result.unwrap(), "0");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&json_file).ok();
    }

    #[test]
    fn json_module_special_characters_in_strings() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let json_file = temp_dir.join("special_chars.json");
        let test_file = temp_dir.join("test_special_chars.js");

        std::fs::write(
            &json_file,
            r#"{"text": "Hello\nWorld", "quote": "She said \"Hi\""}"#,
        )
        .unwrap();
        std::fs::write(
            &test_file,
            format!(
                "import data from '{}';\nglobalThis.special = data;",
                json_file.to_str().unwrap()
            ),
        )
        .unwrap();

        jstime.import(test_file.to_str().unwrap()).unwrap();

        // Check newline is preserved
        let result = jstime.run_script("globalThis.special.text.includes('\\n')", "test");
        assert_eq!(result.unwrap(), "true");

        // Check quote is preserved
        let result = jstime.run_script("globalThis.special.quote.includes('\"')", "test");
        assert_eq!(result.unwrap(), "true");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&json_file).ok();
    }

    #[test]
    fn json_module_unicode_characters() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let json_file = temp_dir.join("unicode.json");
        let test_file = temp_dir.join("test_unicode.js");

        std::fs::write(&json_file, r#"{"emoji": "🎉", "chinese": "你好"}"#).unwrap();
        std::fs::write(
            &test_file,
            format!(
                "import data from '{}';\nglobalThis.unicode = data;",
                json_file.to_str().unwrap()
            ),
        )
        .unwrap();

        jstime.import(test_file.to_str().unwrap()).unwrap();

        let result = jstime.run_script("globalThis.unicode.emoji", "test");
        assert_eq!(result.unwrap(), "🎉");

        let result = jstime.run_script("globalThis.unicode.chinese", "test");
        assert_eq!(result.unwrap(), "你好");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&json_file).ok();
    }

    #[test]
    fn json_module_numeric_edge_cases() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let json_file = temp_dir.join("numbers.json");
        let test_file = temp_dir.join("test_numbers.js");

        std::fs::write(
            &json_file,
            r#"{"zero": 0, "negative": -42, "decimal": 3.14, "exponential": 1e10}"#,
        )
        .unwrap();
        std::fs::write(
            &test_file,
            format!(
                "import data from '{}';\nglobalThis.numbers = data;",
                json_file.to_str().unwrap()
            ),
        )
        .unwrap();

        jstime.import(test_file.to_str().unwrap()).unwrap();

        let result = jstime.run_script("globalThis.numbers.zero", "test");
        assert_eq!(result.unwrap(), "0");

        let result = jstime.run_script("globalThis.numbers.negative", "test");
        assert_eq!(result.unwrap(), "-42");

        let result = jstime.run_script("globalThis.numbers.decimal", "test");
        assert_eq!(result.unwrap(), "3.14");

        let result = jstime.run_script("globalThis.numbers.exponential", "test");
        assert_eq!(result.unwrap(), "10000000000");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&json_file).ok();
    }

    #[test]
    fn json_module_mixed_array_types() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let json_file = temp_dir.join("mixed_array.json");
        let test_file = temp_dir.join("test_mixed_array.js");

        std::fs::write(
            &json_file,
            r#"{"mixed": [1, "two", true, null, {"nested": "object"}]}"#,
        )
        .unwrap();
        std::fs::write(
            &test_file,
            format!(
                "import data from '{}';\nglobalThis.mixed = data.mixed;",
                json_file.to_str().unwrap()
            ),
        )
        .unwrap();

        jstime.import(test_file.to_str().unwrap()).unwrap();

        let result = jstime.run_script("globalThis.mixed[0]", "test");
        assert_eq!(result.unwrap(), "1");

        let result = jstime.run_script("globalThis.mixed[1]", "test");
        assert_eq!(result.unwrap(), "two");

        let result = jstime.run_script("globalThis.mixed[2]", "test");
        assert_eq!(result.unwrap(), "true");

        let result = jstime.run_script("globalThis.mixed[3] === null", "test");
        assert_eq!(result.unwrap(), "true");

        let result = jstime.run_script("globalThis.mixed[4].nested", "test");
        assert_eq!(result.unwrap(), "object");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&json_file).ok();
    }

    #[test]
    fn json_module_relative_path_resolution() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let json_file = temp_dir.join("relative.json");
        let test_file = temp_dir.join("test_relative.js");

        std::fs::write(&json_file, r#"{"relative": true}"#).unwrap();
        std::fs::write(
            &test_file,
            "import data from './relative.json';\nglobalThis.relativeData = data;",
        )
        .unwrap();

        jstime.import(test_file.to_str().unwrap()).unwrap();

        let result = jstime.run_script("globalThis.relativeData.relative", "test");
        assert_eq!(result.unwrap(), "true");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&json_file).ok();
    }
}
