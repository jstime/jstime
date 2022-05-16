use crate::IsolateState;
use std::collections::HashMap;
use std::path::Path;

pub(crate) struct ModuleMap {
    hash_to_absolute_path: HashMap<std::num::NonZeroI32, String>,
    absolute_path_to_module: HashMap<String, v8::Global<v8::Module>>,
}

impl ModuleMap {
    pub(crate) fn new() -> Self {
        Self {
            hash_to_absolute_path: HashMap::new(),
            absolute_path_to_module: HashMap::new(),
        }
    }

    fn insert(
        &mut self,
        scope: &mut v8::HandleScope,
        filepath: &str,
        module: v8::Local<v8::Module>,
    ) {
        self.hash_to_absolute_path
            .insert(module.get_identity_hash(), filepath.to_owned());
        let module = v8::Global::new(scope, module);
        self.absolute_path_to_module
            .insert(filepath.to_owned(), module);
    }
}

pub(crate) struct Loader {}

impl Loader {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn import<'a>(
        &self,
        scope: &mut v8::HandleScope<'a>,
        referrer: &str,
        specifier: &str,
    ) -> Result<v8::Local<'a, v8::Value>, v8::Local<'a, v8::Value>> {
        let scope = &mut v8::TryCatch::new(scope);
        match resolve(scope, referrer, specifier) {
            Some(m) => {
                m.instantiate_module(scope, module_resolve_callback)
                    .unwrap();
                let res = m.evaluate(scope).unwrap();
                let promise = unsafe { v8::Local::<v8::Promise>::cast(res) };
                match promise.state() {
                    v8::PromiseState::Pending => panic!(),
                    v8::PromiseState::Fulfilled => Ok(promise.result(scope)),
                    v8::PromiseState::Rejected => Err(promise.result(scope)),
                }
            }
            None => Err(scope.stack_trace().unwrap()),
        }
    }
}

fn resolve<'a>(
    scope: &mut v8::HandleScope<'a>,
    referrer: &str,
    specifier: &str,
) -> Option<v8::Local<'a, v8::Module>> {
    let state = IsolateState::get(scope);

    let requested_abs_path = normalize_path(referrer, specifier);
    if let Some(module) = state
        .borrow()
        .module_map
        .absolute_path_to_module
        .get(&requested_abs_path)
    {
        return Some(v8::Local::new(scope, module));
    }

    let requested_string = v8::String::new(scope, &requested_abs_path).unwrap();
    let origin = crate::js_loading::create_script_origin(scope, requested_string, true);
    let js_src = std::fs::read_to_string(&requested_abs_path)
        .expect("Something went wrong reading the file");
    let code = v8::String::new(scope, &js_src).unwrap();
    let source = v8::script_compiler::Source::new(code, Some(&origin));

    let module = v8::script_compiler::compile_module(scope, source);
    if let Some(module) = module {
        let state = IsolateState::get(scope);
        state
            .borrow_mut()
            .module_map
            .insert(scope, &requested_abs_path, module);
    }
    module
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

fn module_resolve_callback<'a>(
    context: v8::Local<'a, v8::Context>,
    specifier: v8::Local<'a, v8::String>,
    _import_assertions: v8::Local<'a, v8::FixedArray>,
    referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
    let scope = unsafe { &mut v8::CallbackScope::new(context) };

    let hash = referrer.get_identity_hash();

    let state = IsolateState::get(scope);
    let referrer_path = state
        .borrow()
        .module_map
        .hash_to_absolute_path
        .get(&hash)
        .unwrap()
        .to_owned();

    let requested_rel_path = specifier.to_rust_string_lossy(scope);
    resolve(scope, &referrer_path, &requested_rel_path)
}
