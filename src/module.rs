use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process;
use std::rc::Rc;

use rusty_v8 as v8;

use crate::binding;
use crate::bootstrap;
use crate::js_loading;

struct IsolateState {
    hash_to_absolute_path: HashMap<i32, String>,
    absolute_path_to_module: HashMap<String, v8::Global<v8::Module>>,
}

impl IsolateState {
    fn new() -> Self {
        IsolateState {
            hash_to_absolute_path: HashMap::new(),
            absolute_path_to_module: HashMap::new(),
        }
    }

    fn add_module(&mut self, hash: i32, filepath: String, module: v8::Global<v8::Module>) {
        self.hash_to_absolute_path.insert(hash, filepath.clone());
        self.absolute_path_to_module.insert(filepath, module);
    }

    fn get(scope: &mut v8::Isolate) -> Rc<RefCell<Self>> {
        let s = scope.get_slot::<Rc<RefCell<IsolateState>>>().unwrap();
        s.clone()
    }

    fn get_mut(scope: &mut v8::Isolate) -> Rc<RefCell<Self>> {
        let s = scope.get_slot_mut::<Rc<RefCell<IsolateState>>>().unwrap();
        s.clone()
    }
}

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

    let state = IsolateState::get(scope);
    let referrer_path = state
        .borrow()
        .hash_to_absolute_path
        .get(&hash)
        .unwrap()
        .clone();

    let requested_abs_path = normalize_path(&referrer_path, &requested_rel_path);

    if let Some(module) = state
        .borrow()
        .absolute_path_to_module
        .get(&requested_abs_path)
    {
        return Some(v8::Local::new(scope, module));
    }

    let requested_string = v8::String::new(scope, &requested_abs_path).unwrap();
    let origin = js_loading::create_script_origin(scope, requested_string, true);
    let js_src =
        fs::read_to_string(&requested_abs_path).expect("Something went wrong reading the file");
    let code = v8::String::new(scope, &js_src).unwrap();
    let source = v8::script_compiler::Source::new(code, &origin);

    let module = v8::script_compiler::compile_module(scope, source);
    if let Some(module) = module {
        let hash = module.get_identity_hash();
        let state = IsolateState::get_mut(scope);
        state
            .borrow_mut()
            .add_module(hash, requested_abs_path, v8::Global::new(scope, module));
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

    let hash = script.get_identity_hash();
    let state = IsolateState::get_mut(tc_scope);
    state
        .borrow_mut()
        .add_module(hash, filepath_string, v8::Global::new(tc_scope, script));

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
    isolate.set_slot(Rc::new(RefCell::new(IsolateState::new())));
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
