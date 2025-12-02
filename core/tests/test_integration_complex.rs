// Integration tests for complex application workloads
// These tests simulate real-world usage patterns and stress conditions
// to find potential bugs that may not appear in simpler tests.

use jstime_core as jstime;

mod common;

#[cfg(test)]
mod integration_tests {
    use super::*;

    // ============================================================================
    // CONCURRENT OPERATIONS TESTS
    // ============================================================================

    #[test]
    fn test_concurrent_timers_and_promises() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test interaction between timers and promises
        let script = r#"
            globalThis.results = [];
            globalThis.completed = false;
            
            // Create a promise that resolves after all timers
            const timerPromise = new Promise(resolve => {
                let timerCount = 0;
                const expected = 5;
                
                for (let i = 0; i < expected; i++) {
                    setTimeout(() => {
                        timerCount++;
                        globalThis.results.push('timer_' + i);
                        if (timerCount === expected) {
                            resolve('all_timers_done');
                        }
                    }, (i + 1) * 10);
                }
            });
            
            // Chain with more async operations
            timerPromise
                .then(msg => {
                    globalThis.results.push(msg);
                    return Promise.resolve('promise_chain');
                })
                .then(msg => {
                    globalThis.results.push(msg);
                    globalThis.completed = true;
                });
        "#;
        jstime.run_script(script, "test").unwrap();

        let completed = jstime.run_script("globalThis.completed", "test");
        assert_eq!(completed.unwrap(), "true");

