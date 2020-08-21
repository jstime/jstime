use std::fs;
use std::process;

use rusty_v8 as v8;

use crate::binding;
use crate::bootstrap;

fn create_script_origin<'s>(
    scope: &mut v8::HandleScope<'s, ()>,
    filepath: &str,
) -> v8::ScriptOrigin<'s> {
    let resource_name = v8::String::new(scope, filepath).unwrap().into();
    let resource_line_offset = v8::Integer::new(scope, 0);
    let resource_column_offset = v8::Integer::new(scope, 0);
    let resource_is_shared_cross_origin = v8::Boolean::new(scope, false);
    let script_id = v8::Integer::new(scope, 0);
    let source_map_url = v8::String::new(scope, "").unwrap().into();
    let resource_is_opaque = v8::Boolean::new(scope, true);
    let is_wasm = v8::Boolean::new(scope, false);
    let is_module = v8::Boolean::new(scope, false);

    v8::ScriptOrigin::new(
        resource_name,
        resource_line_offset,
        resource_column_offset,
        resource_is_shared_cross_origin,
        script_id,
        source_map_url,
        resource_is_opaque,
        is_wasm,
        is_module,
    )
}

pub fn run_js_in_scope(scope: &mut v8::HandleScope, js: &str, filepath: &str) -> String {
    let origin = create_script_origin(scope, filepath);

    let code = v8::String::new(scope, js).unwrap();

    let tc_scope = &mut v8::TryCatch::new(scope);
    let script = v8::Script::compile(tc_scope, code, Some(&origin));

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
