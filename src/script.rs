use std::fs;
use std::process;

use rusty_v8 as v8;

use crate::binding;
use crate::bootstrap;

pub(crate) fn run_js_in_scope(scope: &mut v8::HandleScope, js: &str) -> String {
    let code = v8::String::new(scope, js).unwrap();

    let tc_scope = &mut v8::TryCatch::new(scope);
    let script = v8::Script::compile(tc_scope, code, None);

    if script.is_none() {
        let exception = tc_scope.exception().unwrap();
        let msg = v8::Exception::create_message(tc_scope, exception);
        let error_message = msg.get(tc_scope).to_rust_string_lossy(tc_scope);
        eprintln!("{}", &error_message);
        return "".to_string();
    }

    let script = script.unwrap();

    let result = script.run(tc_scope);

    if let Some(stack_trace) = tc_scope.stack_trace() {
        let result = stack_trace.to_string(tc_scope).unwrap();
        let result = result.to_string(tc_scope).unwrap();
        let result = result.to_rust_string_lossy(tc_scope);

        eprintln!("{}", result);

        return "".to_string();
    }

    if result.is_none() {
        let exception = tc_scope.exception().unwrap();
        let msg = v8::Exception::create_message(tc_scope, exception);
        let error_message = msg.get(tc_scope).to_rust_string_lossy(tc_scope);
        eprintln!("{}", &error_message);
        return "".to_string();
    }

    let result = result.unwrap();
    let result = result.to_string(tc_scope).unwrap();
    result.to_rust_string_lossy(tc_scope)
}

pub fn run(js: &str) -> String {
    bootstrap::init();
    let isolate = &mut v8::Isolate::new(Default::default());
    let scope = &mut v8::HandleScope::new(isolate);
    let context = binding::initialize_context(scope);
    let scope = &mut v8::ContextScope::new(scope, context);
    bootstrap::set_globals(scope);
    run_js_in_scope(scope, js)
}

#[allow(dead_code)]
pub(crate) fn run_file(filepath: &str) {
    let stat = fs::metadata(filepath);
    match stat {
        Ok(_stat) => {
            let contents =
                fs::read_to_string(filepath).expect("Something went wrong reading the file");
            run(&contents);
        }
        Err(_e) => {
            eprintln!("Error: file doesn't exist");
            process::exit(1);
        }
    }
}