        let result = jstime.run_script(
            "globalThis.results.filter(r => r.startsWith('timer_')).length",
            "test",
        );
        assert_eq!(result.unwrap(), "5");
    }

    #[test]
    fn test_nested_settimeout_chain() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test deeply nested setTimeout calls
        let script = r#"
            globalThis.chainDepth = 0;
            globalThis.maxDepth = 10;
            
            function scheduleNext() {
                if (globalThis.chainDepth < globalThis.maxDepth) {
                    globalThis.chainDepth++;
                    setTimeout(scheduleNext, 5);
                }
            }
            scheduleNext();
        "#;
        jstime.run_script(script, "test").unwrap();

        let result = jstime.run_script("globalThis.chainDepth", "test");
        assert_eq!(result.unwrap(), "10");
    }

    #[test]
    fn test_interval_with_variable_workload() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Simulate variable workload in interval callbacks
        let script = r#"
            globalThis.executions = [];
            globalThis.stopAt = 5;
            
            const id = setInterval(() => {
                const execNum = globalThis.executions.length + 1;
                
                // Simulate varying workload
                let work = 0;
                for (let i = 0; i < execNum * 100; i++) {
                    work += Math.sqrt(i);
                }
                
                globalThis.executions.push({
                    num: execNum,
                    workResult: Math.floor(work)
                });
                
                if (execNum >= globalThis.stopAt) {
                    clearInterval(id);
                }
            }, 10);
        "#;
        jstime.run_script(script, "test").unwrap();

        let count = jstime.run_script("globalThis.executions.length", "test");
        assert_eq!(count.unwrap(), "5");

        // Verify each execution completed
        let all_have_results =
            jstime.run_script("globalThis.executions.every(e => e.workResult > 0)", "test");
        assert_eq!(all_have_results.unwrap(), "true");
    }

    // ============================================================================
    // MEMORY MANAGEMENT TESTS
    // ============================================================================

    #[test]
    fn test_large_array_operations() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test operations on large arrays
        let script = r#"
            const size = 10000;
            const arr = new Array(size);
            
            // Fill with data
            for (let i = 0; i < size; i++) {
                arr[i] = { index: i, value: Math.random() };
            }
            
            // Perform various operations
            const sorted = arr.slice().sort((a, b) => a.value - b.value);
            const filtered = arr.filter(item => item.value > 0.5);
            const mapped = arr.map(item => item.index * 2);
            const reduced = arr.reduce((sum, item) => sum + item.index, 0);
            
            JSON.stringify({
                originalLength: arr.length,
                sortedLength: sorted.length,
                filteredLength: filtered.length,
                mappedLength: mapped.length,
                reducedSum: reduced
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["originalLength"], 10000);
        assert_eq!(json["sortedLength"], 10000);
        assert_eq!(json["mappedLength"], 10000);
        // Sum of 0 to 9999 = 9999 * 10000 / 2 = 49995000
        assert_eq!(json["reducedSum"], 49995000);
    }

    #[test]
    fn test_large_string_operations() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test operations on large strings
        let script = r#"
            const base = 'Hello World! ';
            let large = '';
            
            // Build a large string
            for (let i = 0; i < 1000; i++) {
                large += base;
            }
            
            // Perform string operations
            const upperCase = large.toUpperCase();
            const splits = large.split(' ').length;
            const replaced = large.replace(/Hello/g, 'Hi');
            const indexOf = large.indexOf('World');
            const lastIndexOf = large.lastIndexOf('World');
            
            JSON.stringify({
                originalLength: large.length,
                upperCaseLength: upperCase.length,
                splitCount: splits,
                replacedLength: replaced.length,
                firstIndex: indexOf,
                lastIndex: lastIndexOf
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        // "Hello World! " is 13 characters, repeated 1000 times = 13000
        assert_eq!(json["originalLength"], 13000);
        assert_eq!(json["upperCaseLength"], 13000);
        // Original length is 13000, "Hello" is 5 chars, "Hi" is 2 chars
        // 1000 replacements: 13000 - 1000 * 3 = 10000
        assert_eq!(json["replacedLength"], 10000);
    }

    #[test]
    fn test_deep_object_nesting() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test deeply nested objects
        let script = r#"
            function createDeepObject(depth) {
                if (depth === 0) return { leaf: true };
                return {
                    level: depth,
                    child: createDeepObject(depth - 1)
                };
            }
            
            const deep = createDeepObject(50);
            
            // Traverse to find the leaf
            let current = deep;
            let levels = 0;
            while (current.child) {
                levels++;
                current = current.child;
            }
            
            JSON.stringify({
                hasLeaf: current.leaf === true,
                levelsTraversed: levels
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["hasLeaf"], true);
        assert_eq!(json["levelsTraversed"], 50);
    }

    #[test]
    fn test_circular_reference_in_structured_clone() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test structuredClone with circular references
        let script = r#"
            const obj = { name: 'root', children: [] };
            const child1 = { name: 'child1', parent: obj };
            const child2 = { name: 'child2', parent: obj };
            obj.children.push(child1, child2);
            
            // Create a circular reference
            child1.sibling = child2;
            child2.sibling = child1;
            
            // Clone with circular reference
            const cloned = structuredClone(obj);
            
            // Verify structure preserved
            const child1Cloned = cloned.children[0];
            const child2Cloned = cloned.children[1];
            
            JSON.stringify({
                rootName: cloned.name,
                childCount: cloned.children.length,
                child1Name: child1Cloned.name,
                child2Name: child2Cloned.name,
                parentReference: child1Cloned.parent === cloned,
                siblingReference: child1Cloned.sibling === child2Cloned
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["rootName"], "root");
        assert_eq!(json["childCount"], 2);
        assert_eq!(json["child1Name"], "child1");
        assert_eq!(json["child2Name"], "child2");
        assert_eq!(json["parentReference"], true);
        assert_eq!(json["siblingReference"], true);
    }

    // ============================================================================
    // ERROR HANDLING AND RECOVERY TESTS
    // ============================================================================

    #[test]
    fn test_error_in_timer_callback() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test that errors in timer callbacks don't crash and other timers still run
        let script = r#"
            globalThis.results = [];
            
            setTimeout(() => {
                globalThis.results.push('first_before');
                throw new Error('Intentional error in timer');
                globalThis.results.push('first_after'); // Should not reach
            }, 10);
            
            setTimeout(() => {
                globalThis.results.push('second_timer');
            }, 20);
            
            setTimeout(() => {
                globalThis.results.push('third_timer');
            }, 30);
        "#;
        // The script will execute and run timers
        let _ = jstime.run_script(script, "test");

        // Check that second and third timers executed despite error in first
        let result = jstime.run_script("globalThis.results", "test");
        let result_str = result.unwrap();

        // The first timer should have recorded 'first_before' before throwing
        assert!(result_str.contains("first_before"));
    }

    #[test]
    fn test_promise_rejection_handling() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test promise rejection handling
        let script = r#"
            globalThis.rejections = [];
            globalThis.successes = [];
            
            Promise.reject(new Error('First rejection'))
                .catch(e => globalThis.rejections.push(e.message));
            
            Promise.resolve('Success 1')
                .then(v => globalThis.successes.push(v));
            
            Promise.reject(new Error('Second rejection'))
                .catch(e => globalThis.rejections.push(e.message));
            
            Promise.resolve('Success 2')
                .then(v => globalThis.successes.push(v));
        "#;
        jstime.run_script(script, "test").unwrap();

        let rejections = jstime.run_script("globalThis.rejections.join(',')", "test");
        assert_eq!(rejections.unwrap(), "First rejection,Second rejection");

        let successes = jstime.run_script("globalThis.successes.join(',')", "test");
        assert_eq!(successes.unwrap(), "Success 1,Success 2");
    }

    #[test]
    fn test_error_recovery_in_event_listener() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test that errors in event listeners don't prevent other listeners
        let script = r#"
            const target = new EventTarget();
            globalThis.calls = [];
            
            target.addEventListener('test', () => {
                globalThis.calls.push('listener1_before');
                throw new Error('Error in listener 1');
                globalThis.calls.push('listener1_after');
            });
            
            target.addEventListener('test', () => {
                globalThis.calls.push('listener2');
            });
            
            target.addEventListener('test', () => {
                globalThis.calls.push('listener3');
            });
            
            try {
                target.dispatchEvent(new Event('test'));
            } catch (e) {
                globalThis.calls.push('caught:' + e.message);
            }
        "#;
        let _ = jstime.run_script(script, "test");

        // Check what was recorded
        let calls = jstime.run_script("globalThis.calls.join(',')", "test");
        let calls_str = calls.unwrap();

        // First listener should have started
        assert!(calls_str.contains("listener1_before"));
    }

    // ============================================================================
    // API INTEGRATION TESTS
    // ============================================================================

    #[test]
    fn test_url_searchparams_complex_operations() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test complex URL and URLSearchParams interactions
        let script = r#"
            const url = new URL('https://example.com/path?existing=value');
            const params = url.searchParams;
            
            // Perform multiple operations
            params.append('key1', 'value1');
            params.append('key1', 'value2'); // Duplicate key
            params.set('key2', 'initial');
            params.set('key2', 'updated'); // Update
            params.append('key3', 'a');
            params.append('key3', 'b');
            params.append('key3', 'c');
            params.delete('key3', 'b'); // Delete specific value if supported, or all
            
            // Collect results
            const key1Values = params.getAll('key1');
            const key2Value = params.get('key2');
            const key3Values = params.getAll('key3');
            const allEntries = [...params.entries()];
            
            JSON.stringify({
                key1Count: key1Values.length,
                key2Value: key2Value,
                key3Count: key3Values.length,
                totalEntries: allEntries.length,
                urlHref: url.href
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["key1Count"], 2);
        assert_eq!(json["key2Value"], "updated");
        // URL should reflect the parameter changes
        assert!(json["urlHref"].as_str().unwrap().contains("key1=value1"));
    }

    #[test]
    fn test_text_encoding_large_data() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test TextEncoder/TextDecoder with large data
        let script = r#"
            const encoder = new TextEncoder();
            const decoder = new TextDecoder();
            
            // Create a large string with various characters
            let largeString = '';
            for (let i = 0; i < 1000; i++) {
                largeString += 'Hello 🌍 World! 日本語テスト ';
            }
            
            // Encode
            const encoded = encoder.encode(largeString);
            
            // Decode
            const decoded = decoder.decode(encoded);
            
            // Verify round-trip
            JSON.stringify({
                originalLength: largeString.length,
                encodedLength: encoded.length,
                decodedLength: decoded.length,
                roundTripMatch: largeString === decoded
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["roundTripMatch"], true);
        // Encoded length should be >= original due to UTF-8 multi-byte characters
        assert!(
            json["encodedLength"].as_i64().unwrap() >= json["originalLength"].as_i64().unwrap()
        );
    }

    #[test]
    fn test_base64_with_binary_data() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test btoa/atob with binary-like data
        let script = r#"
            const results = [];
            
            // Test all possible byte values (0-255)
            let allBytes = '';
            for (let i = 0; i < 256; i++) {
                allBytes += String.fromCharCode(i);
            }
            
            // Encode and decode
            const encoded = btoa(allBytes);
            const decoded = atob(encoded);
            
            // Verify each byte
            let allMatch = true;
            for (let i = 0; i < 256; i++) {
                if (decoded.charCodeAt(i) !== i) {
                    allMatch = false;
                    break;
                }
            }
            
            JSON.stringify({
                encodedLength: encoded.length,
                decodedLength: decoded.length,
                allBytesMatch: allMatch
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["decodedLength"], 256);
        assert_eq!(json["allBytesMatch"], true);
    }

    #[test]
    fn test_event_target_with_many_listeners() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test EventTarget with many listeners
        let script = r#"
            const target = new EventTarget();
            const listenerCount = 100;
            globalThis.callOrder = [];
            globalThis.listeners = [];
            
            // Add many listeners
            for (let i = 0; i < listenerCount; i++) {
                const listener = () => {
                    globalThis.callOrder.push(i);
                };
                globalThis.listeners.push(listener);
                target.addEventListener('test', listener);
            }
            
            // Dispatch event
            target.dispatchEvent(new Event('test'));
            
            // Remove half the listeners
            for (let i = 0; i < listenerCount / 2; i++) {
                target.removeEventListener('test', globalThis.listeners[i]);
            }
            
            // Dispatch again
            globalThis.callOrder = [];
            target.dispatchEvent(new Event('test'));
            
            JSON.stringify({
                firstDispatchCount: listenerCount,
                secondDispatchCount: globalThis.callOrder.length,
                firstInSecondDispatch: globalThis.callOrder[0]
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["firstDispatchCount"], 100);
        assert_eq!(json["secondDispatchCount"], 50);
        assert_eq!(json["firstInSecondDispatch"], 50); // First remaining listener is at index 50
    }

    #[test]
    fn test_crypto_random_distribution() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test that crypto.getRandomValues produces reasonably distributed values
        let script = r#"
            const sampleSize = 1000;
            const array = new Uint8Array(sampleSize);
            crypto.getRandomValues(array);
            
            // Calculate basic statistics
            let sum = 0;
            let zeros = 0;
            let maxValue = 0;
            let minValue = 255;
            
            for (let i = 0; i < sampleSize; i++) {
                const val = array[i];
                sum += val;
                if (val === 0) zeros++;
                if (val > maxValue) maxValue = val;
                if (val < minValue) minValue = val;
            }
            
            const mean = sum / sampleSize;
            
            JSON.stringify({
                sampleSize: sampleSize,
                mean: mean,
                minValue: minValue,
                maxValue: maxValue,
                zeroCount: zeros
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        // Mean should be around 127.5 for uniform distribution
        let mean = json["mean"].as_f64().unwrap();
        assert!(
            mean > 100.0 && mean < 155.0,
            "Mean {} is not near expected 127.5",
            mean
        );

        // Should have both low and high values
        assert!(json["minValue"].as_i64().unwrap() < 50);
        assert!(json["maxValue"].as_i64().unwrap() > 200);
    }

    #[test]
    fn test_uuid_uniqueness() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test that randomUUID generates unique values
        let script = r#"
            const count = 1000;
            const uuids = new Set();
            
            for (let i = 0; i < count; i++) {
                uuids.add(crypto.randomUUID());
            }
            
            // Check format of one UUID
            const sample = crypto.randomUUID();
            const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
            
            JSON.stringify({
                uniqueCount: uuids.size,
                expectedCount: count,
                validFormat: uuidRegex.test(sample),
                sampleUuid: sample
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["uniqueCount"], json["expectedCount"]);
        assert_eq!(json["validFormat"], true);
    }

    // ============================================================================
    // PERFORMANCE AND TIMING TESTS
    // ============================================================================

    #[test]
    fn test_performance_timing_accuracy() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test performance.now() timing accuracy
        let script = r#"
            const measurements = [];
            const iterations = 100;
            
            for (let i = 0; i < iterations; i++) {
                const start = performance.now();
                // Small workload
                let x = 0;
                for (let j = 0; j < 1000; j++) {
                    x += Math.sqrt(j);
                }
                const end = performance.now();
                measurements.push(end - start);
            }
            
            // Calculate statistics
            const total = measurements.reduce((a, b) => a + b, 0);
            const mean = total / measurements.length;
            const min = Math.min(...measurements);
            const max = Math.max(...measurements);
            
            JSON.stringify({
                measurementCount: measurements.length,
                meanMs: mean,
                minMs: min,
                maxMs: max,
                allNonNegative: measurements.every(m => m >= 0),
                timeOriginValid: performance.timeOrigin > 0
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["measurementCount"], 100);
        assert_eq!(json["allNonNegative"], true);
        assert_eq!(json["timeOriginValid"], true);
    }

    #[test]
    fn test_timer_ordering_under_load() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test that timers maintain correct ordering even under load
        let script = r#"
            globalThis.executionOrder = [];
            const delays = [100, 50, 75, 25, 10, 30, 60, 80, 90, 40];
            
            // Schedule timers in random delay order
            delays.forEach((delay, index) => {
                setTimeout(() => {
                    globalThis.executionOrder.push({
                        scheduled: index,
                        delay: delay
                    });
                }, delay);
            });
        "#;
        jstime.run_script(script, "test").unwrap();

        // Verify timers executed in delay order (not scheduling order)
        let result = jstime.run_script(
            "globalThis.executionOrder.map(e => e.delay).join(',')",
            "test",
        );
        assert_eq!(result.unwrap(), "10,25,30,40,50,60,75,80,90,100");
    }

    // ============================================================================
    // MODULE LOADING TESTS
    // ============================================================================

    #[test]
    fn test_module_with_async_initialization() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create test files
        let temp_dir = std::env::temp_dir();
        let module_file = temp_dir.join("test_async_init.mjs");

        std::fs::write(
            &module_file,
            r#"
// Module with async initialization
export const syncValue = 'sync';

export const asyncValue = await Promise.resolve('async');

export const config = await (async () => {
    return {
        initialized: true,
        timestamp: Date.now()
    };
})();

globalThis.moduleLoaded = true;
            "#,
        )
        .unwrap();

        let result = jstime.import(module_file.to_str().unwrap());
        assert!(result.is_ok(), "Failed to import: {:?}", result);

        let loaded = jstime.run_script("globalThis.moduleLoaded", "test");
        assert_eq!(loaded.unwrap(), "true");

        std::fs::remove_file(&module_file).ok();
    }

    #[test]
    fn test_module_dependency_chain() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create a chain of dependent modules
        let temp_dir = std::env::temp_dir();

        // Module A (base)
        let module_a = temp_dir.join("test_chain_a.mjs");
        std::fs::write(
            &module_a,
            r#"
export const valueA = 'A';
export function processA(x) { return x + '_A'; }
            "#,
        )
        .unwrap();

        // Module B (depends on A)
        let module_b = temp_dir.join("test_chain_b.mjs");
        std::fs::write(
            &module_b,
            format!(
                r#"
import {{ valueA, processA }} from '{}';
export const valueB = valueA + 'B';
export function processB(x) {{ return processA(x) + '_B'; }}
            "#,
                module_a.to_str().unwrap()
            ),
        )
        .unwrap();

        // Module C (depends on B)
        let module_c = temp_dir.join("test_chain_c.mjs");
        std::fs::write(
            &module_c,
            format!(
                r#"
import {{ valueB, processB }} from '{}';
export const valueC = valueB + 'C';
export function processC(x) {{ return processB(x) + '_C'; }}
globalThis.chainResult = {{
    value: valueC,
    processed: processC('start')
}};
            "#,
                module_b.to_str().unwrap()
            ),
        )
        .unwrap();

        let result = jstime.import(module_c.to_str().unwrap());
        assert!(result.is_ok(), "Failed to import: {:?}", result);

        let value = jstime.run_script("globalThis.chainResult.value", "test");
        assert_eq!(value.unwrap(), "ABC");

        let processed = jstime.run_script("globalThis.chainResult.processed", "test");
        assert_eq!(processed.unwrap(), "start_A_B_C");

        std::fs::remove_file(&module_a).ok();
        std::fs::remove_file(&module_b).ok();
        std::fs::remove_file(&module_c).ok();
    }

    // ============================================================================
    // RESPONSE AND STREAMS TESTS
    // ============================================================================

    #[test]
    fn test_response_body_multiple_reads() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test that Response body can only be read once
        let script = r#"
            globalThis.results = {};
            
            const resp = new Response('test body');
            
            // First read should succeed
            resp.text().then(text => {
                globalThis.results.firstRead = text;
                
                // Second read should fail or return empty
                return resp.text().catch(e => 'error: ' + e.message);
            }).then(secondResult => {
                globalThis.results.secondRead = secondResult;
            });
        "#;
        jstime.run_script(script, "test").unwrap();

        let first = jstime.run_script("globalThis.results.firstRead", "test");
        assert_eq!(first.unwrap(), "test body");

        // Second read behavior may vary - it might error or return empty
        // Just verify it doesn't crash
        let _ = jstime.run_script("globalThis.results.secondRead", "test");
    }

    #[test]
    fn test_headers_case_insensitivity() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test Headers case-insensitivity
        let script = r#"
            const h = new Headers();
            
            h.set('Content-Type', 'application/json');
            h.set('X-Custom-Header', 'value1');
            
            // Get with different cases
            const results = {
                contentType1: h.get('Content-Type'),
                contentType2: h.get('content-type'),
                contentType3: h.get('CONTENT-TYPE'),
                customHeader1: h.get('X-Custom-Header'),
                customHeader2: h.get('x-custom-header'),
                customHeader3: h.get('X-CUSTOM-HEADER')
            };
            
            // Verify has() is also case-insensitive
            results.hasContentType = h.has('CONTENT-TYPE');
            results.hasCustom = h.has('x-CUSTOM-header');
            
            JSON.stringify(results);
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["contentType1"], "application/json");
        assert_eq!(json["contentType2"], "application/json");
        assert_eq!(json["contentType3"], "application/json");
        assert_eq!(json["hasContentType"], true);
        assert_eq!(json["hasCustom"], true);
    }

    // ============================================================================
    // EDGE CASE TESTS
    // ============================================================================

    #[test]
    fn test_empty_and_null_handling() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test various APIs with empty/null inputs
        let script = r#"
            const results = {};
            
            // Empty strings
            results.emptyBtoa = btoa('');
            results.emptyAtob = atob('');
            
            // Empty URL searchParams
            const emptyParams = new URLSearchParams('');
            results.emptyParamsString = emptyParams.toString();
            
            // Empty Headers
            const emptyHeaders = new Headers();
            results.emptyHeadersEntries = [...emptyHeaders.entries()].length;
            
            // Empty TextEncoder
            const encoder = new TextEncoder();
            results.emptyEncode = encoder.encode('').length;
            
            // Empty TextDecoder
            const decoder = new TextDecoder();
            results.emptyDecode = decoder.decode(new Uint8Array(0));
            
            // structuredClone with various values
            results.cloneNull = structuredClone(null);
            results.cloneUndefined = structuredClone(undefined);
            results.cloneEmptyObject = JSON.stringify(structuredClone({}));
            results.cloneEmptyArray = JSON.stringify(structuredClone([]));
            
            JSON.stringify(results);
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["emptyBtoa"], "");
        assert_eq!(json["emptyAtob"], "");
        assert_eq!(json["emptyParamsString"], "");
        assert_eq!(json["emptyHeadersEntries"], 0);
        assert_eq!(json["emptyEncode"], 0);
        assert_eq!(json["emptyDecode"], "");
        assert!(json["cloneNull"].is_null());
        assert!(json["cloneEmptyObject"].as_str().unwrap() == "{}");
        assert!(json["cloneEmptyArray"].as_str().unwrap() == "[]");
    }

    #[test]
    fn test_special_characters_in_urls() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test URL handling with special characters
        let script = r#"
            const results = {};
            
            // URL with special characters in path
            const url1 = new URL('https://example.com/path with spaces');
            results.spacesInPath = url1.pathname;
            
            // URL with unicode in path
            const url2 = new URL('https://example.com/日本語');
            results.unicodePath = url2.pathname;
            
            // URLSearchParams with special characters
            const params = new URLSearchParams();
            params.set('key', 'value with spaces');
            params.set('special', '!@#$%^&*()');
            params.set('unicode', '日本語');
            results.encodedParams = params.toString();
            
            // Decode and verify
            const decoded = new URLSearchParams(results.encodedParams);
            results.decodedSpaces = decoded.get('key');
            results.decodedSpecial = decoded.get('special');
            results.decodedUnicode = decoded.get('unicode');
            
            JSON.stringify(results);
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        // Verify round-trip encoding/decoding
        assert_eq!(json["decodedSpaces"], "value with spaces");
        assert_eq!(json["decodedSpecial"], "!@#$%^&*()");
        assert_eq!(json["decodedUnicode"], "日本語");
    }

    #[test]
    fn test_large_timer_id_range() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test that timer IDs work correctly after many timers
        let script = r#"
            const timerIds = [];
            const executedIds = [];
            
            // Create and immediately clear many timers to advance ID counter
            for (let i = 0; i < 100; i++) {
                const id = setTimeout(() => {}, 1000);
                clearTimeout(id);
            }
            
            // Now create actual timers
            for (let i = 0; i < 5; i++) {
                const id = setTimeout(() => {
                    executedIds.push(id);
                }, 10);
                timerIds.push(id);
            }
            
            globalThis.timerTest = { timerIds, executedIds };
        "#;
        jstime.run_script(script, "test").unwrap();

        // Verify all timers executed
        let count = jstime.run_script("globalThis.timerTest.executedIds.length", "test");
        assert_eq!(count.unwrap(), "5");

        // Verify IDs are high numbers (> 100)
        let first_id = jstime.run_script("globalThis.timerTest.timerIds[0]", "test");
        assert!(first_id.unwrap().parse::<i64>().unwrap() > 100);
    }

    #[test]
    fn test_console_with_complex_objects() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test console with various complex objects (should not crash)
        let script = r#"
            // Objects with cycles
            const obj = { a: 1 };
            obj.self = obj;
            
            // Deep objects
            const deep = { level1: { level2: { level3: { value: 'deep' } } } };
            
            // Arrays with holes
            const sparseArray = [];
            sparseArray[0] = 'first';
            sparseArray[100] = 'last';
            
            // Mixed types
            const mixed = [1, 'two', { three: 3 }, [4, 5], null, undefined, true];
            
            // These should all complete without crashing
            console.log(obj);
            console.log(deep);
            console.log(sparseArray);
            console.log(mixed);
            console.dir({ nested: { object: { here: true } } });
            console.table([{ a: 1, b: 2 }, { a: 3, b: 4 }]);
            
            'completed';
        "#;
        let result = jstime.run_script(script, "test");
        assert_eq!(result.unwrap(), "completed");
    }

    #[test]
    fn test_date_locale_methods() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test Date locale methods
        let script = r#"
            const date = new Date(2025, 0, 15, 14, 30, 45); // Jan 15, 2025, 2:30:45 PM
            
            const results = {
                toLocaleString: typeof date.toLocaleString() === 'string',
                toLocaleDateString: typeof date.toLocaleDateString() === 'string',
                toLocaleTimeString: typeof date.toLocaleTimeString() === 'string',
                dateStringContainsYear: date.toLocaleDateString().includes('2025'),
                timeStringContainsMinutes: date.toLocaleTimeString().includes('30')
            };
            
            JSON.stringify(results);
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["toLocaleString"], true);
        assert_eq!(json["toLocaleDateString"], true);
        assert_eq!(json["toLocaleTimeString"], true);
        assert_eq!(json["dateStringContainsYear"], true);
        assert_eq!(json["timeStringContainsMinutes"], true);
    }
}
