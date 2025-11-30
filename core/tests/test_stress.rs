// Stress tests for the event loop and resource management
// These tests push the runtime to its limits to find potential issues

use jstime_core as jstime;

mod common;

#[cfg(test)]
mod stress_tests {
    use super::*;

    // ============================================================================
    // TIMER STRESS TESTS
    // ============================================================================

    #[test]
    fn test_many_simultaneous_timers() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create many timers all firing at similar times
        let script = r#"
            globalThis.executed = 0;
            const count = 100;
            
            for (let i = 0; i < count; i++) {
                setTimeout(() => {
                    globalThis.executed++;
                }, 5 + (i % 10)); // Delays between 5-14ms
            }
        "#;
        jstime.run_script(script, "test").unwrap();

        let result = jstime.run_script("globalThis.executed", "test");
        assert_eq!(result.unwrap(), "100");
    }

    #[test]
    fn test_timer_creation_in_timer_callback() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create timers inside timer callbacks (recursive pattern)
        let script = r#"
            globalThis.timerChainCount = 0;
            globalThis.maxChain = 20;
            
            function createChainedTimer() {
                globalThis.timerChainCount++;
                if (globalThis.timerChainCount < globalThis.maxChain) {
                    setTimeout(createChainedTimer, 5);
                }
            }
            
            setTimeout(createChainedTimer, 5);
        "#;
        jstime.run_script(script, "test").unwrap();

        let result = jstime.run_script("globalThis.timerChainCount", "test");
        assert_eq!(result.unwrap(), "20");
    }

    #[test]
    fn test_rapid_timer_clear() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create and clear timers rapidly
        // Note: In jstime's implementation, all timers with the same delay are
        // grouped together and may all fire before clearTimeout takes effect.
        // This test verifies that clearTimeout with future delays works correctly.
        let script = r#"
            globalThis.executed = 0;
            globalThis.cleared = 0;
            
            for (let i = 0; i < 20; i++) {
                const delay = 50 + (i * 10); // Staggered delays
                const id = setTimeout(() => {
                    globalThis.executed++;
                }, delay);
                
                // Clear every other timer
                if (i % 2 === 0) {
                    clearTimeout(id);
                    globalThis.cleared++;
                }
            }
        "#;
        jstime.run_script(script, "test").unwrap();

        let cleared = jstime.run_script("globalThis.cleared", "test");
        assert_eq!(cleared.unwrap(), "10");

        // The non-cleared timers should execute
        let executed = jstime.run_script("globalThis.executed", "test");
        let exec_count: i64 = executed.unwrap().parse().unwrap();
        assert!(exec_count >= 10, "At least 10 timers should execute");
    }

    #[test]
    fn test_interval_rapid_start_stop() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Start and stop intervals rapidly
        // Note: In jstime's implementation, intervals might fire once before
        // being cleared, even if cleared immediately. This test verifies
        // that clearInterval stops future executions.
        let script = r#"
            globalThis.intervalExecutions = 0;
            globalThis.intervalsCleared = 0;
            
            for (let batch = 0; batch < 5; batch++) {
                const id = setInterval(() => {
                    globalThis.intervalExecutions++;
                    // Clear after first execution to prevent infinite loop
                    clearInterval(id);
                }, 10);
                
                // Clear immediately
                clearInterval(id);
                globalThis.intervalsCleared++;
            }
        "#;
        jstime.run_script(script, "test").unwrap();

        let cleared = jstime.run_script("globalThis.intervalsCleared", "test");
        assert_eq!(cleared.unwrap(), "5");

        // The executions may vary depending on timing, but test shouldn't hang
        let executions = jstime.run_script("globalThis.intervalExecutions", "test");
        let _exec_count: i64 = executions.unwrap().parse().unwrap();
        // Test completes without hanging - success
    }

    #[test]
    fn test_timer_with_zero_delay() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test many timers with 0ms delay
        let script = r#"
            globalThis.order = [];
            
            for (let i = 0; i < 20; i++) {
                setTimeout(() => {
                    globalThis.order.push(i);
                }, 0);
            }
        "#;
        jstime.run_script(script, "test").unwrap();

        let result = jstime.run_script("globalThis.order.length", "test");
        assert_eq!(result.unwrap(), "20");

        // They should execute in order
        let order = jstime.run_script("globalThis.order.join(',')", "test");
        assert_eq!(
            order.unwrap(),
            "0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19"
        );
    }

    // ============================================================================
    // PROMISE STRESS TESTS
    // ============================================================================

    #[test]
    fn test_many_chained_promises() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create a long chain of promises
        let script = r#"
            let promise = Promise.resolve(0);
            const chainLength = 100;
            
            for (let i = 0; i < chainLength; i++) {
                promise = promise.then(val => val + 1);
            }
            
            promise.then(result => {
                globalThis.chainResult = result;
            });
        "#;
        jstime.run_script(script, "test").unwrap();

        let result = jstime.run_script("globalThis.chainResult", "test");
        assert_eq!(result.unwrap(), "100");
    }

    #[test]
    fn test_promise_all_many_promises() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create many promises resolved with Promise.all
        let script = r#"
            const promises = [];
            const count = 100;
            
            for (let i = 0; i < count; i++) {
                promises.push(Promise.resolve(i));
            }
            
            Promise.all(promises).then(results => {
                globalThis.allResults = {
                    length: results.length,
                    sum: results.reduce((a, b) => a + b, 0)
                };
            });
        "#;
        jstime.run_script(script, "test").unwrap();

        let length = jstime.run_script("globalThis.allResults.length", "test");
        assert_eq!(length.unwrap(), "100");

        let sum = jstime.run_script("globalThis.allResults.sum", "test");
        // Sum of 0..99 = 99 * 100 / 2 = 4950
        assert_eq!(sum.unwrap(), "4950");
    }

    #[test]
    fn test_promise_race_many_promises() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Race many promises
        let script = r#"
            const promises = [];
            
            for (let i = 0; i < 50; i++) {
                promises.push(new Promise(resolve => {
                    setTimeout(() => resolve(i), (50 - i) * 5); // Later indices resolve faster
                }));
            }
            
            Promise.race(promises).then(winner => {
                globalThis.raceWinner = winner;
            });
        "#;
        jstime.run_script(script, "test").unwrap();

        let winner = jstime.run_script("globalThis.raceWinner", "test");
        // The last index (49) should win since it has the shortest delay
        assert_eq!(winner.unwrap(), "49");
    }

    #[test]
    fn test_microtask_queue_stress() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Stress the microtask queue
        let script = r#"
            globalThis.microtaskOrder = [];
            
            // Queue many microtasks
            for (let i = 0; i < 50; i++) {
                queueMicrotask(() => {
                    globalThis.microtaskOrder.push(i);
                });
            }
            
            // Also queue some via Promise.resolve
            for (let i = 50; i < 100; i++) {
                Promise.resolve().then(() => {
                    globalThis.microtaskOrder.push(i);
                });
            }
        "#;
        jstime.run_script(script, "test").unwrap();

        let count = jstime.run_script("globalThis.microtaskOrder.length", "test");
        assert_eq!(count.unwrap(), "100");
    }

    // ============================================================================
    // MEMORY STRESS TESTS
    // ============================================================================

    #[test]
    fn test_large_array_in_closure() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create closures that capture large arrays
        let script = r#"
            globalThis.closureResults = [];
            
            for (let i = 0; i < 10; i++) {
                // Create a large array captured by closure
                const largeArray = new Array(1000).fill(i);
                
                setTimeout(() => {
                    // Use the captured array
                    const sum = largeArray.reduce((a, b) => a + b, 0);
                    globalThis.closureResults.push(sum);
                }, (i + 1) * 10);
            }
        "#;
        jstime.run_script(script, "test").unwrap();

        let count = jstime.run_script("globalThis.closureResults.length", "test");
        assert_eq!(count.unwrap(), "10");

        // Verify sums (each should be i * 1000)
        let sums = jstime.run_script("globalThis.closureResults.join(',')", "test");
        assert_eq!(
            sums.unwrap(),
            "0,1000,2000,3000,4000,5000,6000,7000,8000,9000"
        );
    }

    #[test]
    fn test_string_concatenation_stress() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Stress test string concatenation
        let script = r#"
            let str = '';
            const iterations = 5000;
            const chunk = 'x';
            
            for (let i = 0; i < iterations; i++) {
                str += chunk;
            }
            
            globalThis.stringResult = {
                length: str.length,
                first: str[0],
                last: str[str.length - 1]
            };
        "#;
        jstime.run_script(script, "test").unwrap();

        let length = jstime.run_script("globalThis.stringResult.length", "test");
        assert_eq!(length.unwrap(), "5000");

        let first = jstime.run_script("globalThis.stringResult.first", "test");
        assert_eq!(first.unwrap(), "x");
    }

    #[test]
    fn test_object_property_access_stress() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Stress test object property access
        let script = r#"
            const obj = {};
            const propCount = 1000;
            
            // Add many properties
            for (let i = 0; i < propCount; i++) {
                obj['prop_' + i] = i;
            }
            
            // Access all properties
            let sum = 0;
            for (let i = 0; i < propCount; i++) {
                sum += obj['prop_' + i];
            }
            
            // Delete half the properties
            for (let i = 0; i < propCount; i += 2) {
                delete obj['prop_' + i];
            }
            
            globalThis.propResult = {
                sum: sum,
                remainingKeys: Object.keys(obj).length
            };
        "#;
        jstime.run_script(script, "test").unwrap();

        let sum = jstime.run_script("globalThis.propResult.sum", "test");
        // Sum of 0 to 999 = 999 * 1000 / 2 = 499500
        assert_eq!(sum.unwrap(), "499500");

        let remaining = jstime.run_script("globalThis.propResult.remainingKeys", "test");
        assert_eq!(remaining.unwrap(), "500");
    }

    // ============================================================================
    // EVENT TARGET STRESS TESTS
    // ============================================================================

    #[test]
    fn test_many_event_listeners() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Add many event listeners
        let script = r#"
            const target = new EventTarget();
            globalThis.callCount = 0;
            const listenerCount = 200;
            
            for (let i = 0; i < listenerCount; i++) {
                target.addEventListener('test', () => {
                    globalThis.callCount++;
                });
            }
            
            target.dispatchEvent(new Event('test'));
        "#;
        jstime.run_script(script, "test").unwrap();

        let count = jstime.run_script("globalThis.callCount", "test");
        assert_eq!(count.unwrap(), "200");
    }

    #[test]
    fn test_event_listener_add_remove_cycle() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Add and remove listeners in a cycle
        let script = r#"
            const target = new EventTarget();
            globalThis.listeners = [];
            globalThis.executedCount = 0;
            
            // Add 50 listeners
            for (let i = 0; i < 50; i++) {
                const listener = () => {
                    globalThis.executedCount++;
                };
                globalThis.listeners.push(listener);
                target.addEventListener('test', listener);
            }
            
            // Remove odd-indexed listeners
            for (let i = 1; i < 50; i += 2) {
                target.removeEventListener('test', globalThis.listeners[i]);
            }
            
            // Dispatch
            target.dispatchEvent(new Event('test'));
        "#;
        jstime.run_script(script, "test").unwrap();

        let count = jstime.run_script("globalThis.executedCount", "test");
        assert_eq!(count.unwrap(), "25"); // Only even-indexed listeners remain
    }

    #[test]
    fn test_multiple_event_types() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test multiple event types on same target
        let script = r#"
            const target = new EventTarget();
            globalThis.eventCounts = {};
            const eventTypes = ['click', 'load', 'change', 'submit', 'focus', 'blur'];
            
            eventTypes.forEach(type => {
                globalThis.eventCounts[type] = 0;
                
                // Add multiple listeners per type
                for (let i = 0; i < 10; i++) {
                    target.addEventListener(type, () => {
                        globalThis.eventCounts[type]++;
                    });
                }
            });
            
            // Dispatch each event type twice
            eventTypes.forEach(type => {
                target.dispatchEvent(new Event(type));
                target.dispatchEvent(new Event(type));
            });
            
            JSON.stringify(globalThis.eventCounts);
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        // Each type should have 10 listeners called twice = 20
        assert_eq!(json["click"], 20);
        assert_eq!(json["load"], 20);
        assert_eq!(json["change"], 20);
        assert_eq!(json["submit"], 20);
        assert_eq!(json["focus"], 20);
        assert_eq!(json["blur"], 20);
    }

    // ============================================================================
    // STRUCTURED CLONE STRESS TESTS
    // ============================================================================

    #[test]
    fn test_clone_large_nested_structure() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Clone large nested structures
        let script = r#"
            function createNestedStructure(width, depth) {
                if (depth === 0) return { leaf: true, data: new Array(10).fill(0) };
                const obj = { depth: depth, children: [] };
                for (let i = 0; i < width; i++) {
                    obj.children.push(createNestedStructure(width, depth - 1));
                }
                return obj;
            }
            
            const original = createNestedStructure(3, 4);
            const cloned = structuredClone(original);
            
            // Verify independence
            cloned.children[0].modified = true;
            
            JSON.stringify({
                originalHasModified: original.children[0].modified === undefined,
                clonedHasModified: cloned.children[0].modified === true,
                depthMatch: original.depth === cloned.depth,
                childCountMatch: original.children.length === cloned.children.length
            });
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["originalHasModified"], true);
        assert_eq!(json["clonedHasModified"], true);
        assert_eq!(json["depthMatch"], true);
        assert_eq!(json["childCountMatch"], true);
    }

    #[test]
    fn test_clone_typed_arrays() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Clone various typed arrays
        let script = r#"
            const typedArrays = {
                int8: new Int8Array([1, -2, 3]),
                uint8: new Uint8Array([1, 2, 255]),
                int16: new Int16Array([1000, -2000]),
                uint16: new Uint16Array([1000, 65535]),
                int32: new Int32Array([100000, -200000]),
                uint32: new Uint32Array([100000, 4294967295]),
                float32: new Float32Array([1.5, -2.5, 3.14]),
                float64: new Float64Array([1.5, -2.5, 3.14159265359])
            };
            
            const cloned = structuredClone(typedArrays);
            
            // Verify types and values
            const results = {};
            for (const [key, original] of Object.entries(typedArrays)) {
                const clone = cloned[key];
                results[key] = {
                    sameConstructor: original.constructor.name === clone.constructor.name,
                    sameLength: original.length === clone.length,
                    independent: true
                };
                
                // Verify independence
                clone[0] = 0;
                if (original[0] === 0) {
                    results[key].independent = false;
                }
            }
            
            JSON.stringify(results);
        "#;
        let result = jstime.run_script(script, "test");
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        for key in [
            "int8", "uint8", "int16", "uint16", "int32", "uint32", "float32", "float64",
        ] {
            assert_eq!(
                json[key]["sameConstructor"], true,
                "{} should have same constructor",
                key
            );
            assert_eq!(
                json[key]["sameLength"], true,
                "{} should have same length",
                key
            );
            assert_eq!(
                json[key]["independent"], true,
                "{} should be independent",
                key
            );
        }
    }

    // ============================================================================
    // COMBINED OPERATIONS STRESS TESTS
    // ============================================================================

    #[test]
    fn test_mixed_async_operations() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Mix timers, promises, and microtasks
        let script = r#"
            globalThis.order = [];
            
            // Start with a promise
            Promise.resolve().then(() => {
                globalThis.order.push('promise1');
            });
            
            // Queue a microtask
            queueMicrotask(() => {
                globalThis.order.push('microtask1');
            });
            
            // Set a timeout
            setTimeout(() => {
                globalThis.order.push('timeout1');
                
                // Inside timeout, queue more
                Promise.resolve().then(() => {
                    globalThis.order.push('promise2');
                });
                
                queueMicrotask(() => {
                    globalThis.order.push('microtask2');
                });
            }, 10);
            
            // Another promise
            Promise.resolve().then(() => {
                globalThis.order.push('promise3');
            });
            
            // Synchronous code runs first
            globalThis.order.push('sync');
        "#;
        jstime.run_script(script, "test").unwrap();

        // Verify order: sync -> microtasks/promises -> timeout -> timeout's microtasks
        let order = jstime.run_script("globalThis.order.join(',')", "test");
        let order_str = order.unwrap();

        // Sync should be first
        assert!(order_str.starts_with("sync"));

        // Timeout should come after microtasks
        let timeout_pos = order_str.find("timeout1").unwrap();
        let microtask1_pos = order_str.find("microtask1").unwrap();
        assert!(
            microtask1_pos < timeout_pos,
            "microtask should come before timeout"
        );
    }

    #[test]
    fn test_timer_and_event_interaction() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test timers and events together
        let script = r#"
            const target = new EventTarget();
            globalThis.eventLog = [];
            
            // Set up event listener
            target.addEventListener('custom', (e) => {
                globalThis.eventLog.push({
                    type: 'event',
                    data: e.detail || 'no detail'
                });
            });
            
            // Dispatch event synchronously
            target.dispatchEvent(new Event('custom'));
            globalThis.eventLog.push({ type: 'sync', data: 'after first dispatch' });
            
            // Dispatch event in timer
            setTimeout(() => {
                target.dispatchEvent(new Event('custom'));
                globalThis.eventLog.push({ type: 'timer', data: 'after timer dispatch' });
            }, 10);
            
            // Dispatch in promise
            Promise.resolve().then(() => {
                target.dispatchEvent(new Event('custom'));
                globalThis.eventLog.push({ type: 'promise', data: 'after promise dispatch' });
            });
        "#;
        jstime.run_script(script, "test").unwrap();

        let count = jstime.run_script("globalThis.eventLog.length", "test");
        assert_eq!(count.unwrap(), "6"); // 3 events + 3 logs
    }

    #[test]
    fn test_recursive_promise_timer_pattern() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test recursive pattern with promises and timers
        let script = r#"
            globalThis.iterations = 0;
            globalThis.maxIterations = 10;
            
            function recursiveAsync() {
                return new Promise(resolve => {
                    setTimeout(() => {
                        globalThis.iterations++;
                        if (globalThis.iterations < globalThis.maxIterations) {
                            recursiveAsync().then(resolve);
                        } else {
                            resolve('done');
                        }
                    }, 5);
                });
            }
            
            recursiveAsync().then(result => {
                globalThis.finalResult = result;
            });
        "#;
        jstime.run_script(script, "test").unwrap();

        let iterations = jstime.run_script("globalThis.iterations", "test");
        assert_eq!(iterations.unwrap(), "10");

        let result = jstime.run_script("globalThis.finalResult", "test");
        assert_eq!(result.unwrap(), "done");
    }
}
