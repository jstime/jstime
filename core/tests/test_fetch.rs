use jstime_core as jstime;

mod common;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_exists() {
        let result = common::get_type_of("fetch");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn headers_class_exists() {
        let result = common::get_type_of("Headers");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn request_class_exists() {
        let result = common::get_type_of("Request");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn response_class_exists() {
        let result = common::get_type_of("Response");
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
        jstime
            .run_script(
                "globalThis.result = null; \
             const resp = new Response('hello world'); \
             resp.text().then(t => { globalThis.result = t; });",
                "jstime",
            )
            .unwrap();

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
        jstime
            .run_script(
                r#"globalThis.result = null; 
             const resp = new Response('{"foo":"bar"}'); 
             resp.json().then(j => { globalThis.result = j.foo; });"#,
                "jstime",
            )
            .unwrap();

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

    #[test]
    fn response_body_is_readable_stream() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const resp = new Response('test'); \
             resp.body instanceof ReadableStream;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn response_body_stream_readable() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Queue the stream reading
        jstime
            .run_script(
                "globalThis.result = null; \
                 const resp = new Response('Hello Stream'); \
                 const reader = resp.body.getReader(); \
                 reader.read().then(chunk => { \
                   globalThis.result = chunk; \
                 });",
                "jstime",
            )
            .unwrap();

        // Check the result
        let done_result = jstime.run_script("globalThis.result.done;", "jstime");
        assert_eq!(done_result.unwrap(), "false");

        let has_value =
            jstime.run_script("globalThis.result.value instanceof Uint8Array;", "jstime");
        assert_eq!(has_value.unwrap(), "true");
    }

    #[test]
    fn response_body_stream_closes() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Queue the stream reading
        jstime
            .run_script(
                "globalThis.chunks = []; \
                 const resp = new Response('test'); \
                 const reader = resp.body.getReader(); \
                 async function readAll() { \
                   while (true) { \
                     const {done, value} = await reader.read(); \
                     if (done) break; \
                     globalThis.chunks.push(value); \
                   } \
                 } \
                 readAll();",
                "jstime",
            )
            .unwrap();

        // Check we got at least one chunk
        let chunk_count = jstime.run_script("globalThis.chunks.length;", "jstime");
        assert!(chunk_count.unwrap().parse::<i32>().unwrap() > 0);
    }

    #[test]
    fn response_text_with_streaming() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Queue the promise
        jstime
            .run_script(
                "globalThis.result = null; \
                 const resp = new Response('streaming text'); \
                 resp.text().then(t => { globalThis.result = t; });",
                "jstime",
            )
            .unwrap();

        // Check the result
        let result = jstime.run_script("globalThis.result;", "jstime");
        assert_eq!(result.unwrap(), "streaming text");
    }

    #[test]
    fn response_large_content() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test with content larger than chunk size (64KB)
        jstime
            .run_script(
                "globalThis.result = null; \
                 const largeContent = 'A'.repeat(100000); \
                 const resp = new Response(largeContent); \
                 resp.text().then(t => { \
                   globalThis.result = (t.length === 100000 && t[0] === 'A'); \
                 });",
                "jstime",
            )
            .unwrap();

        // Check the result
        let result = jstime.run_script("globalThis.result;", "jstime");
        assert_eq!(result.unwrap(), "true");
    }
}
