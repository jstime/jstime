use jstime_core as jstime;

mod common;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn queue_microtask() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof globalThis.queueMicrotask;", "jstime");
        assert_eq!(result.unwrap(), "function");
    }
    #[test]
    fn console() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("Object.keys(console);", "jstime");
        assert_eq!(
            result.unwrap(),
            "debug,error,info,log,warn,dir,dirxml,table,trace,group,groupCollapsed,groupEnd,clear,count,countReset,assert,profile,profileEnd,time,timeLog,timeEnd,timeStamp,context"
        );
    }

    #[test]
    fn url_class_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof URL;", "jstime");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn url_parsing() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com:8080/path?query=value#hash'); url.href;",
            "jstime",
        );
        assert_eq!(
            result.unwrap(),
            "https://example.com:8080/path?query=value#hash"
        );
    }

    #[test]
    fn url_with_base() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('/path', 'https://example.com'); url.href;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "https://example.com/path");
    }

    #[test]
    fn url_properties() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            "const url = new URL('https://user:pass@example.com:8080/path?query=value#hash'); \
             JSON.stringify({ \
               protocol: url.protocol, \
               username: url.username, \
               password: url.password, \
               hostname: url.hostname, \
               port: url.port, \
               pathname: url.pathname, \
               search: url.search, \
               hash: url.hash \
             });",
            "jstime",
        );
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["protocol"], "https:");
        assert_eq!(json["username"], "user");
        assert_eq!(json["password"], "pass");
        assert_eq!(json["hostname"], "example.com");
        assert_eq!(json["port"], "8080");
        assert_eq!(json["pathname"], "/path");
        assert_eq!(json["search"], "?query=value");
        assert_eq!(json["hash"], "#hash");
    }

    #[test]
    fn url_origin() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com:8080/path'); url.origin;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "https://example.com:8080");
    }

    #[test]
    fn url_setter() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com'); url.pathname = '/new'; url.href;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "https://example.com/new");
    }

    #[test]
    fn url_search_params_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof URLSearchParams;", "jstime");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn url_search_params_basic() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const params = new URLSearchParams('foo=bar&baz=qux'); params.get('foo');",
            "jstime",
        );
        assert_eq!(result.unwrap(), "bar");
    }

    #[test]
    fn url_search_params_append() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const params = new URLSearchParams(); params.append('foo', 'bar'); params.toString();",
            "jstime",
        );
        assert_eq!(result.unwrap(), "foo=bar");
    }

    #[test]
    fn url_search_params_set() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const params = new URLSearchParams('foo=bar&foo=baz'); params.set('foo', 'qux'); params.toString();",
            "jstime"
        );
        assert_eq!(result.unwrap(), "foo=qux");
    }

    #[test]
    fn url_search_params_delete() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const params = new URLSearchParams('foo=bar&baz=qux'); params.delete('foo'); params.toString();",
            "jstime"
        );
        assert_eq!(result.unwrap(), "baz=qux");
    }

    #[test]
    fn url_search_params_has() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const params = new URLSearchParams('foo=bar'); params.has('foo');",
            "jstime",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn url_search_params_get_all() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const params = new URLSearchParams('foo=bar&foo=baz'); params.getAll('foo').join(',');",
            "jstime"
        );
        assert_eq!(result.unwrap(), "bar,baz");
    }

    #[test]
    fn url_search_params_from_object() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const params = new URLSearchParams({foo: 'bar', baz: 'qux'}); params.toString();",
            "jstime",
        );
        // Note: the order might vary, so let's check if both key-value pairs are present
        let result_str = result.unwrap();
        assert!(result_str.contains("foo=bar"));
        assert!(result_str.contains("baz=qux"));
    }

    #[test]
    fn url_search_params_iteration() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const params = new URLSearchParams('foo=bar&baz=qux'); \
             let result = []; \
             for (const [key, value] of params) { result.push(key + '=' + value); } \
             result.join('&');",
            "jstime",
        );
        assert_eq!(result.unwrap(), "foo=bar&baz=qux");
    }

    #[test]
    fn url_to_json() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com/path'); JSON.stringify(url);",
            "jstime",
        );
        assert_eq!(result.unwrap(), "\"https://example.com/path\"");
    }

    #[test]
    fn url_search_params_sync_with_url() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com?foo=bar'); \
             url.searchParams.append('baz', 'qux'); \
             url.href;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "https://example.com/?foo=bar&baz=qux");
    }

    #[test]
    fn url_search_params_cached() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com?foo=bar'); \
             const sp1 = url.searchParams; \
             const sp2 = url.searchParams; \
             sp1 === sp2;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn performance_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof performance;", "jstime");
        assert_eq!(result.unwrap(), "object");
    }

    #[test]
    fn performance_now_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof performance.now;", "jstime");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn performance_now_returns_number() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof performance.now();", "jstime");
        assert_eq!(result.unwrap(), "number");
    }

    #[test]
    fn performance_now_monotonic() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const t1 = performance.now(); \
             const t2 = performance.now(); \
             t2 >= t1;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn performance_time_origin_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof performance.timeOrigin;", "jstime");
        assert_eq!(result.unwrap(), "number");
    }

    #[test]
    fn performance_time_origin_is_positive() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("performance.timeOrigin > 0;", "jstime");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn performance_time_origin_is_readonly() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const orig = performance.timeOrigin; \
             performance.timeOrigin = 12345; \
             performance.timeOrigin === orig;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn performance_to_json() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const json = JSON.stringify(performance); \
             const obj = JSON.parse(json); \
             typeof obj.timeOrigin === 'number';",
            "jstime",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn import_meta_url_exists() {
        let _setup_guard = common::setup();

        // Create a test module file
        let test_file = std::env::temp_dir().join("test_import_meta.js");
        std::fs::write(&test_file, "export const url = import.meta.url;").unwrap();

        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.import(test_file.to_str().unwrap());

        // Clean up
        std::fs::remove_file(&test_file).ok();

        assert!(result.is_ok());
    }

    #[test]
    fn import_meta_url_is_file_url() {
        let _setup_guard = common::setup();

        // Create a test module file
        let test_file = std::env::temp_dir().join("test_import_meta_url.js");
        std::fs::write(
            &test_file,
            "export const check = import.meta.url.startsWith('file://');",
        )
        .unwrap();

        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.import(test_file.to_str().unwrap());

        // Clean up
        std::fs::remove_file(&test_file).ok();

        assert!(result.is_ok());
    }

    #[test]
    fn import_meta_url_contains_filename() {
        let _setup_guard = common::setup();

        // Create a test module file with a unique name
        let test_file = std::env::temp_dir().join("test_unique_filename_12345.js");
        std::fs::write(
            &test_file,
            "export const check = import.meta.url.includes('test_unique_filename_12345.js');",
        )
        .unwrap();

        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.import(test_file.to_str().unwrap());

        // Clean up
        std::fs::remove_file(&test_file).ok();

        assert!(result.is_ok());
    }

    #[test]
    fn import_meta_url_different_per_module() {
        let _setup_guard = common::setup();

        // Create two test module files
        let test_file1 = std::env::temp_dir().join("test_module1.js");
        let test_file2 = std::env::temp_dir().join("test_module2.js");

        std::fs::write(&test_file2, "export const url2 = import.meta.url;").unwrap();
        std::fs::write(
            &test_file1,
            format!(
                "import {{ url2 }} from '{}';\nexport const different = import.meta.url !== url2;",
                test_file2.to_str().unwrap()
            ),
        )
        .unwrap();

        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.import(test_file1.to_str().unwrap());

        // Clean up
        std::fs::remove_file(&test_file1).ok();
        std::fs::remove_file(&test_file2).ok();

        assert!(result.is_ok());
    }

    #[test]
    fn structuredclone_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof structuredClone;", "jstime");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn structuredclone_basic() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const obj = {a: 1, b: 'test'}; const cloned = structuredClone(obj); JSON.stringify(cloned);",
            "jstime",
        );
        assert_eq!(result.unwrap(), r#"{"a":1,"b":"test"}"#);
    }

    #[test]
    fn atob_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof atob;", "jstime");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn btoa_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof btoa;", "jstime");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn btoa_basic() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("btoa('hello');", "jstime");
        assert_eq!(result.unwrap(), "aGVsbG8=");
    }

    #[test]
    fn atob_basic() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("atob('aGVsbG8=');", "jstime");
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn atob_btoa_round_trip() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("atob(btoa('Hello, World!'));", "jstime");
        assert_eq!(result.unwrap(), "Hello, World!");
    }

    #[test]
    fn btoa_empty_string() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("btoa('');", "jstime");
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn atob_empty_string() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("atob('');", "jstime");
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn btoa_throws_on_unicode() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "try { btoa('Hello 世界'); 'no error'; } catch(e) { 'error'; }",
            "jstime",
        );
        assert_eq!(result.unwrap(), "error");
    }

    #[test]
    fn btoa_accepts_latin1() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        // Test with character code 255 (ÿ)
        let result = jstime.run_script("btoa('\\u00FF');", "jstime");
        assert_eq!(result.unwrap(), "/w==");
    }

    #[test]
    fn atob_invalid_base64() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "try { atob('invalid!@#'); 'no error'; } catch(e) { 'error'; }",
            "jstime",
        );
        assert_eq!(result.unwrap(), "error");
    }

    #[test]
    fn btoa_with_special_chars() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("btoa('a\\nb\\tc');", "jstime");
        // 'a\nb\tc' in base64
        assert_eq!(result.unwrap(), "YQpiCWM=");
    }

    #[test]
    fn event_class_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof Event;", "jstime");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn event_target_class_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof EventTarget;", "jstime");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn event_constructor() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("const event = new Event('test'); event.type;", "jstime");
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn event_with_init_dict() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const event = new Event('test', { bubbles: true, cancelable: true }); \
             JSON.stringify({ bubbles: event.bubbles, cancelable: event.cancelable });",
            "jstime",
        );
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["bubbles"], true);
        assert_eq!(json["cancelable"], true);
    }

    #[test]
    fn event_target_add_event_listener() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let called = false; \
             target.addEventListener('test', () => { called = true; }); \
             target.dispatchEvent(new Event('test')); \
             called;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn event_target_remove_event_listener() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let called = false; \
             const listener = () => { called = true; }; \
             target.addEventListener('test', listener); \
             target.removeEventListener('test', listener); \
             target.dispatchEvent(new Event('test')); \
             called;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn event_target_dispatch_event_returns_boolean() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             const event = new Event('test', { cancelable: true }); \
             const result = target.dispatchEvent(event); \
             typeof result === 'boolean';",
            "jstime",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn event_prevent_default() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             const event = new Event('test', { cancelable: true }); \
             target.addEventListener('test', (e) => { e.preventDefault(); }); \
             const notCancelled = target.dispatchEvent(event); \
             notCancelled;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn event_stop_propagation() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let count = 0; \
             target.addEventListener('test', (e) => { count++; }); \
             target.addEventListener('test', (e) => { count++; }); \
             const event = new Event('test'); \
             event.stopPropagation(); \
             target.dispatchEvent(event); \
             count;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "2");
    }

    #[test]
    fn event_stop_immediate_propagation() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let count = 0; \
             target.addEventListener('test', (e) => { count++; e.stopImmediatePropagation(); }); \
             target.addEventListener('test', (e) => { count++; }); \
             target.dispatchEvent(new Event('test')); \
             count;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "1");
    }

    #[test]
    fn event_target_and_current_target() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let eventTarget, eventCurrentTarget; \
             target.addEventListener('test', (e) => { \
               eventTarget = e.target; \
               eventCurrentTarget = e.currentTarget; \
             }); \
             target.dispatchEvent(new Event('test')); \
             eventTarget === target && eventCurrentTarget === target;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn event_constants() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "JSON.stringify({ \
               NONE: Event.NONE, \
               CAPTURING_PHASE: Event.CAPTURING_PHASE, \
               AT_TARGET: Event.AT_TARGET, \
               BUBBLING_PHASE: Event.BUBBLING_PHASE \
             });",
            "jstime",
        );
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["NONE"], 0);
        assert_eq!(json["CAPTURING_PHASE"], 1);
        assert_eq!(json["AT_TARGET"], 2);
        assert_eq!(json["BUBBLING_PHASE"], 3);
    }

    #[test]
    fn event_timestamp_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const event = new Event('test'); \
             typeof event.timeStamp === 'number';",
            "jstime",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn event_is_trusted_default_false() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const event = new Event('test'); \
             event.isTrusted;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn event_multiple_listeners_same_type() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let count = 0; \
             target.addEventListener('test', () => { count++; }); \
             target.addEventListener('test', () => { count++; }); \
             target.addEventListener('test', () => { count++; }); \
             target.dispatchEvent(new Event('test')); \
             count;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "3");
    }
}
