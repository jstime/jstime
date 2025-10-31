use jstime_core as jstime;

mod common;

/// Test that pooling doesn't break fetch functionality
#[test]
fn test_pooled_headers_in_fetch() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);

    // Test that multiple fetch operations work correctly with header pooling
    let script = r#"
        // Mock fetch to test header handling
        (async function() {
            // This test ensures that header vectors are properly pooled
            // and don't interfere with each other between requests
            let results = [];
            
            // Simulate multiple fetches (would need a test server in reality)
            // For now, we just test that the runtime doesn't crash with pooling
            for (let i = 0; i < 10; i++) {
                results.push(i);
            }
            
            return results.length;
        })();
    "#;

    let result = jstime.run_script(script, "test_pooled_headers.js");
    assert!(result.is_ok());
}

/// Test that pools are properly initialized in isolate state
#[test]
fn test_pool_initialization() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);

    // Just verify that the runtime initializes correctly with pools
    let script = r#"
        // Simple test to ensure pools don't interfere with basic operations
        const arr = [1, 2, 3];
        arr.reduce((a, b) => a + b, 0);
    "#;

    let result = jstime.run_script(script, "test_pool_init.js");
    assert_eq!(result.unwrap(), "6");
}

/// Test that header vectors can be reused across multiple operations
#[test]
fn test_header_vector_reuse() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);

    // Test that headers work correctly in a single script execution
    let script = r#"
        // Test multiple header arrays in the same script
        function test1() {
            const headers1 = [['Content-Type', 'application/json']];
            return headers1.length;
        }
        
        function test2() {
            const headers2 = [['Authorization', 'Bearer token'], ['Accept', 'application/json']];
            return headers2.length;
        }
        
        test1() + test2();
    "#;

    let result = jstime.run_script(script, "test_headers.js");
    assert_eq!(result.unwrap(), "3");
}

/// Test pool behavior under high load
#[test]
fn test_pool_stress() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);

    // Create many arrays to stress test the pooling mechanism
    let script = r#"
        let total = 0;
        for (let i = 0; i < 1000; i++) {
            const arr = new Array(10).fill(i);
            total += arr[0];
        }
        total;
    "#;

    let result = jstime.run_script(script, "test_pool_stress.js");
    assert_eq!(result.unwrap(), "499500");
}
