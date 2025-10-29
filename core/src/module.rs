use crate::IsolateState;
use rustc_hash::FxHashMap;
use std::path::Path;

pub(crate) struct ModuleMap {
    hash_to_absolute_path: FxHashMap<std::num::NonZeroI32, String>,
    absolute_path_to_module: FxHashMap<String, v8::Global<v8::Module>>,
}

impl ModuleMap {
    pub(crate) fn new() -> Self {
        Self {
            hash_to_absolute_path: FxHashMap::default(),
            absolute_path_to_module: FxHashMap::default(),
        }
    }

    fn insert(&mut self, isolate: &v8::Isolate, filepath: &str, module: v8::Local<v8::Module>) {
        self.hash_to_absolute_path
            .insert(module.get_identity_hash(), filepath.to_owned());
        let module = v8::Global::new(isolate, module);
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
        scope: &mut v8::PinScope<'a, '_>,
        referrer: &str,
        specifier: &str,
    ) -> Result<v8::Local<'a, v8::Value>, v8::Local<'a, v8::Value>> {
        v8::tc_scope!(let tc, scope);
        match resolve(tc, referrer, specifier) {
            Some(m) => {
                m.instantiate_module(tc, module_resolve_callback).unwrap();
                let res = m.evaluate(tc).unwrap();
                let promise = unsafe { v8::Local::<v8::Promise>::cast_unchecked(res) };
                match promise.state() {
                    v8::PromiseState::Pending => panic!(),
                    v8::PromiseState::Fulfilled => Ok(promise.result(tc)),
                    v8::PromiseState::Rejected => {
                        // Throw the rejected promise value as an exception so it can be
                        // properly formatted with source location and stack trace
                        tc.throw_exception(promise.result(tc));
                        Err(promise.result(tc))
                    }
                }
            }
            None => {
                // Check if we have a caught exception
                if tc.has_caught() {
                    Err(tc.exception().unwrap())
                } else {
                    // Error was caught during resolve, create a generic error
                    let msg = v8::String::new(tc, "Module import failed").unwrap();
                    let exception = v8::Exception::error(tc, msg);
                    Err(exception)
                }
            }
        }
    }
}

#[inline]
fn resolve<'a>(
    scope: &mut v8::PinScope<'a, '_>,
    referrer: &str,
    specifier: &str,
) -> Option<v8::Local<'a, v8::Module>> {
    let isolate: &mut v8::Isolate = scope;
    let state = IsolateState::get(isolate);

    // Handle node: prefix for built-in modules
    if let Some(module_name) = specifier.strip_prefix("node:") {
        return resolve_builtin_module(scope, module_name);
    }

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

    // Check if this is a JSON file
    let is_json = requested_abs_path.ends_with(".json");

    let js_src = if is_json {
        // For JSON files, read the content and wrap it in a module that exports it as default
        let json_content = std::fs::read_to_string(&requested_abs_path)
            .expect("Something went wrong reading the JSON file");
        // Create a synthetic module that exports the JSON as the default export
        format!("export default {};", json_content)
    } else {
        std::fs::read_to_string(&requested_abs_path).expect("Something went wrong reading the file")
    };

    let code = v8::String::new(scope, &js_src).unwrap();
    let mut source = v8::script_compiler::Source::new(code, Some(&origin));

    // Compile the module - errors will be thrown as exceptions
    let (module, error_msg) = {
        v8::tc_scope!(let tc, scope);
        let module = v8::script_compiler::compile_module(tc, &mut source);

        if let Some(module) = module {
            (Some(module), None)
        } else if tc.has_caught() {
            // Format the compilation error as a string
            let error_message = crate::error::format_exception(tc);
            (None, Some(error_message))
        } else {
            (None, None)
        }
    };

    if let Some(module) = module {
        let isolate: &mut v8::Isolate = scope;
        let state = IsolateState::get(isolate);
        state
            .borrow_mut()
            .module_map
            .insert(isolate, &requested_abs_path, module);
        Some(module)
    } else if let Some(error_msg) = error_msg {
        // Now create a new error with our formatted message
        let msg = v8::String::new(scope, &error_msg).unwrap();
        let exception = v8::Exception::error(scope, msg);
        scope.throw_exception(exception);
        None
    } else {
        None
    }
}

