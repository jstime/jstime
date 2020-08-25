use std::fs;
use std::process;

use rusty_v8 as v8;

use crate::binding;
use crate::bootstrap;
use crate::js_loading;

pub(crate) fn run_js_in_scope_internal<'s>(
    scope: &mut v8::HandleScope<'s>,
    js: &str,
    filepath: &str,
) -> Option<v8::Local<'s, v8::Value>> {
    let filepath = v8::String::new(scope, filepath).unwrap();
    let origin = js_loading::create_script_origin(scope, filepath, false);

    let code = v8::String::new(scope, js).unwrap();

    v8::Script::compile(scope, code, Some(&origin)).and_then(|script| script.run(scope))
}

pub fn run_js_in_scope(scope: &mut v8::HandleScope, js: &str, filepath: &str) -> String {
    let tc_scope = &mut v8::TryCatch::new(scope);

    let result = run_js_in_scope_internal(tc_scope, js, filepath);

    if let Some(stack_trace) = tc_scope.stack_trace() {
        let result = stack_trace.to_string(tc_scope).unwrap();
        let result = result.to_string(tc_scope).unwrap();
        let result = result.to_rust_string_lossy(tc_scope);

        eprintln!("{}", result);

        return "".to_string();
    }

    let result = result.unwrap();
    let result = result.to_string(tc_scope).unwrap();
    result.to_rust_string_lossy(tc_scope)
}

fn run_internal(js: &str, filepath: &str) -> String {
    bootstrap::init();
    let isolate = &mut v8::Isolate::new(Default::default());
    let scope = &mut v8::HandleScope::new(isolate);
    let context = binding::initialize_context(scope);
    let scope = &mut v8::ContextScope::new(scope, context);
    bootstrap::set_globals(scope);

    run_js_in_scope(scope, js, filepath)
}

pub fn run(js: &str) -> String {
    run_internal(js, "jstime")
}

pub fn run_file(filepath: &str) {
    let stat = fs::metadata(filepath);
    match stat {
        Ok(_stat) => {
            let contents =
                fs::read_to_string(filepath).expect("Something went wrong reading the file");
            run_internal(&contents, filepath);
        }
        Err(_e) => {
            eprintln!("Error: file doesn't exist");
            process::exit(1);
        }
    }
}
