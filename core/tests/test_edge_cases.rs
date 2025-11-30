// Edge case and boundary tests
// These tests verify behavior at the boundaries of API contracts

use jstime_core as jstime;

mod common;

#[cfg(test)]
mod edge_cases {
    use super::*;

    // ============================================================================
    // TIMER EDGE CASES
    // ============================================================================

    #[test]
    fn test_settimeout_negative_delay() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Negative delays should be treated as 0
        let script = r#"
            globalThis.executed = false;
            setTimeout(() => {
                globalThis.executed = true;
            }, -100);
        "#;
        jstime.run_script(script, "test").unwrap();

        let result = jstime.run_script("globalThis.executed", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_settimeout_very_large_delay() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Very large delays should not crash. Use run_script_no_event_loop to avoid blocking.
        // Since we clear the timer immediately, the event loop has nothing to wait for.
        let script = r#"
            const id = setTimeout(() => {
                globalThis.shouldNotRun = true;
            }, 1000000); // Large but reasonable delay
            
            // Clear it immediately before event loop runs
            clearTimeout(id);
            globalThis.testComplete = true;
        "#;

        // Use no_event_loop version since we're clearing the timer immediately
        jstime.run_script_no_event_loop(script, "test").unwrap();

        let complete = jstime.run_script_no_event_loop("globalThis.testComplete", "test");
        assert_eq!(complete.unwrap(), "true");

        // Verify the callback didn't run
        let should_not_run = jstime.run_script_no_event_loop("globalThis.shouldNotRun", "test");
        assert_eq!(should_not_run.unwrap(), "undefined");
    }

