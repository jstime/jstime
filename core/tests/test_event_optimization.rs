// Test for event listener performance optimizations
use jstime_core as jstime;

mod common;

#[test]
fn test_remove_listener_early_exit() {
    // Test that removing a non-existent listener doesn't rebuild the array
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script(
        "const target = new EventTarget(); \
         let count = 0; \
         const l1 = () => { count++; }; \
         const l2 = () => { count++; }; \
         const l3 = () => { count++; }; \
         target.addEventListener('test', l1); \
         target.addEventListener('test', l2); \
         // Try to remove non-existent listener - should be fast, no array rebuild
         target.removeEventListener('test', l3); \
         // Verify listeners are still intact
         target.dispatchEvent(new Event('test')); \
         count;",
        "test_event_optimization",
    );
    assert_eq!(result.unwrap(), "2"); // l1 and l2 only
}

#[test]
fn test_remove_listener_middle() {
    // Test that removing a listener from the middle works correctly
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script(
        "const target = new EventTarget(); \
         let order = []; \
         const l1 = () => { order.push(1); }; \
         const l2 = () => { order.push(2); }; \
         const l3 = () => { order.push(3); }; \
         target.addEventListener('test', l1); \
         target.addEventListener('test', l2); \
         target.addEventListener('test', l3); \
         // Remove middle listener
         target.removeEventListener('test', l2); \
         target.dispatchEvent(new Event('test')); \
         JSON.stringify(order);",
        "test_event_optimization",
    );
    assert_eq!(result.unwrap(), "[1,3]");
}

#[test]
fn test_remove_listener_first() {
    // Test removing the first listener
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script(
        "const target = new EventTarget(); \
         let order = []; \
         const l1 = () => { order.push(1); }; \
         const l2 = () => { order.push(2); }; \
         const l3 = () => { order.push(3); }; \
         target.addEventListener('test', l1); \
         target.addEventListener('test', l2); \
         target.addEventListener('test', l3); \
         target.removeEventListener('test', l1); \
         target.dispatchEvent(new Event('test')); \
         JSON.stringify(order);",
        "test_event_optimization",
    );
    assert_eq!(result.unwrap(), "[2,3]");
}

#[test]
fn test_remove_listener_last() {
    // Test removing the last listener
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script(
        "const target = new EventTarget(); \
         let order = []; \
         const l1 = () => { order.push(1); }; \
         const l2 = () => { order.push(2); }; \
         const l3 = () => { order.push(3); }; \
         target.addEventListener('test', l1); \
         target.addEventListener('test', l2); \
         target.addEventListener('test', l3); \
         target.removeEventListener('test', l3); \
         target.dispatchEvent(new Event('test')); \
         JSON.stringify(order);",
        "test_event_optimization",
    );
    assert_eq!(result.unwrap(), "[1,2]");
}

#[test]
fn test_remove_all_listeners() {
    // Test removing all listeners
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script(
        "const target = new EventTarget(); \
         let count = 0; \
         const l1 = () => { count++; }; \
         const l2 = () => { count++; }; \
         target.addEventListener('test', l1); \
         target.addEventListener('test', l2); \
         target.removeEventListener('test', l1); \
         target.removeEventListener('test', l2); \
         target.dispatchEvent(new Event('test')); \
         count;",
        "test_event_optimization",
    );
    assert_eq!(result.unwrap(), "0");
}

#[test]
fn test_initial_array_capacity() {
    // Verify that adding multiple listeners works correctly with pre-allocated capacity
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script(
        "const target = new EventTarget(); \
         let count = 0; \
         // Add more than initial capacity (2) to test growth
         target.addEventListener('test', () => { count++; }); \
         target.addEventListener('test', () => { count++; }); \
         target.addEventListener('test', () => { count++; }); \
         target.addEventListener('test', () => { count++; }); \
         target.dispatchEvent(new Event('test')); \
         count;",
        "test_event_optimization",
    );
    assert_eq!(result.unwrap(), "4");
}
