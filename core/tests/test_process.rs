use jstime_core as jstime;

mod common;

#[test]
fn test_process_exists() {
    let _setup_guard = common::setup();
    let options = jstime::Options {
        process_argv: vec!["jstime".to_string(), "test.js".to_string()],
    };
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script("typeof process", "test");
    assert_eq!(result.unwrap(), "object");
}

#[test]
fn test_process_env() {
    let _setup_guard = common::setup();
    let options = jstime::Options {
        process_argv: vec!["jstime".to_string()],
    };
    let mut jstime = jstime::JSTime::new(options);

    // Set an environment variable for testing
    unsafe {
        std::env::set_var("TEST_VAR", "test_value");
    }

    let result = jstime.run_script("process.env.TEST_VAR", "test");
    assert_eq!(result.unwrap(), "test_value");

    // Clean up
    unsafe {
        std::env::remove_var("TEST_VAR");
    }
}

#[test]
fn test_process_env_is_object() {
    let _setup_guard = common::setup();
    let options = jstime::Options {
        process_argv: vec!["jstime".to_string()],
    };
    let mut jstime = jstime::JSTime::new(options);

    let result = jstime.run_script("typeof process.env", "test");
    assert_eq!(result.unwrap(), "object");
}

#[test]
fn test_process_argv() {
    let _setup_guard = common::setup();
    let options = jstime::Options {
        process_argv: vec![
            "jstime".to_string(),
            "test.js".to_string(),
            "arg1".to_string(),
        ],
    };
    let mut jstime = jstime::JSTime::new(options);

    let result = jstime.run_script("Array.isArray(process.argv)", "test");
    assert_eq!(result.unwrap(), "true");

    let result = jstime.run_script("process.argv.length", "test");
    assert_eq!(result.unwrap(), "3");

    let result = jstime.run_script("process.argv[0]", "test");
    assert_eq!(result.unwrap(), "jstime");

    let result = jstime.run_script("process.argv[1]", "test");
    assert_eq!(result.unwrap(), "test.js");

    let result = jstime.run_script("process.argv[2]", "test");
    assert_eq!(result.unwrap(), "arg1");
}

#[test]
fn test_process_cwd() {
    let _setup_guard = common::setup();
    let options = jstime::Options {
        process_argv: vec!["jstime".to_string()],
    };
    let mut jstime = jstime::JSTime::new(options);

    let result = jstime.run_script("typeof process.cwd", "test");
    assert_eq!(result.unwrap(), "function");

    let result = jstime.run_script("typeof process.cwd()", "test");
    assert_eq!(result.unwrap(), "string");

    // Check that cwd returns a non-empty string
    let result = jstime.run_script("process.cwd().length > 0", "test");
    assert_eq!(result.unwrap(), "true");
}

#[test]
fn test_process_exit_function_exists() {
    let _setup_guard = common::setup();
    let options = jstime::Options {
        process_argv: vec!["jstime".to_string()],
    };
    let mut jstime = jstime::JSTime::new(options);

    let result = jstime.run_script("typeof process.exit", "test");
    assert_eq!(result.unwrap(), "function");
}

#[test]
fn test_process_argv_empty() {
    let _setup_guard = common::setup();
    let options = jstime::Options {
        process_argv: vec![],
    };
    let mut jstime = jstime::JSTime::new(options);

    let result = jstime.run_script("process.argv.length", "test");
    assert_eq!(result.unwrap(), "0");
}

#[test]
fn test_process_env_multiple_vars() {
    let _setup_guard = common::setup();
    let options = jstime::Options {
        process_argv: vec!["jstime".to_string()],
    };
    let mut jstime = jstime::JSTime::new(options);

    // Set multiple environment variables
    unsafe {
        std::env::set_var("TEST_VAR1", "value1");
        std::env::set_var("TEST_VAR2", "value2");
    }

    let result = jstime.run_script(
        "process.env.TEST_VAR1 === 'value1' && process.env.TEST_VAR2 === 'value2'",
        "test",
    );
    assert_eq!(result.unwrap(), "true");

    // Clean up
    unsafe {
        std::env::remove_var("TEST_VAR1");
        std::env::remove_var("TEST_VAR2");
    }
}

#[test]
fn test_process_cwd_matches_rust() {
    let _setup_guard = common::setup();
    let options = jstime::Options {
        process_argv: vec!["jstime".to_string()],
    };
    let mut jstime = jstime::JSTime::new(options);

    let rust_cwd = std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let result = jstime.run_script("process.cwd()", "test");
    assert_eq!(result.unwrap(), rust_cwd);
}

#[test]
fn test_process_env_special_chars() {
    let _setup_guard = common::setup();
    let options = jstime::Options {
        process_argv: vec!["jstime".to_string()],
    };
    let mut jstime = jstime::JSTime::new(options);

    // Test environment variable with special characters
    unsafe {
        std::env::set_var("TEST_SPECIAL", "value with spaces and symbols: !@#$%");
    }

    let result = jstime.run_script("process.env.TEST_SPECIAL", "test");
    assert_eq!(result.unwrap(), "value with spaces and symbols: !@#$%");

    // Clean up
    unsafe {
        std::env::remove_var("TEST_SPECIAL");
    }
}