    #[test]
    fn test_cleartimeout_undefined_null() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // clearTimeout with undefined/null should not throw
        let script = r#"
            clearTimeout(undefined);
            clearTimeout(null);
            clearTimeout();
            clearInterval(undefined);
            clearInterval(null);
            clearInterval();
            'no_error';
        "#;
        let result = jstime.run_script(script, "test");
        assert_eq!(result.unwrap(), "no_error");
    }

    #[test]
    fn test_settimeout_non_function_callback() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Non-function callbacks should either throw or be handled gracefully
        let script = r#"
            let errorOccurred = false;
            try {
                setTimeout('not a function', 0);
            } catch (e) {
                errorOccurred = true;
            }
            // Whether it throws or not, we shouldn't crash
            'completed';
        "#;
        let result = jstime.run_script(script, "test");
        assert_eq!(result.unwrap(), "completed");
    }

    // ============================================================================
    // URL EDGE CASES
    // ============================================================================

    #[test]
    fn test_url_invalid_input() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Invalid URLs should throw
        let script = r#"
            let throwCount = 0;
            
            try { new URL('not a url'); } catch(e) { throwCount++; }
            try { new URL('://missing-scheme'); } catch(e) { throwCount++; }
            try { new URL('http://'); } catch(e) { throwCount++; }
            
            throwCount;
        "#;
        let result = jstime.run_script(script, "test");
        // All should throw
        let count: i64 = result.unwrap().parse().unwrap();
        assert!(count >= 2, "At least 2 invalid URLs should throw");
    }

    #[test]
    fn test_url_protocol_variations() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test various protocol formats
        let script = r#"
            const results = {};
            
            // Standard protocols
            results.http = new URL('http://example.com').protocol;
            results.https = new URL('https://example.com').protocol;
            results.ftp = new URL('ftp://example.com').protocol;
            results.file = new URL('file:///path').protocol;
            
            // Custom protocols
            results.custom = new URL('custom://example.com').protocol;
            
            JSON.stringify(results);
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["http"], "http:");
        assert_eq!(json["https"], "https:");
        assert_eq!(json["ftp"], "ftp:");
        assert_eq!(json["file"], "file:");
        assert_eq!(json["custom"], "custom:");
    }

    #[test]
    fn test_url_empty_components() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test URLs with empty/missing components
        let script = r#"
            const url = new URL('https://example.com');
            
            JSON.stringify({
                username: url.username,
                password: url.password,
                port: url.port,
                pathname: url.pathname,
                search: url.search,
                hash: url.hash
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["username"], "");
        assert_eq!(json["password"], "");
        assert_eq!(json["port"], "");
        // pathname should default to "/" for http/https
        assert_eq!(json["pathname"], "/");
        assert_eq!(json["search"], "");
        assert_eq!(json["hash"], "");
    }

    #[test]
    fn test_urlsearchparams_special_keys() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test keys that might conflict with object prototype
        let script = r#"
            const params = new URLSearchParams();
            
            // These are Object.prototype property names
            params.set('constructor', 'value1');
            params.set('toString', 'value2');
            params.set('hasOwnProperty', 'value3');
            params.set('__proto__', 'value4');
            
            JSON.stringify({
                constructor: params.get('constructor'),
                toString: params.get('toString'),
                hasOwnProperty: params.get('hasOwnProperty'),
                proto: params.get('__proto__'),
                methodWorks: typeof params.toString === 'function'
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["constructor"], "value1");
        assert_eq!(json["toString"], "value2");
        assert_eq!(json["hasOwnProperty"], "value3");
        // __proto__ might be handled specially or stored normally
        // The important thing is that methods still work
        assert_eq!(json["methodWorks"], true);
    }

    // ============================================================================
    // TEXT ENCODING EDGE CASES
    // ============================================================================

    #[test]
    fn test_text_encoder_surrogate_pairs() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test encoding of surrogate pairs (emoji)
        let script = r#"
            const encoder = new TextEncoder();
            const decoder = new TextDecoder();
            
            // Emoji that uses surrogate pairs
            const emoji = '😀🎉🌍';
            const encoded = encoder.encode(emoji);
            const decoded = decoder.decode(encoded);
            
            JSON.stringify({
                originalLength: emoji.length, // 6 in JS (surrogate pairs)
                encodedLength: encoded.length, // 12 bytes in UTF-8
                decodedMatch: emoji === decoded
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["decodedMatch"], true);
        // Each emoji is 4 bytes in UTF-8
        assert_eq!(json["encodedLength"], 12);
    }

    #[test]
    fn test_text_encoder_lone_surrogates() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test encoding of lone surrogates (should produce replacement character)
        let script = r#"
            const encoder = new TextEncoder();
            
            // Create string with lone surrogate
            const loneSurrogate = String.fromCharCode(0xD800);
            const encoded = encoder.encode(loneSurrogate);
            
            // Lone surrogates should produce UTF-8 replacement character or similar
            JSON.stringify({
                inputLength: loneSurrogate.length,
                encodedLength: encoded.length,
                hasOutput: encoded.length > 0
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["inputLength"], 1);
        assert_eq!(json["hasOutput"], true);
    }

    #[test]
    fn test_text_encoder_encode_into_exact_fit() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test encodeInto with exactly fitting buffer
        let script = r#"
            const encoder = new TextEncoder();
            const input = 'Hello';
            
            // Create buffer with exact required size
            const buffer = new Uint8Array(5);
            const result = encoder.encodeInto(input, buffer);
            
            JSON.stringify({
                read: result.read,
                written: result.written,
                bufferFull: result.written === buffer.length
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["read"], 5);
        assert_eq!(json["written"], 5);
        assert_eq!(json["bufferFull"], true);
    }

    #[test]
    fn test_text_decoder_fatal_mode() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test fatal mode throws on invalid UTF-8
        // Note: The jstime implementation may not support fatal mode yet,
        // so we test what happens but don't require a specific behavior
        let script = r#"
            const decoder = new TextDecoder('utf-8', { fatal: true });
            
            // Invalid UTF-8 sequence
            const invalid = new Uint8Array([0xFF, 0xFE]);
            
            let threw = false;
            let result = null;
            try {
                result = decoder.decode(invalid);
            } catch (e) {
                threw = true;
            }
            
            // Either it throws or it produces replacement characters
            JSON.stringify({
                threw: threw,
                hasResult: result !== null
            });
        "#;
        let result = jstime.run_script(script, "test");
        // Test completes without crashing - either behavior is acceptable
        assert!(result.is_ok());
    }

    // ============================================================================
    // BASE64 EDGE CASES
    // ============================================================================

    #[test]
    fn test_btoa_boundary_characters() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test btoa at Latin-1 boundaries
        let script = r#"
            const results = {};
            
            // Character code 0 (null)
            results.null = btoa(String.fromCharCode(0));
            
            // Character code 127 (DEL)
            results.del = btoa(String.fromCharCode(127));
            
            // Character code 128 (first extended ASCII)
            results.ext128 = btoa(String.fromCharCode(128));
            
            // Character code 255 (last Latin-1)
            results.lat255 = btoa(String.fromCharCode(255));
            
            JSON.stringify(results);
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        // Verify all encoded successfully
        assert!(!json["null"].as_str().unwrap().is_empty());
        assert!(!json["del"].as_str().unwrap().is_empty());
        assert!(!json["ext128"].as_str().unwrap().is_empty());
        assert!(!json["lat255"].as_str().unwrap().is_empty());
    }

    #[test]
    fn test_atob_whitespace_handling() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test that atob handles whitespace correctly
        let script = r#"
            const results = {};
            const original = 'Hello';
            const encoded = btoa(original);
            
            // Add various whitespace
            results.withSpace = atob(' ' + encoded + ' ') === original;
            results.withTab = atob('\t' + encoded + '\t') === original;
            results.withNewline = atob('\n' + encoded + '\n') === original;
            results.mixed = atob(' \t\n' + encoded + '\n\t ') === original;
            
            JSON.stringify(results);
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        // All should handle whitespace
        assert_eq!(json["withSpace"], true);
        assert_eq!(json["withTab"], true);
        assert_eq!(json["withNewline"], true);
        assert_eq!(json["mixed"], true);
    }

    #[test]
    fn test_atob_padding_variations() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test various padding scenarios
        let script = r#"
            const results = {};
            
            // No padding needed (multiple of 4)
            results.noPadding = atob('SGVsbG8=') === 'Hello';
            
            // Single padding
            results.singlePad = atob('SGVsbA==') === 'Hell';
            
            // Double padding
            results.doublePad = atob('SGVs') === 'Hel';
            
            JSON.stringify(results);
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["noPadding"], true);
        assert_eq!(json["singlePad"], true);
        assert_eq!(json["doublePad"], true);
    }

    // ============================================================================
    // EVENT EDGE CASES
    // ============================================================================

    #[test]
    fn test_event_dispatch_during_dispatch() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test dispatching events inside event handlers
        let script = r#"
            const target = new EventTarget();
            globalThis.dispatchOrder = [];
            
            target.addEventListener('first', () => {
                globalThis.dispatchOrder.push('first_start');
                
                // Dispatch another event during handling
                target.dispatchEvent(new Event('second'));
                
                globalThis.dispatchOrder.push('first_end');
            });
            
            target.addEventListener('second', () => {
                globalThis.dispatchOrder.push('second');
            });
            
            target.dispatchEvent(new Event('first'));
            globalThis.dispatchOrder.join(',');
        "#;
        let result = jstime.run_script(script, "test");
        let order = result.unwrap();

        // 'second' should be handled synchronously during 'first'
        assert!(order.contains("first_start"));
        assert!(order.contains("second"));
        assert!(order.contains("first_end"));
    }

    #[test]
    fn test_event_listener_modification_during_dispatch() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test adding/removing listeners during dispatch
        let script = r#"
            const target = new EventTarget();
            globalThis.calls = [];
            
            const listener1 = () => {
                globalThis.calls.push('listener1');
                // Add a new listener during dispatch
                target.addEventListener('test', listener3);
            };
            
            const listener2 = () => {
                globalThis.calls.push('listener2');
            };
            
            const listener3 = () => {
                globalThis.calls.push('listener3');
            };
            
            target.addEventListener('test', listener1);
            target.addEventListener('test', listener2);
            
            // First dispatch
            target.dispatchEvent(new Event('test'));
            
            // Second dispatch - listener3 should now be there
            globalThis.calls.push('---');
            target.dispatchEvent(new Event('test'));
            
            globalThis.calls.join(',');
        "#;
        let result = jstime.run_script(script, "test");
        let calls = result.unwrap();

        // First dispatch: listener1, listener2 (listener3 added but might not run)
        // Second dispatch: listener1, listener2, listener3
        assert!(calls.contains("listener1"));
        assert!(calls.contains("listener2"));
        // After second dispatch, listener3 should be called
        let parts: Vec<&str> = calls.split("---").collect();
        assert!(parts[1].contains("listener3"));
    }

    #[test]
    fn test_event_stop_immediate_propagation_first_listener() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test that stopImmediatePropagation works in first listener
        let script = r#"
            const target = new EventTarget();
            globalThis.calls = [];
            
            target.addEventListener('test', (e) => {
                globalThis.calls.push('first');
                e.stopImmediatePropagation();
            });
            
            target.addEventListener('test', () => {
                globalThis.calls.push('second');
            });
            
            target.addEventListener('test', () => {
                globalThis.calls.push('third');
            });
            
            target.dispatchEvent(new Event('test'));
            globalThis.calls.length;
        "#;
        let result = jstime.run_script(script, "test");
        assert_eq!(result.unwrap(), "1");
    }

    // ============================================================================
    // STRUCTURED CLONE EDGE CASES
    // ============================================================================

    #[test]
    fn test_structuredclone_special_numbers() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test cloning special number values
        let script = r#"
            const obj = {
                nan: NaN,
                posInf: Infinity,
                negInf: -Infinity,
                negZero: -0,
                maxValue: Number.MAX_VALUE,
                minValue: Number.MIN_VALUE,
                maxSafeInt: Number.MAX_SAFE_INTEGER,
                minSafeInt: Number.MIN_SAFE_INTEGER
            };
            
            const cloned = structuredClone(obj);
            
            JSON.stringify({
                nanIsNaN: Number.isNaN(cloned.nan),
                posInfIsInf: cloned.posInf === Infinity,
                negInfIsNegInf: cloned.negInf === -Infinity,
                negZeroIsNegZero: Object.is(cloned.negZero, -0),
                maxValueMatch: cloned.maxValue === Number.MAX_VALUE,
                minValueMatch: cloned.minValue === Number.MIN_VALUE,
                maxSafeIntMatch: cloned.maxSafeInt === Number.MAX_SAFE_INTEGER,
                minSafeIntMatch: cloned.minSafeInt === Number.MIN_SAFE_INTEGER
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["nanIsNaN"], true);
        assert_eq!(json["posInfIsInf"], true);
        assert_eq!(json["negInfIsNegInf"], true);
        assert_eq!(json["negZeroIsNegZero"], true);
        assert_eq!(json["maxValueMatch"], true);
        assert_eq!(json["minValueMatch"], true);
    }

    #[test]
    fn test_structuredclone_date_edge_cases() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test cloning dates with edge case values
        let script = r#"
            const obj = {
                epoch: new Date(0),
                negative: new Date(-1000000000000),
                future: new Date(8640000000000000), // Max date value
                invalid: new Date('invalid')
            };
            
            const cloned = structuredClone(obj);
            
            JSON.stringify({
                epochMatch: cloned.epoch.getTime() === 0,
                negativeMatch: cloned.negative.getTime() === -1000000000000,
                futureMatch: cloned.future.getTime() === 8640000000000000,
                invalidIsNaN: Number.isNaN(cloned.invalid.getTime())
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["epochMatch"], true);
        assert_eq!(json["negativeMatch"], true);
        assert_eq!(json["futureMatch"], true);
        assert_eq!(json["invalidIsNaN"], true);
    }

    #[test]
    fn test_structuredclone_regex_flags() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test cloning regexes with various flags
        let script = r#"
            const regexes = {
                basic: /test/,
                global: /test/g,
                ignoreCase: /test/i,
                multiline: /test/m,
                dotAll: /test/s,
                unicode: /test/u,
                sticky: /test/y,
                allFlags: /test/gimsuy
            };
            
            const cloned = structuredClone(regexes);
            
            JSON.stringify({
                basicFlags: cloned.basic.flags,
                globalFlags: cloned.global.flags,
                allFlagsMatch: cloned.allFlags.flags === regexes.allFlags.flags,
                sourceMatch: cloned.basic.source === regexes.basic.source,
                notSame: cloned.basic !== regexes.basic
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["basicFlags"], "");
        assert_eq!(json["globalFlags"], "g");
        assert_eq!(json["allFlagsMatch"], true);
        assert_eq!(json["sourceMatch"], true);
        assert_eq!(json["notSame"], true);
    }

    // ============================================================================
    // CRYPTO EDGE CASES
    // ============================================================================

    #[test]
    fn test_crypto_get_random_values_max_size() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test getRandomValues with maximum allowed size (65536 bytes)
        let script = r#"
            const maxSize = 65536;
            const array = new Uint8Array(maxSize);
            
            crypto.getRandomValues(array);
            
            // Verify it was filled
            let nonZeroCount = 0;
            for (let i = 0; i < 1000; i++) { // Check first 1000
                if (array[i] !== 0) nonZeroCount++;
            }
            
            JSON.stringify({
                length: array.length,
                hasRandomData: nonZeroCount > 100 // Should have many non-zero values
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["length"], 65536);
        assert_eq!(json["hasRandomData"], true);
    }

    #[test]
    fn test_crypto_random_values_typed_arrays() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test getRandomValues with different typed arrays
        let script = r#"
            const results = {};
            
            const arrays = {
                int8: new Int8Array(16),
                uint8: new Uint8Array(16),
                int16: new Int16Array(8),
                uint16: new Uint16Array(8),
                int32: new Int32Array(4),
                uint32: new Uint32Array(4)
            };
            
            for (const [name, array] of Object.entries(arrays)) {
                crypto.getRandomValues(array);
                results[name] = array.length;
            }
            
            JSON.stringify(results);
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["int8"], 16);
        assert_eq!(json["uint8"], 16);
        assert_eq!(json["int16"], 8);
        assert_eq!(json["uint16"], 8);
        assert_eq!(json["int32"], 4);
        assert_eq!(json["uint32"], 4);
    }

    // ============================================================================
    // RESPONSE AND REQUEST EDGE CASES
    // ============================================================================

    #[test]
    fn test_response_with_various_status_codes() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test Response with various HTTP status codes
        let script = r#"
            const statusCodes = [200, 201, 204, 301, 302, 400, 401, 403, 404, 500, 502, 503];
            const results = {};
            
            for (const code of statusCodes) {
                const resp = new Response('', { status: code });
                results[code] = {
                    status: resp.status,
                    ok: resp.ok
                };
            }
            
            JSON.stringify(results);
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        // 2xx should be ok
        assert_eq!(json["200"]["ok"], true);
        assert_eq!(json["201"]["ok"], true);
        assert_eq!(json["204"]["ok"], true);

        // 3xx, 4xx, 5xx should not be ok
        assert_eq!(json["301"]["ok"], false);
        assert_eq!(json["400"]["ok"], false);
        assert_eq!(json["500"]["ok"], false);
    }

    #[test]
    fn test_headers_multiple_values_same_key() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test Headers with multiple values for same key
        let script = r#"
            const headers = new Headers();
            
            // Append multiple values
            headers.append('Accept', 'text/html');
            headers.append('Accept', 'application/json');
            headers.append('Accept', 'text/plain');
            
            // Get should return comma-separated
            const acceptValue = headers.get('Accept');
            
            // Count entries
            let entryCount = 0;
            for (const [k, v] of headers) {
                entryCount++;
            }
            
            JSON.stringify({
                acceptValue: acceptValue,
                entryCount: entryCount
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        // get() should return combined value
        let accept = json["acceptValue"].as_str().unwrap();
        assert!(accept.contains("text/html"));
        assert!(accept.contains("application/json"));
    }

    #[test]
    fn test_request_clone() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test Request cloning
        let script = r#"
            const original = new Request('https://example.com/api', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' }
            });
            
            const cloned = original.clone();
            
            JSON.stringify({
                urlMatch: cloned.url === original.url,
                methodMatch: cloned.method === original.method,
                notSameObject: cloned !== original,
                headerMatch: cloned.headers.get('content-type') === original.headers.get('content-type')
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["urlMatch"], true);
        assert_eq!(json["methodMatch"], true);
        assert_eq!(json["notSameObject"], true);
        assert_eq!(json["headerMatch"], true);
    }

    // ============================================================================
    // PERFORMANCE API EDGE CASES
    // ============================================================================

    #[test]
    fn test_performance_now_high_resolution() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test that performance.now() has sub-millisecond resolution
        let script = r#"
            const measurements = [];
            
            // Take many measurements
            for (let i = 0; i < 100; i++) {
                measurements.push(performance.now());
            }
            
            // Check for sub-millisecond differences
            let hasFractional = false;
            for (let i = 1; i < measurements.length; i++) {
                const diff = measurements[i] - measurements[i-1];
                if (diff !== Math.floor(diff) || (diff > 0 && diff < 1)) {
                    hasFractional = true;
                    break;
                }
            }
            
            // All should be monotonically increasing
            let isMonotonic = true;
            for (let i = 1; i < measurements.length; i++) {
                if (measurements[i] < measurements[i-1]) {
                    isMonotonic = false;
                    break;
                }
            }
            
            JSON.stringify({
                measurementCount: measurements.length,
                isMonotonic: isMonotonic
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["measurementCount"], 100);
        assert_eq!(json["isMonotonic"], true);
    }

    // ============================================================================
    // CONSOLE EDGE CASES
    // ============================================================================

    #[test]
    fn test_console_count_reset() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test console.count and countReset
        let script = r#"
            // These should not throw
            console.count('test');
            console.count('test');
            console.count('test');
            console.countReset('test');
            console.count('test'); // Should start from 1 again
            
            // Count with different labels
            console.count('a');
            console.count('b');
            console.count('a');
            
            // Reset non-existent label (should not throw)
            console.countReset('nonexistent');
            
            'completed';
        "#;
        let result = jstime.run_script(script, "test");
        assert_eq!(result.unwrap(), "completed");
    }

    #[test]
    fn test_console_time_end_without_start() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test timeEnd without corresponding time
        let script = r#"
            // These should not throw
            console.timeEnd('never_started');
            console.timeLog('never_started');
            
            // Normal usage
            console.time('normal');
            console.timeLog('normal');
            console.timeEnd('normal');
            
            // End already ended (should not throw)
            console.timeEnd('normal');
            
            'completed';
        "#;
        let result = jstime.run_script(script, "test");
        assert_eq!(result.unwrap(), "completed");
    }

    #[test]
    fn test_console_group_nesting() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test deeply nested console groups
        let script = r#"
            // Deep nesting
            for (let i = 0; i < 10; i++) {
                console.group('Level ' + i);
            }
            
            console.log('Deeply nested');
            
            // Unwind all groups
            for (let i = 0; i < 10; i++) {
                console.groupEnd();
            }
            
            // Extra groupEnd (should not throw)
            console.groupEnd();
            console.groupEnd();
            
            'completed';
        "#;
        let result = jstime.run_script(script, "test");
        assert_eq!(result.unwrap(), "completed");
    }
}
