// Fetch API Conformance Tests
// Based on https://fetch.spec.whatwg.org/

use jstime_core as jstime;

mod common;

#[cfg(test)]
mod conformance_fetch {
    use super::*;

    #[test]
    fn fetch_exists() {
        let result = common::get_type_of("fetch");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn headers_constructor_exists() {
        let result = common::get_type_of("Headers");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn request_constructor_exists() {
        let result = common::get_type_of("Request");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn response_constructor_exists() {
        let result = common::get_type_of("Response");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn fetch_returns_promise() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const p = fetch('https://example.com'); p instanceof Promise;",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn headers_can_be_instantiated() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("new Headers() instanceof Headers;", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn headers_has_append_method() {
        let result = common::get_type_of("Headers.prototype.append");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn headers_has_delete_method() {
        let result = common::get_type_of("Headers.prototype.delete");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn headers_has_get_method() {
        let result = common::get_type_of("Headers.prototype.get");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn headers_has_has_method() {
        let result = common::get_type_of("Headers.prototype.has");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn headers_has_set_method() {
        let result = common::get_type_of("Headers.prototype.set");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn headers_append_works() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const h = new Headers(); h.append('Content-Type', 'text/plain'); h.get('content-type');",
            "test",
        );
        assert_eq!(result.unwrap(), "text/plain");
    }

    #[test]
    fn headers_is_case_insensitive() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const h = new Headers(); \
             h.append('Content-Type', 'text/plain'); \
             h.get('content-type') === h.get('Content-Type');",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn headers_delete_works() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const h = new Headers(); \
             h.append('Content-Type', 'text/plain'); \
             h.delete('content-type'); \
             h.has('content-type');",
            "test",
        );
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn headers_from_object() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const h = new Headers({'Content-Type': 'application/json'}); \
             h.has('content-type');",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn headers_is_iterable() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const h = new Headers({'a': '1'}); \
             let count = 0; \
             for (const [k, v] of h) { count++; } \
             count;",
            "test",
        );
        assert_eq!(result.unwrap(), "1");
    }

    #[test]
    fn request_can_be_instantiated() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "new Request('https://example.com') instanceof Request;",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn request_has_url_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const req = new Request('https://example.com'); req.url;",
            "test",
        );
        assert_eq!(result.unwrap(), "https://example.com");
    }

    #[test]
    fn request_has_method_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const req = new Request('https://example.com'); req.method;",
            "test",
        );
        assert_eq!(result.unwrap(), "GET");
    }

    #[test]
    fn request_accepts_method_option() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const req = new Request('https://example.com', {method: 'POST'}); req.method;",
            "test",
        );
        assert_eq!(result.unwrap(), "POST");
    }

    #[test]
    fn request_has_headers_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const req = new Request('https://example.com'); req.headers instanceof Headers;",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn request_from_request() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const req1 = new Request('https://example.com', {method: 'POST'}); \
             const req2 = new Request(req1); \
             req2.method;",
            "test",
        );
        assert_eq!(result.unwrap(), "POST");
    }

    #[test]
    fn response_can_be_instantiated() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("new Response() instanceof Response;", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn response_has_status_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const resp = new Response('', {status: 404}); resp.status;",
            "test",
        );
        assert_eq!(result.unwrap(), "404");
    }

    #[test]
    fn response_has_ok_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const resp = new Response('', {status: 200}); resp.ok;",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn response_ok_is_false_for_error_status() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const resp = new Response('', {status: 404}); resp.ok;",
            "test",
        );
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn response_has_headers_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const resp = new Response(); resp.headers instanceof Headers;",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn response_text_returns_promise() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const resp = new Response('test'); resp.text() instanceof Promise;",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn response_json_returns_promise() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"const resp = new Response('{"foo":"bar"}'); resp.json() instanceof Promise;"#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn response_text_resolves_with_text() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        jstime
            .run_script(
                "globalThis.result = null; \
                 const resp = new Response('hello'); \
                 resp.text().then(t => { globalThis.result = t; });",
                "test",
            )
            .unwrap();
        let result = jstime.run_script("globalThis.result;", "test");
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn response_json_resolves_with_json() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        jstime
            .run_script(
                r#"globalThis.result = null; 
                   const resp = new Response('{"key":"value"}'); 
                   resp.json().then(j => { globalThis.result = j.key; });"#,
                "test",
            )
            .unwrap();
        let result = jstime.run_script("globalThis.result;", "test");
        assert_eq!(result.unwrap(), "value");
    }

    #[test]
    fn response_statustext_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const resp = new Response('', {status: 200, statusText: 'OK'}); typeof resp.statusText;",
            "test",
        );
        assert_eq!(result.unwrap(), "string");
    }
}
