use jstime_core as jstime;

mod common;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settimeout() {
        let result = common::get_type_of("globalThis.setTimeout");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn test_setinterval() {
        let result = common::get_type_of("globalThis.setInterval");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn test_cleartimeout() {
        let result = common::get_type_of("globalThis.clearTimeout");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn test_clearinterval() {
        let result = common::get_type_of("globalThis.clearInterval");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn test_settimeout_execution() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        // Set up the variable
        jstime
            .run_script("globalThis.timerResult = 'initial';", "jstime")
            .unwrap();
        // Queue the timeout
        jstime
            .run_script(
                "setTimeout(() => { globalThis.timerResult = 'timeout executed'; }, 10);",
                "jstime",
            )
            .unwrap();
        // Check the result after event loop completes
        let result = jstime.run_script("globalThis.timerResult;", "jstime");
        assert_eq!(result.unwrap(), "timeout executed");
    }

    #[test]
    fn test_setinterval_with_clear() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let script = r#"
            globalThis.count = 0;
            const id = setInterval(() => {
                globalThis.count++;
                if (globalThis.count >= 3) {
                    clearInterval(id);
                }
            }, 10);
        "#;
        jstime.run_script(script, "jstime").unwrap();
        // Check count after intervals execute
        let result = jstime.run_script("globalThis.count;", "jstime");
        assert_eq!(result.unwrap(), "3");
    }

    #[test]
    fn test_multiple_timeouts() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let script = r#"
            globalThis.results = [];
            setTimeout(() => { globalThis.results.push(1); }, 50);
            setTimeout(() => { globalThis.results.push(2); }, 20);
            setTimeout(() => { globalThis.results.push(3); }, 10);
        "#;
        jstime.run_script(script, "jstime").unwrap();
        // Check results after timeouts execute
        let result = jstime.run_script("globalThis.results.join(',');", "jstime");
        assert_eq!(result.unwrap(), "3,2,1");
    }
}
