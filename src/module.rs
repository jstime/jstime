use core::cell::{Ref, RefMut};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process;

use rusty_v8 as v8;

use crate::binding;
use crate::bootstrap;
use crate::js_loading;

fn normalize_path(referrer_path: &str, requested: &str) -> String {
    let req_path = Path::new(requested);
    if req_path.is_absolute() {
        return requested.to_string();
    }
    let ref_dir = Path::new(referrer_path).parent().unwrap();
    let normalized = ref_dir.join(req_path).canonicalize();
    normalized.unwrap().to_string_lossy().into()
}

fn resolve_callback<'a>(
    context: v8::Local<'a, v8::Context>,
    specifier: v8::Local<'a, v8::String>,
    referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
    let mut cbs = unsafe { v8::CallbackScope::new(context) };
    let scope = &mut cbs;

    let hash = referrer.get_identity_hash();
    let requested_rel_path = specifier.to_rust_string_lossy(scope);
    let referrer_path = {
        let path_map: Ref<HashMap<i32, String>> = scope.get_slot().unwrap();
        path_map.get(&hash).unwrap().clone()
    };

    let requested_abs_path = normalize_path(&referrer_path, &requested_rel_path);

    // TODO(bengl)
    // 1. Cache modules so they don't get re-evaluated.
    // 2. Oh wow some better error handling woops!

    let requested_string = v8::String::new(scope, &requested_abs_path).unwrap();
    let origin = js_loading::create_script_origin(scope, requested_string, true);
    let js_src =
        fs::read_to_string(&requested_abs_path).expect("Something went wrong reading the file");
    let code = v8::String::new(scope, &js_src).unwrap();
    let source = v8::script_compiler::Source::new(code, &origin);

    let module = v8::script_compiler::compile_module(scope, source);
    if let Some(unwrapped_module) = module {
        let mut path_map: RefMut<HashMap<i32, String>> = scope.get_slot_mut().unwrap();
        let hash = unwrapped_module.get_identity_hash();
        path_map.insert(hash, requested_abs_path.clone());
    }
    module
}

pub fn run_js_in_scope(scope: &mut v8::HandleScope, js: &str, filepath: &str) -> String {
    let curr: String = std::env::current_dir()
        .unwrap()
        .join("current")
        .to_string_lossy()
        .into();
    let filepath_string = normalize_path(&curr, filepath);
    let filepath = v8::String::new(scope, &filepath_string).unwrap();
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

    {
        let mut path_map: RefMut<HashMap<i32, String>> = tc_scope.get_slot_mut().unwrap();
        let hash = script.get_identity_hash();
        path_map.insert(hash, filepath_string.clone());
    }

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
    let hash_to_absolute_path: HashMap<i32, String> = HashMap::new();
    isolate.set_slot(hash_to_absolute_path);
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
