// Text Encoding API Conformance Tests
// Based on https://encoding.spec.whatwg.org/

use jstime_core as jstime;

mod common;

#[cfg(test)]
mod conformance_text_encoding {
    use super::*;

    // ============================================================================
    // TextEncoder Tests
    // ============================================================================

    #[test]
    fn text_encoder_exists_on_global() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("globalThis.hasOwnProperty('TextEncoder');", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn text_encoder_is_constructor() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof TextEncoder", "test");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn text_encoder_can_be_instantiated() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); encoder instanceof TextEncoder",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn text_encoder_encoding_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); encoder.encoding",
            "test",
        );
        assert_eq!(result.unwrap(), "utf-8");
    }

    #[test]
    fn text_encoder_encode_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); typeof encoder.encode",
            "test",
        );
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn text_encoder_encode_empty_string() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); const result = encoder.encode(''); result.length",
            "test",
        );
        assert_eq!(result.unwrap(), "0");
    }

    #[test]
    fn text_encoder_encode_ascii() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             const result = encoder.encode('hello'); \
             Array.from(result).join(',')",
            "test",
        );
        assert_eq!(result.unwrap(), "104,101,108,108,111");
    }

    #[test]
    fn text_encoder_encode_returns_uint8array() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             const result = encoder.encode('test'); \
             result instanceof Uint8Array",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn text_encoder_encode_utf8_multibyte() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             const result = encoder.encode('€'); \
             Array.from(result).join(',')",
            "test",
        );
        // Euro sign (€) is encoded as 0xE2, 0x82, 0xAC in UTF-8
        assert_eq!(result.unwrap(), "226,130,172");
    }

    #[test]
    fn text_encoder_encode_emoji() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             const result = encoder.encode('😀'); \
             Array.from(result).join(',')",
            "test",
        );
        // Grinning face emoji (😀) is encoded as 0xF0, 0x9F, 0x98, 0x80 in UTF-8
        assert_eq!(result.unwrap(), "240,159,152,128");
    }

    #[test]
    fn text_encoder_encode_mixed_content() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             const result = encoder.encode('Hello 世界 🌍'); \
             result.length",
            "test",
        );
        // "Hello " = 6 bytes
        // "世" = 3 bytes (0xE4, 0xB8, 0x96)
        // "界" = 3 bytes (0xE7, 0x95, 0x8C)
        // " " = 1 byte
        // "🌍" = 4 bytes (0xF0, 0x9F, 0x8C, 0x8D)
        // Total = 17 bytes
        assert_eq!(result.unwrap(), "17");
    }

    #[test]
    fn text_encoder_encode_without_argument() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             const result = encoder.encode(); \
             result.length",
            "test",
        );
        // Should encode '' (empty string) due to default parameter
        assert_eq!(result.unwrap(), "0");
    }

    #[test]
    fn text_encoder_encode_converts_to_string() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             const result = encoder.encode(123); \
             Array.from(result).join(',')",
            "test",
        );
        // Should encode '123'
        assert_eq!(result.unwrap(), "49,50,51");
    }

    // ============================================================================
    // TextEncoder.encodeInto Tests
    // ============================================================================

    #[test]
    fn text_encoder_encode_into_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             typeof encoder.encodeInto",
            "test",
        );
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn text_encoder_encode_into_basic() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             const dest = new Uint8Array(10); \
             const result = encoder.encodeInto('hello', dest); \
             JSON.stringify(result)",
            "test",
        );
        assert_eq!(result.unwrap(), r#"{"read":5,"written":5}"#);
    }

    #[test]
    fn text_encoder_encode_into_writes_to_buffer() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             const dest = new Uint8Array(10); \
             encoder.encodeInto('hi', dest); \
             Array.from(dest.slice(0, 2)).join(',')",
            "test",
        );
        assert_eq!(result.unwrap(), "104,105");
    }

    #[test]
    fn text_encoder_encode_into_partial_write() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             const dest = new Uint8Array(3); \
             const result = encoder.encodeInto('hello', dest); \
             JSON.stringify(result)",
            "test",
        );
        // Should write 'hel' (3 chars, 3 bytes) and report read: 3, written: 3
        assert_eq!(result.unwrap(), r#"{"read":3,"written":3}"#);
    }

    #[test]
    fn text_encoder_encode_into_multibyte_cutoff() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             const dest = new Uint8Array(4); \
             const result = encoder.encodeInto('a€', dest); \
             JSON.stringify(result)",
            "test",
        );
        // 'a' = 1 byte, '€' = 3 bytes, total 4 bytes
        // Should write both and report read: 2, written: 4
        assert_eq!(result.unwrap(), r#"{"read":2,"written":4}"#);
    }

    #[test]
    fn text_encoder_encode_into_requires_uint8array() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             try { \
               encoder.encodeInto('test', {}); \
               'no error'; \
             } catch(e) { \
               'error'; \
             }",
            "test",
        );
        assert_eq!(result.unwrap(), "error");
    }

    // ============================================================================
    // TextDecoder Tests
    // ============================================================================

    #[test]
    fn text_decoder_exists_on_global() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("globalThis.hasOwnProperty('TextDecoder');", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn text_decoder_is_constructor() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof TextDecoder", "test");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn text_decoder_can_be_instantiated() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const decoder = new TextDecoder(); \
             decoder instanceof TextDecoder",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn text_decoder_default_encoding() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const decoder = new TextDecoder(); \
             decoder.encoding",
            "test",
        );
        assert_eq!(result.unwrap(), "utf-8");
    }

    #[test]
    fn text_decoder_explicit_utf8_encoding() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const decoder = new TextDecoder('utf-8'); \
             decoder.encoding",
            "test",
        );
        assert_eq!(result.unwrap(), "utf-8");
    }

    #[test]
    fn text_decoder_utf8_alias() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const decoder = new TextDecoder('utf8'); \
             decoder.encoding",
            "test",
        );
        assert_eq!(result.unwrap(), "utf-8");
    }

    #[test]
    fn text_decoder_invalid_encoding_throws() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "try { \
               new TextDecoder('invalid-encoding'); \
               'no error'; \
             } catch(e) { \
               'error'; \
             }",
            "test",
        );
        assert_eq!(result.unwrap(), "error");
    }

    #[test]
    fn text_decoder_decode_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const decoder = new TextDecoder(); \
             typeof decoder.decode",
            "test",
        );
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn text_decoder_decode_empty() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const decoder = new TextDecoder(); \
             const result = decoder.decode(new Uint8Array([])); \
             result",
            "test",
        );
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn text_decoder_decode_ascii() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const decoder = new TextDecoder(); \
             const data = new Uint8Array([104, 101, 108, 108, 111]); \
             decoder.decode(data)",
            "test",
        );
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn text_decoder_decode_utf8_multibyte() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const decoder = new TextDecoder(); \
             const data = new Uint8Array([226, 130, 172]); \
             decoder.decode(data)",
            "test",
        );
        // Euro sign (€)
        assert_eq!(result.unwrap(), "€");
    }

    #[test]
    fn text_decoder_decode_emoji() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const decoder = new TextDecoder(); \
             const data = new Uint8Array([240, 159, 152, 128]); \
             decoder.decode(data)",
            "test",
        );
        // Grinning face emoji (😀)
        assert_eq!(result.unwrap(), "😀");
    }

    #[test]
    fn text_decoder_decode_chinese() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const decoder = new TextDecoder(); \
             const data = new Uint8Array([228, 184, 150, 231, 149, 140]); \
             decoder.decode(data)",
            "test",
        );
        // 世界 (world in Chinese)
        assert_eq!(result.unwrap(), "世界");
    }

    #[test]
    fn text_decoder_decode_without_argument() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const decoder = new TextDecoder(); \
             const result = decoder.decode(); \
             result",
            "test",
        );
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn text_decoder_decode_null() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const decoder = new TextDecoder(); \
             const result = decoder.decode(null); \
             result",
            "test",
        );
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn text_decoder_decode_arraybuffer() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const decoder = new TextDecoder(); \
             const buffer = new Uint8Array([104, 105]).buffer; \
             decoder.decode(buffer)",
            "test",
        );
        assert_eq!(result.unwrap(), "hi");
    }

    // ============================================================================
    // Round-trip Tests
    // ============================================================================

    #[test]
    fn roundtrip_ascii() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             const decoder = new TextDecoder(); \
             const original = 'Hello, World!'; \
             const encoded = encoder.encode(original); \
             const decoded = decoder.decode(encoded); \
             decoded === original",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn roundtrip_utf8() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             const decoder = new TextDecoder(); \
             const original = 'Hello 世界 🌍 €'; \
             const encoded = encoder.encode(original); \
             const decoded = decoder.decode(encoded); \
             decoded === original",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn roundtrip_emoji() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             const decoder = new TextDecoder(); \
             const original = '😀😃😄😁🤣'; \
             const encoded = encoder.encode(original); \
             const decoded = decoder.decode(encoded); \
             decoded === original",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn roundtrip_special_chars() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const encoder = new TextEncoder(); \
             const decoder = new TextDecoder(); \
             const original = '\\n\\t\\r\\0'; \
             const encoded = encoder.encode(original); \
             const decoded = decoder.decode(encoded); \
             decoded === original",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }
}
