// Heavily inspired by https://github.com/denoland/rusty_v8/blob/master/tests/test_api.rs
// https://github.com/denoland/deno/blob/master/LICENSE
// MIT License
//
// Copyright (c) 2018-2020 the Deno authors

// We only want to run jstime::init once, but we also
// don't want to run any tests until the runtime is
// rip roaring and ready to execute JS

use lazy_static::lazy_static;

use std::sync::Mutex;

use jstime_core as jstime;

lazy_static! {
    static ref INIT_LOCK: Mutex<u32> = Mutex::new(0);
}

#[must_use]
pub struct SetupGuard {}

impl Drop for SetupGuard {
    fn drop(&mut self) {
        // TODO shutdown process cleanly.
    }
}

pub fn setup() -> SetupGuard {
    let mut g = INIT_LOCK.lock().unwrap();
    *g += 1;
    if *g == 1 {
        jstime::init(None);
    }
    SetupGuard {}
}

/// Helper function to run a JavaScript script in a test environment.
/// This handles the setup, creates a JSTime instance with default options,
/// runs the script, and returns the result.
///
/// # Example
/// ```no_run
/// let result = run_test_script("1 + 1");
/// assert_eq!(result.unwrap(), "2");
/// ```
#[allow(dead_code)]
pub fn run_test_script(code: &str) -> Result<String, String> {
    let _setup_guard = setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    jstime.run_script(code, "test")
}

/// Helper function to check if a JavaScript API exists.
/// Returns true if the API exists and is not undefined, false otherwise.
///
/// # Example
/// ```no_run
/// assert!(assert_api_exists("console"));
/// assert!(assert_api_exists("setTimeout"));
/// ```
#[allow(dead_code)]
pub fn assert_api_exists(api_name: &str) -> bool {
    let script = format!("typeof {} !== 'undefined';", api_name);
    match run_test_script(&script) {
        Ok(result) => result == "true",
        Err(_) => false,
    }
}

/// Helper function to get the type of a JavaScript value or API.
/// Returns the typeof result as a String.
///
/// # Example
/// ```no_run
/// assert_eq!(get_type_of("console"), Ok("object".to_string()));
/// assert_eq!(get_type_of("setTimeout"), Ok("function".to_string()));
/// ```
#[allow(dead_code)]
pub fn get_type_of(api_name: &str) -> Result<String, String> {
    let script = format!("typeof {};", api_name);
    run_test_script(&script)
}
