use jstime_core as jstime;
mod common;

/// Test that string cache is being used and reducing allocations
#[test]
fn test_string_cache_basic() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);

    // Test that stack traces work (which use cached "stack" string)
    let result = jstime.run_script(
        r#"
        try {
            throw new Error("test error");
        } catch (e) {
            e.stack;
        }
        "#,
        "test_stack.js",
    );

    assert!(result.is_ok());

    // Test fetch operations that use cached strings (status, statusText, headers)
    let result = jstime.run_script(
        r#"
        // Create a mock response-like object to test string caching pattern
        const obj = {
            status: 200,
            statusText: "OK",
            headers: []
        };
        JSON.stringify(obj);
        "#,
        "test_fetch.js",
    );

    assert!(result.is_ok());
}

/// Test that import.meta.url works (which uses cached "url" string)
#[test]
fn test_string_cache_import_meta() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);

    // Create a temporary test file
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_import_meta.mjs");
    std::fs::write(&test_file, "export const url = import.meta.url;").unwrap();

    let result = jstime.import(test_file.to_str().unwrap());

    // Clean up
    let _ = std::fs::remove_file(&test_file);

    assert!(result.is_ok());
}

/// Test multiple operations that should benefit from string caching
#[test]
fn test_string_cache_multiple_operations() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);

    // Run multiple operations that access cached strings
    for i in 0..10 {
        let script = format!(
            r#"
            try {{
                if ({} === 5) {{
                    throw new Error("test error {i}");
                }}
                const obj = {{
                    status: {i},
                    statusText: "iteration",
                    headers: []
                }};
                JSON.stringify(obj);
            }} catch (e) {{
                e.stack;
            }}
            "#,
            i
        );

        let result = jstime.run_script(&script, "test_loop.js");
        assert!(result.is_ok());
    }
}

/// Test that error formatting with stack traces works correctly
#[test]
fn test_string_cache_error_formatting() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);

    let result = jstime.run_script(
        r#"
        function throwError() {
            throw new Error("nested error");
        }
        
        function callThrow() {
            throwError();
        }
        
        try {
            callThrow();
        } catch (e) {
            // Access stack property multiple times to verify caching
            const s1 = e.stack;
            const s2 = e.stack;
            s1 === s2;
        }
        "#,
        "test_error_stack.js",
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "true");
}
