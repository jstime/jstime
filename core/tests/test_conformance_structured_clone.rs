use jstime_core as jstime;

mod common;

#[cfg(test)]
mod conformance_structured_clone {
    use super::*;

    #[test]
    fn structuredclone_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof structuredClone", "test");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn structuredclone_is_global() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof globalThis.structuredClone", "test");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn structuredclone_clones_primitive_values() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Number
        let result = jstime.run_script("structuredClone(42)", "test");
        assert_eq!(result.unwrap(), "42");

        // String
        let result = jstime.run_script("structuredClone('hello')", "test");
        assert_eq!(result.unwrap(), "hello");

        // Boolean
        let result = jstime.run_script("structuredClone(true)", "test");
        assert_eq!(result.unwrap(), "true");

        // Null
        let result = jstime.run_script("structuredClone(null)", "test");
        assert_eq!(result.unwrap(), "null");

        // Undefined
        let result = jstime.run_script("structuredClone(undefined)", "test");
        assert_eq!(result.unwrap(), "undefined");
    }

    #[test]
    fn structuredclone_clones_objects() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const obj = { a: 1, b: 'test', c: true };
            const cloned = structuredClone(obj);
            JSON.stringify(cloned);
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), r#"{"a":1,"b":"test","c":true}"#);
    }

    #[test]
    fn structuredclone_creates_deep_copy() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const obj = { a: 1 };
            const cloned = structuredClone(obj);
            obj !== cloned;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn structuredclone_clones_nested_objects() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const obj = { a: { b: { c: 1 } } };
            const cloned = structuredClone(obj);
            obj.a !== cloned.a && JSON.stringify(cloned);
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), r#"{"a":{"b":{"c":1}}}"#);
    }

    #[test]
    fn structuredclone_clones_arrays() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const arr = [1, 2, 3];
            const cloned = structuredClone(arr);
            JSON.stringify(cloned);
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "[1,2,3]");
    }

    #[test]
    fn structuredclone_clones_date_objects() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const date = new Date('2024-01-01T00:00:00.000Z');
            const cloned = structuredClone(date);
            cloned.toISOString();
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "2024-01-01T00:00:00.000Z");
    }

    #[test]
    fn structuredclone_clones_regexp() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const re = /test/gi;
            const cloned = structuredClone(re);
            cloned.source + '/' + cloned.flags;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "test/gi");
    }

    #[test]
    fn structuredclone_clones_map() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const map = new Map([['key1', 'value1'], ['key2', 'value2']]);
            const cloned = structuredClone(map);
            cloned.get('key1') + ',' + cloned.get('key2');
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "value1,value2");
    }

    #[test]
    fn structuredclone_clones_set() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const set = new Set([1, 2, 3]);
            const cloned = structuredClone(set);
            cloned.size;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "3");
    }

    #[test]
    fn structuredclone_preserves_set_values() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const set = new Set([1, 2, 3]);
            const cloned = structuredClone(set);
            cloned.has(1) && cloned.has(2) && cloned.has(3);
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn structuredclone_clones_arraybuffer() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const buffer = new ArrayBuffer(8);
            const cloned = structuredClone(buffer);
            cloned.byteLength;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "8");
    }

    #[test]
    fn structuredclone_clones_typed_arrays() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const arr = new Uint8Array([1, 2, 3, 4]);
            const cloned = structuredClone(arr);
            Array.from(cloned).join(',');
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "1,2,3,4");
    }

    #[test]
    fn structuredclone_handles_circular_references() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const obj = { a: 1 };
            obj.self = obj;
            const cloned = structuredClone(obj);
            cloned.self === cloned && cloned.self.a;
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "1");
    }

    #[test]
    fn structuredclone_clones_boolean_objects() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const bool = new Boolean(true);
            const cloned = structuredClone(bool);
            cloned.valueOf();
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn structuredclone_clones_number_objects() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const num = new Number(42);
            const cloned = structuredClone(num);
            cloned.valueOf();
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "42");
    }

    #[test]
    fn structuredclone_clones_string_objects() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const str = new String("test");
            const cloned = structuredClone(str);
            cloned.valueOf();
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn structuredclone_throws_on_function() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            try {
                structuredClone(() => {});
                'should have thrown';
            } catch (e) {
                'error thrown';
            }
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "error thrown");
    }

    #[test]
    fn structuredclone_throws_on_symbol() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            try {
                structuredClone(Symbol('test'));
                'should have thrown';
            } catch (e) {
                'error thrown';
            }
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "error thrown");
    }

    #[test]
    fn structuredclone_with_complex_nested_structure() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            r#"
            const complex = {
                num: 42,
                str: "hello",
                bool: true,
                date: new Date("2024-01-01"),
                arr: [1, 2, { nested: true }],
                map: new Map([["key", "value"]]),
                set: new Set([1, 2, 3]),
                regexp: /test/i
            };
            const cloned = structuredClone(complex);
            cloned.num === 42 && cloned.str === "hello" && cloned.map.get("key") === "value";
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }
}