fn resolve_builtin_module<'a>(
    scope: &mut v8::PinScope<'a, '_>,
    module_name: &str,
) -> Option<v8::Local<'a, v8::Module>> {
    let isolate: &mut v8::Isolate = scope;
    let state = IsolateState::get(isolate);

    // Use a synthetic path for built-in modules
    let synthetic_path = format!("node:{}", module_name);

    // Check if already loaded
    if let Some(module) = state
        .borrow()
        .module_map
        .absolute_path_to_module
        .get(&synthetic_path)
    {
        return Some(v8::Local::new(scope, module));
    }

    // Generate ES module code that exports from globalThis.__node_modules
    let js_src = format!(
        r#"
        const mod = globalThis.__node_modules['node:{}'];
        if (!mod) {{
            throw new Error('Built-in module not found: {}');
        }}
        export const {{ readFile, readdir, writeFile, appendFile, mkdir, rmdir, unlink, rename, copyFile, stat, access, rm, truncate, realpath, chmod, mkdtemp, readlink, symlink, lstat, chown, utimes, constants }} = mod;
        export default mod;
        "#,
        module_name, module_name
    );

    let requested_string = v8::String::new(scope, &synthetic_path).unwrap();
    let origin = crate::js_loading::create_script_origin(scope, requested_string, true);
    let code = v8::String::new(scope, &js_src).unwrap();
    let mut source = v8::script_compiler::Source::new(code, Some(&origin));

    let module = v8::script_compiler::compile_module(scope, &mut source);
    if let Some(module) = module {
        let isolate: &mut v8::Isolate = scope;
        let state = IsolateState::get(isolate);
        state
            .borrow_mut()
            .module_map
            .insert(isolate, &synthetic_path, module);
    }
    module
}

#[inline]
fn normalize_path(referrer_path: &str, requested: &str) -> String {
    let req_path = Path::new(requested);
    if req_path.is_absolute() {
        return requested.to_string();
    }
    let ref_dir = Path::new(referrer_path).parent().unwrap();
    let normalized = ref_dir.join(req_path).canonicalize();
    normalized.unwrap().to_str().unwrap().to_string()
}

fn module_resolve_callback<'a>(
    context: v8::Local<'a, v8::Context>,
    specifier: v8::Local<'a, v8::String>,
    _import_assertions: v8::Local<'a, v8::FixedArray>,
    referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
    v8::callback_scope!(unsafe let scope, context);

    let hash = referrer.get_identity_hash();

    let isolate: &mut v8::Isolate = scope;
    let state = IsolateState::get(isolate);

    // Clone referrer_path only once and reuse it
    let referrer_path = state
        .borrow()
        .module_map
        .hash_to_absolute_path
        .get(&hash)
        .cloned()
        .unwrap();

    let isolate: &v8::Isolate = scope;
    let requested_rel_path = specifier.to_rust_string_lossy(isolate);
    resolve(scope, &referrer_path, &requested_rel_path)
}

pub(crate) unsafe extern "C" fn host_initialize_import_meta_object_callback(
    context: v8::Local<v8::Context>,
    module: v8::Local<v8::Module>,
    meta: v8::Local<v8::Object>,
) {
    v8::callback_scope!(unsafe let scope, context);

    let hash = module.get_identity_hash();
    let isolate: &mut v8::Isolate = scope;
    let state = IsolateState::get(isolate);

    let module_path = state
        .borrow()
        .module_map
        .hash_to_absolute_path
        .get(&hash)
        .cloned();

    if let Some(module_path) = module_path {
        // Convert file path to file:// URL
        let url = format!("file://{}", module_path);
        let url_key = v8::String::new(scope, "url").unwrap();
        let url_value = v8::String::new(scope, &url).unwrap();
        meta.set(scope, url_key.into(), url_value.into());
    }
}
