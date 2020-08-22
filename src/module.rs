use std::fs;
use std::process;

use rusty_v8 as v8;

use crate::binding;
use crate::bootstrap;
use crate::js_loading;

fn resolve_callback<'a>(
    context: v8::Local<'a, v8::Context>,
    specifier: v8::Local<'a, v8::String>,
    _referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
    let mut cbs = unsafe { v8::CallbackScope::new(context) };
    let scope = &mut cbs;

    // TODO(bengl)
    // 1. Normalize to absoluate paths.
    // 2. Cache modules so they don't get re-evaluated.
    // 3. Oh wow some better error handling woops!

    let origin = js_loading::create_script_origin(scope, specifier, true);
    let filename = &specifier.to_rust_string_lossy(scope);
    let js_src = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let code = v8::String::new(scope, &js_src).unwrap();
    let source = v8::script_compiler::Source::new(code, &origin);

    v8::script_compiler::compile_module(scope, source)
}

pub fn run_js_in_scope(scope: &mut v8::HandleScope, js: &str, filepath: &str) -> String {
    let filepath = v8::String::new(scope, filepath).unwrap();
    let origin = js_loading::create_script_origin(scope, filepath, true);

    let code = v8::String::new(scope, js).unwrap();

    let tc_scope = &mut v8::TryCatch::new(scope);
    let source = v8::script_compiler::Source::new(code, &origin);
    let module = v8::script_compiler::compile_module(tc_scope, source);

    if module.is_none() {
        let exception = tc_scope.exception().unwrap();
        let msg = v8::Exception::create_message(tc_scope, exception);
        let error_message = msg.get(tc_scope).to_rust_string_lossy(tc_scope);
        eprintln!("{}", &error_message);
        return "".to_string();
    }

    let script = module.unwrap();

    let _ = script.instantiate_module(tc_scope, resolve_callback);

    if let Some(stack_trace) = tc_scope.stack_trace() {
        let result = stack_trace.to_string(tc_scope).unwrap();
        let result = result.to_string(tc_scope).unwrap();
        let result = result.to_rust_string_lossy(tc_scope);

        eprintln!("{}", result);

        return "".to_string();
    }

    let result = script.evaluate(tc_scope);

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

#[allow(dead_code)]
pub fn run(js: &str) -> String {
    run_internal(js, "jstime")
}

#[allow(dead_code)]
pub(crate) fn run_file(filepath: &str) {
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
