use jstime_core as jstime;

mod common;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_alloc() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-alloc.js");
        let result = jstime.run_script("globalThis.testBufferAlloc", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_alloc_fill() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-alloc-fill.js");
        let result = jstime.run_script("globalThis.testBufferAllocFill", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_from_string() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-from-string.js");
        let result = jstime.run_script("globalThis.testBufferFromString", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_from_array() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-from-array.js");
        let result = jstime.run_script("globalThis.testBufferFromArray", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_from_hex() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-from-hex.js");
        let result = jstime.run_script("globalThis.testBufferFromHex", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_from_base64() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-from-base64.js");
        let result = jstime.run_script("globalThis.testBufferFromBase64", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_tostring_hex() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-tostring-hex.js");
        let result = jstime.run_script("globalThis.testBufferToStringHex", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_tostring_base64() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-tostring-base64.js");
        let result = jstime.run_script("globalThis.testBufferToStringBase64", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_concat() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-concat.js");
        let result = jstime.run_script("globalThis.testBufferConcat", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_bytelength() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-bytelength.js");
        let result = jstime.run_script("globalThis.testBufferByteLength", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_compare() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-compare.js");
        let result = jstime.run_script("globalThis.testBufferCompare", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_is_encoding() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-isencoding.js");
        let result = jstime.run_script("globalThis.testBufferIsEncoding", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_is_buffer() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-isbuffer.js");
        let result = jstime.run_script("globalThis.testBufferIsBuffer", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_read_write() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-readwrite.js");
        let result = jstime.run_script("globalThis.testBufferReadWrite", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_copy() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-copy.js");
        let result = jstime.run_script("globalThis.testBufferCopy", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_fill() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-fill.js");
        let result = jstime.run_script("globalThis.testBufferFill", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_indexof() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-indexof.js");
        let result = jstime.run_script("globalThis.testBufferIndexOf", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_module_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-module-exists.js");
        let result = jstime.run_script("globalThis.testBufferModuleExists", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_named_imports() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-named-imports.js");
        let result = jstime.run_script("globalThis.testNamedImports", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_buffer_tojson() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/buffer/test-tojson.js");
        let result = jstime.run_script("globalThis.testBufferToJSON", "test");
        assert_eq!(result.unwrap(), "true");
    }
}
