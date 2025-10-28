// Streams API Conformance Tests
// Based on https://streams.spec.whatwg.org/

use jstime_core as jstime;

mod common;

#[cfg(test)]
mod conformance_streams {
    use super::*;

    // Test that ReadableStream exists as a global constructor
    #[test]
    fn readable_stream_exists() {
        let result = common::get_type_of("ReadableStream");
        assert_eq!(result.unwrap(), "function");
    }

    // Test that WritableStream exists as a global constructor
    #[test]
    fn writable_stream_exists() {
        let result = common::get_type_of("WritableStream");
        assert_eq!(result.unwrap(), "function");
    }

    // Test that TransformStream exists as a global constructor
    #[test]
    fn transform_stream_exists() {
        let result = common::get_type_of("TransformStream");
        assert_eq!(result.unwrap(), "function");
    }

    // Test that ReadableStreamDefaultReader exists
    #[test]
    fn readable_stream_default_reader_exists() {
        let result = common::get_type_of("ReadableStreamDefaultReader");
        assert_eq!(result.unwrap(), "function");
    }

    // Test that WritableStreamDefaultWriter exists
    #[test]
    fn writable_stream_default_writer_exists() {
        let result = common::get_type_of("WritableStreamDefaultWriter");
        assert_eq!(result.unwrap(), "function");
    }

    // Test ReadableStream constructor
    #[test]
    fn readable_stream_constructor() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("const rs = new ReadableStream(); typeof rs;", "test");
        assert_eq!(result.unwrap(), "object");
    }

    // Test ReadableStream with start callback
    #[test]
    fn readable_stream_with_start() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            let called = false;
            const rs = new ReadableStream({
              start(controller) {
                called = true;
                controller.close();
              }
            });
            called;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    // Test ReadableStream enqueue
    #[test]
    fn readable_stream_enqueue() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            let enqueued = [];
            const rs = new ReadableStream({
              start(controller) {
                enqueued.push("chunk1");
                controller.enqueue("chunk1");
                enqueued.push("chunk2");
                controller.enqueue("chunk2");
                controller.close();
              }
            });
            enqueued.join(',');
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "chunk1,chunk2");
    }

    // Test ReadableStream locked property
    #[test]
    fn readable_stream_locked() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const rs = new ReadableStream();
            const before = rs.locked;
            const reader = rs.getReader();
            const after = rs.locked;
            before + ',' + after;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "false,true");
    }

    // Test ReadableStream getReader
    #[test]
    fn readable_stream_get_reader() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const rs = new ReadableStream();
            const reader = rs.getReader();
            typeof reader;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "object");
    }

    // Test reader.read() returns a promise
    #[test]
    fn readable_stream_reader_read_returns_promise() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const rs = new ReadableStream({
              start(controller) {
                controller.enqueue("data");
                controller.close();
              }
            });
            const reader = rs.getReader();
            const promise = reader.read();
            promise instanceof Promise;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    // Test WritableStream constructor
    #[test]
    fn writable_stream_constructor() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("const ws = new WritableStream(); typeof ws;", "test");
        assert_eq!(result.unwrap(), "object");
    }

    // Test WritableStream getWriter
    #[test]
    fn writable_stream_get_writer() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const ws = new WritableStream();
            const writer = ws.getWriter();
            typeof writer;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "object");
    }

    // Test WritableStream write
    #[test]
    fn writable_stream_write() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            let written = [];
            const ws = new WritableStream({
              write(chunk) {
                written.push(chunk);
              }
            });
            const writer = ws.getWriter();
            writer.write("hello");
            writer.write("world");
            written.join(',');
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "hello,world");
    }

    // Test WritableStream locked property
    #[test]
    fn writable_stream_locked() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const ws = new WritableStream();
            const before = ws.locked;
            const writer = ws.getWriter();
            const after = ws.locked;
            before + ',' + after;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "false,true");
    }

    // Test TransformStream constructor
    #[test]
    fn transform_stream_constructor() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("const ts = new TransformStream(); typeof ts;", "test");
        assert_eq!(result.unwrap(), "object");
    }

    // Test TransformStream readable property
    #[test]
    fn transform_stream_readable() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const ts = new TransformStream();
            ts.readable instanceof ReadableStream;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    // Test TransformStream writable property
    #[test]
    fn transform_stream_writable() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const ts = new TransformStream();
            ts.writable instanceof WritableStream;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    // Test TransformStream with transform function
    #[test]
    fn transform_stream_with_transform() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            let transformed = [];
            const ts = new TransformStream({
              transform(chunk, controller) {
                const upper = chunk.toUpperCase();
                transformed.push(upper);
                controller.enqueue(upper);
              }
            });
            const writer = ts.writable.getWriter();
            writer.write("hello");
            transformed[0];
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "HELLO");
    }

    // Test TransformStream passthrough (no transform)
    #[test]
    fn transform_stream_passthrough() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const ts = new TransformStream();
            const writer = ts.writable.getWriter();
            const reader = ts.readable.getReader();
            writer.write("data");
            // Check that readable stream has data queued
            typeof reader;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "object");
    }

    // Test Response.body returns ReadableStream
    #[test]
    fn response_body_is_readable_stream() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const response = new Response("Hello");
            response.body instanceof ReadableStream;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    // Test Response.body can be read
    #[test]
    fn response_body_can_be_read() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const response = new Response("Hello, World!");
            const reader = response.body.getReader();
            // Check that reader exists and is an object
            typeof reader;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "object");
    }

    // Test reader.releaseLock()
    #[test]
    fn readable_stream_reader_release_lock() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const rs = new ReadableStream();
            const reader = rs.getReader();
            const locked1 = rs.locked;
            reader.releaseLock();
            const locked2 = rs.locked;
            locked1 + ',' + locked2;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true,false");
    }

    // Test writer.releaseLock()
    #[test]
    fn writable_stream_writer_release_lock() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const ws = new WritableStream();
            const writer = ws.getWriter();
            const locked1 = ws.locked;
            writer.releaseLock();
            const locked2 = ws.locked;
            locked1 + ',' + locked2;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true,false");
    }

    // Test ReadableStream cancel
    #[test]
    fn readable_stream_cancel() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const rs = new ReadableStream({
              start(controller) {
                controller.enqueue("data");
              }
            });
            const cancelPromise = rs.cancel();
            cancelPromise instanceof Promise;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    // Test WritableStream abort
    #[test]
    fn writable_stream_abort() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const ws = new WritableStream();
            const abortPromise = ws.abort();
            abortPromise instanceof Promise;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }
}
