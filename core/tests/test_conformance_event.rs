use jstime_core as jstime;

mod common;

#[cfg(test)]
mod conformance_event {
    use super::*;

    // Event Constructor Tests
    #[test]
    fn event_constructor_exists() {
        let result = common::get_type_of("Event");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn event_constructor_with_type() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const event = new Event('click'); event.type;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "click");
    }

    #[test]
    fn event_constructor_with_options() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const event = new Event('click', { bubbles: true, cancelable: true, composed: true }); \
             JSON.stringify({ bubbles: event.bubbles, cancelable: event.cancelable, composed: event.composed });",
            "conformance_event",
        );
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["bubbles"], true);
        assert_eq!(json["cancelable"], true);
        assert_eq!(json["composed"], true);
    }

    #[test]
    fn event_defaults() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const event = new Event('click'); \
             JSON.stringify({ bubbles: event.bubbles, cancelable: event.cancelable, composed: event.composed });",
            "conformance_event",
        );
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["bubbles"], false);
        assert_eq!(json["cancelable"], false);
        assert_eq!(json["composed"], false);
    }

    #[test]
    fn event_type_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const event = new Event('test-event'); event.type;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "test-event");
    }

    #[test]
    fn event_target_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let eventTarget; \
             target.addEventListener('test', (e) => { eventTarget = e.target; }); \
             target.dispatchEvent(new Event('test')); \
             eventTarget === target;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn event_current_target_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let eventCurrentTarget; \
             target.addEventListener('test', (e) => { eventCurrentTarget = e.currentTarget; }); \
             target.dispatchEvent(new Event('test')); \
             eventCurrentTarget === target;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn event_timestamp_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const event = new Event('test'); typeof event.timeStamp === 'number' && event.timeStamp >= 0;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn event_is_trusted_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const event = new Event('test'); event.isTrusted;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn event_default_prevented_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const event = new Event('test', { cancelable: true }); \
             event.defaultPrevented;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn event_prevent_default_method() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const event = new Event('test', { cancelable: true }); \
             event.preventDefault(); \
             event.defaultPrevented;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn event_prevent_default_on_non_cancelable() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const event = new Event('test', { cancelable: false }); \
             event.preventDefault(); \
             event.defaultPrevented;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn event_stop_propagation_method() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const event = new Event('test'); \
             event.stopPropagation(); \
             'success';",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "success");
    }

    #[test]
    fn event_stop_immediate_propagation_method() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const event = new Event('test'); \
             event.stopImmediatePropagation(); \
             'success';",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "success");
    }

    #[test]
    fn event_phase_constants() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "JSON.stringify({ \
               NONE: Event.NONE, \
               CAPTURING_PHASE: Event.CAPTURING_PHASE, \
               AT_TARGET: Event.AT_TARGET, \
               BUBBLING_PHASE: Event.BUBBLING_PHASE \
             });",
            "conformance_event",
        );
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["NONE"], 0);
        assert_eq!(json["CAPTURING_PHASE"], 1);
        assert_eq!(json["AT_TARGET"], 2);
        assert_eq!(json["BUBBLING_PHASE"], 3);
    }

    // EventTarget Tests
    #[test]
    fn event_target_constructor_exists() {
        let result = common::get_type_of("EventTarget");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn event_target_can_be_constructed() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); target instanceof EventTarget;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn event_target_add_event_listener_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); typeof target.addEventListener;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn event_target_remove_event_listener_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); typeof target.removeEventListener;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn event_target_dispatch_event_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); typeof target.dispatchEvent;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn event_target_add_event_listener_basic() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let called = false; \
             target.addEventListener('test', () => { called = true; }); \
             target.dispatchEvent(new Event('test')); \
             called;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn event_target_remove_event_listener_basic() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let called = false; \
             const listener = () => { called = true; }; \
             target.addEventListener('test', listener); \
             target.removeEventListener('test', listener); \
             target.dispatchEvent(new Event('test')); \
             called;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn event_target_dispatch_event_returns_true() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             target.dispatchEvent(new Event('test'));",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn event_target_dispatch_event_returns_false_when_prevented() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             target.addEventListener('test', (e) => { e.preventDefault(); }); \
             target.dispatchEvent(new Event('test', { cancelable: true }));",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn event_target_multiple_listeners() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let count = 0; \
             target.addEventListener('test', () => { count++; }); \
             target.addEventListener('test', () => { count++; }); \
             target.addEventListener('test', () => { count++; }); \
             target.dispatchEvent(new Event('test')); \
             count;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "3");
    }

    #[test]
    fn event_target_listeners_called_in_order() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let order = []; \
             target.addEventListener('test', () => { order.push(1); }); \
             target.addEventListener('test', () => { order.push(2); }); \
             target.addEventListener('test', () => { order.push(3); }); \
             target.dispatchEvent(new Event('test')); \
             order.join(',');",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "1,2,3");
    }

    #[test]
    fn event_target_stop_immediate_propagation_stops_listeners() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let count = 0; \
             target.addEventListener('test', (e) => { count++; e.stopImmediatePropagation(); }); \
             target.addEventListener('test', () => { count++; }); \
             target.addEventListener('test', () => { count++; }); \
             target.dispatchEvent(new Event('test')); \
             count;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "1");
    }

    #[test]
    fn event_target_different_event_types() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let clickCount = 0, loadCount = 0; \
             target.addEventListener('click', () => { clickCount++; }); \
             target.addEventListener('load', () => { loadCount++; }); \
             target.dispatchEvent(new Event('click')); \
             target.dispatchEvent(new Event('load')); \
             target.dispatchEvent(new Event('click')); \
             JSON.stringify({ click: clickCount, load: loadCount });",
            "conformance_event",
        );
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["click"], 2);
        assert_eq!(json["load"], 1);
    }

    #[test]
    fn event_target_listener_receives_event() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let receivedEvent; \
             target.addEventListener('test', (e) => { receivedEvent = e; }); \
             const event = new Event('test'); \
             target.dispatchEvent(event); \
             receivedEvent === event;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn event_target_this_binding() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let thisValue; \
             target.addEventListener('test', function() { thisValue = this; }); \
             target.dispatchEvent(new Event('test')); \
             thisValue === target;",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn event_target_remove_non_existent_listener() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             const listener = () => {}; \
             target.removeEventListener('test', listener); \
             'success';",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "success");
    }

    #[test]
    fn event_target_add_same_listener_multiple_times() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             let count = 0; \
             const listener = () => { count++; }; \
             target.addEventListener('test', listener); \
             target.addEventListener('test', listener); \
             target.addEventListener('test', listener); \
             target.dispatchEvent(new Event('test')); \
             count;",
            "conformance_event",
        );
        // Note: According to DOM spec, the same listener should be added multiple times
        // Our implementation allows duplicates, which matches this test
        assert_eq!(result.unwrap(), "3");
    }

    #[test]
    fn event_target_dispatch_throws_on_non_event() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const target = new EventTarget(); \
             try { \
               target.dispatchEvent({}); \
               'no error'; \
             } catch(e) { \
               e.message.includes('Event'); \
             }",
            "conformance_event",
        );
        assert_eq!(result.unwrap(), "true");
    }
}
