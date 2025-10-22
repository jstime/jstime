// Base64 Encoding/Decoding Conformance Tests
// Based on https://html.spec.whatwg.org/multipage/webappapis.html#atob

use jstime_core as jstime;

mod common;

#[cfg(test)]
mod conformance_base64 {
    use super::*;

    // Test that btoa exists as a global function
    #[test]
    fn btoa_exists_on_global() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("globalThis.hasOwnProperty('btoa');", "test");
        assert_eq!(result.unwrap(), "true");
    }

    // Test that atob exists as a global function
    #[test]
    fn atob_exists_on_global() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("globalThis.hasOwnProperty('atob');", "test");
        assert_eq!(result.unwrap(), "true");
    }

    // Test that btoa is a function
    #[test]
    fn btoa_is_function() {
        let result = common::get_type_of("btoa");
        assert_eq!(result.unwrap(), "function");
    }

    // Test that atob is a function
    #[test]
    fn atob_is_function() {
        let result = common::get_type_of("atob");
        assert_eq!(result.unwrap(), "function");
    }

    // Test btoa with empty string
    #[test]
    fn btoa_empty_string() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("btoa('');", "test");
        assert_eq!(result.unwrap(), "");
    }

    // Test atob with empty string
    #[test]
    fn atob_empty_string() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("atob('');", "test");
        assert_eq!(result.unwrap(), "");
    }

    // Test btoa with ASCII text
    #[test]
    fn btoa_ascii_text() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("btoa('hello');", "test");
        assert_eq!(result.unwrap(), "aGVsbG8=");
    }

    // Test atob with valid base64
    #[test]
    fn atob_valid_base64() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("atob('aGVsbG8=');", "test");
        assert_eq!(result.unwrap(), "hello");
    }

    // Test round-trip encoding/decoding
    #[test]
    fn btoa_atob_round_trip() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("atob(btoa('Hello, World!'));", "test");
        assert_eq!(result.unwrap(), "Hello, World!");
    }

    // Test btoa with special characters in Latin-1 range
    #[test]
    fn btoa_special_characters() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("btoa('\\n\\t\\r');", "test");
        assert_eq!(result.unwrap(), "CgkN");
    }

    // Test btoa with null byte
    #[test]
    fn btoa_null_byte() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("btoa('\\x00');", "test");
        assert_eq!(result.unwrap(), "AA==");
    }

    // Test btoa with Latin-1 boundary (character code 255)
    #[test]
    fn btoa_latin1_max_character() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("btoa('\\xFF');", "test");
        assert_eq!(result.unwrap(), "/w==");
    }

    // Test btoa throws on character outside Latin-1 range
    #[test]
    fn btoa_throws_on_unicode() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "try { btoa('\\u0100'); 'no error'; } catch(e) { 'error'; }",
            "test",
        );
        assert_eq!(result.unwrap(), "error");
    }

    // Test btoa throws on emoji
    #[test]
    fn btoa_throws_on_emoji() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "try { btoa('😀'); 'no error'; } catch(e) { 'error'; }",
            "test",
        );
        assert_eq!(result.unwrap(), "error");
    }

    // Test btoa throws on Chinese characters
    #[test]
    fn btoa_throws_on_chinese() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "try { btoa('世界'); 'no error'; } catch(e) { 'error'; }",
            "test",
        );
        assert_eq!(result.unwrap(), "error");
    }

    // Test btoa requires at least one argument
    #[test]
    fn btoa_requires_argument() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("try { btoa(); 'no error'; } catch(e) { 'error'; }", "test");
        assert_eq!(result.unwrap(), "error");
    }

    // Test atob requires at least one argument
    #[test]
    fn atob_requires_argument() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("try { atob(); 'no error'; } catch(e) { 'error'; }", "test");
        assert_eq!(result.unwrap(), "error");
    }

    // Test atob throws on invalid base64 character
    #[test]
    fn atob_throws_on_invalid_character() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "try { atob('!!!'); 'no error'; } catch(e) { 'error'; }",
            "test",
        );
        assert_eq!(result.unwrap(), "error");
    }

    // Test atob throws on invalid base64 length
    #[test]
    fn atob_throws_on_invalid_length() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "try { atob('abc'); 'no error'; } catch(e) { 'error'; }",
            "test",
        );
        assert_eq!(result.unwrap(), "error");
    }

    // Test atob with padding
    #[test]
    fn atob_with_single_padding() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("atob('YQ==');", "test");
        assert_eq!(result.unwrap(), "a");
    }

    // Test atob with double padding
    #[test]
    fn atob_with_double_padding() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("atob('YWI=');", "test");
        assert_eq!(result.unwrap(), "ab");
    }

    // Test atob with no padding
    #[test]
    fn atob_without_padding() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("atob('YWJj');", "test");
        assert_eq!(result.unwrap(), "abc");
    }

    // Test atob ignores whitespace (per spec)
    #[test]
    fn atob_ignores_whitespace() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("atob('Y W Jj');", "test");
        assert_eq!(result.unwrap(), "abc");
    }

    // Test btoa converts to string
    #[test]
    fn btoa_converts_to_string() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("btoa(123);", "test");
        assert_eq!(result.unwrap(), "MTIz");
    }

    // Test atob converts to string
    #[test]
    fn atob_converts_to_string() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let _result = jstime.run_script("atob(123);", "test");
        // This should throw due to invalid base64 length
        let result = jstime.run_script(
            "try { atob(123); 'no error'; } catch(e) { 'error'; }",
            "test",
        );
        assert_eq!(result.unwrap(), "error");
    }

    // Test btoa with all ASCII printable characters
    #[test]
    fn btoa_ascii_printable() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("btoa('ABC123xyz');", "test");
        assert_eq!(result.unwrap(), "QUJDMTIzeHl6");
    }

    // Test atob decodes to correct values
    #[test]
    fn atob_decodes_correctly() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("atob('QUJDMTIzeHl6');", "test");
        assert_eq!(result.unwrap(), "ABC123xyz");
    }

    // Test btoa with all Latin-1 characters (0-255)
    #[test]
    fn btoa_all_latin1_characters() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        // Test a sampling of Latin-1 characters
        let result = jstime.run_script("btoa('\\x00\\x01\\x7F\\x80\\xFF');", "test");
        assert_eq!(result.unwrap(), "AAF/gP8=");
    }

    // Test atob returns Latin-1 characters correctly
    #[test]
    fn atob_returns_latin1_characters() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const result = atob('AAF/gP8='); \
             result.charCodeAt(0) === 0 && \
             result.charCodeAt(1) === 1 && \
             result.charCodeAt(2) === 127 && \
             result.charCodeAt(3) === 128 && \
             result.charCodeAt(4) === 255;",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }
}
