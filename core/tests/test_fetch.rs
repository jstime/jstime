use jstime_core as jstime;

mod common;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof fetch;", "jstime");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn headers_class_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof Headers;", "jstime");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn request_class_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof Request;", "jstime");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn response_class_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof Response;", "jstime");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn headers_basic_operations() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const h = new Headers(); \
             h.append('Content-Type', 'application/json'); \
             h.get('content-type');",
            "jstime",
        );
        assert_eq!(result.unwrap(), "application/json");
    }

    #[test]
    fn headers_from_object() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const h = new Headers({'Content-Type': 'application/json', 'Accept': 'text/html'}); \
             h.has('content-type') && h.has('accept');",
            "jstime",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn request_basic() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const req = new Request('https://example.com'); \
             req.url;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "https://example.com");
    }

    #[test]
    fn request_with_method() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const req = new Request('https://example.com', {method: 'POST'}); \
             req.method;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "POST");
    }

    #[test]
    fn response_basic() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const resp = new Response('hello', {status: 200}); \
             resp.status;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "200");
    }

    #[test]
    fn response_text() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        
        // Queue the promise
        jstime.run_script(
            "globalThis.result = null; \
             const resp = new Response('hello world'); \
             resp.text().then(t => { globalThis.result = t; });",
            "jstime",
        ).unwrap();
        
        // Check the result after event loop completes
        let result = jstime.run_script("globalThis.result;", "jstime");
        assert_eq!(result.unwrap(), "hello world");
    }

    #[test]
    fn response_json() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        
        // Queue the promise
        jstime.run_script(
            r#"globalThis.result = null; 
             const resp = new Response('{"foo":"bar"}'); 
             resp.json().then(j => { globalThis.result = j.foo; });"#,
            "jstime",
        ).unwrap();
        
        // Check the result after event loop completes
        let result = jstime.run_script("globalThis.result;", "jstime");
        assert_eq!(result.unwrap(), "bar");
    }

    #[test]
    fn fetch_returns_promise() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const p = fetch('https://example.com'); \
             p instanceof Promise;",
            "jstime",
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
            "jstime",
        );
        assert_eq!(result.unwrap(), "POST");
    }

    #[test]
    fn headers_iteration() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const h = new Headers({'a': '1', 'b': '2'}); \
             let count = 0; \
             for (const [k, v] of h) { count++; } \
             count;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "2");
    }

    #[test]
    fn response_ok_status() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const resp = new Response('', {status: 404}); \
             resp.ok;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "false");
    }

}
