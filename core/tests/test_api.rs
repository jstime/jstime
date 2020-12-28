use jstime_core as jstime;

mod common;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_script() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("\"Hello, World!\"", "jstime");
        assert_eq!(result.unwrap(), "Hello, World!");
        let result = jstime.run_script("1 + 1", "jstime");
        assert_eq!(result.unwrap(), "2");
        let result = jstime.run_script("const a = 123; const b = 456; a + b;", "jstime");
        assert_eq!(result.unwrap(), "579");
        let result = jstime.run_script("a", "jstime");
        assert_eq!(result.unwrap(), "123");
    }
    #[test]
    fn run_script_error() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let err = match jstime.run_script("a", "jstime") {
            Ok(_result) => panic!(),
            Err(e) => e,
        };
        assert_eq!(
            err.to_string(),
            "ReferenceError: a is not defined\n    at jstime:1:1"
        );
        let err = match jstime.run_script("}", "jstime") {
            Ok(_result) => panic!(),
            Err(e) => e,
        };
        assert_eq!(err.to_string(), "SyntaxError: Unexpected token \'}\'");
    }
    #[test]
    fn import() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let hello_path = "./tests/fixtures/hello-world.js";
        let _result = jstime.import(&hello_path);
        let result = jstime.run_script("globalThis.hello", "jstime");
        assert_eq!(result.unwrap(), "hello world");
    }
}
