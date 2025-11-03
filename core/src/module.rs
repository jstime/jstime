use crate::IsolateState;
use lazy_static::lazy_static;
use rustc_hash::FxHashMap;
use std::collections::HashSet;
use std::path::Path;
use std::sync::RwLock;

// Global source code cache shared across all JSTime instances
// Maps absolute file path to source code content
//
// This cache improves:
// - Startup time: Avoids repeated file I/O for commonly imported modules
// - Memory efficiency: Source code is shared across JSTime instances
//
// Note: The cache does not automatically invalidate when files change.
// This is acceptable for most use cases where modules don't change during runtime.
// For development scenarios where file changes are expected, consider restarting
// the application or using the clear_source_cache() function.
lazy_static! {
    static ref SOURCE_CACHE: RwLock<FxHashMap<String, String>> = RwLock::new(FxHashMap::default());
}

/// Clear the global source code cache.
/// This is useful in testing or development scenarios where modules may change.
#[allow(dead_code)]
pub(crate) fn clear_source_cache() {
    SOURCE_CACHE.write().unwrap().clear();
}

/// Read source code from file, using cache if available
fn read_source_cached(path: &str) -> std::io::Result<String> {
    // Try to read from cache first
    {
        let cache = SOURCE_CACHE.read().unwrap();
        if let Some(source) = cache.get(path) {
            return Ok(source.clone());
        }
    }

    // Not in cache, read from file
    let source = std::fs::read_to_string(path)?;

    // Store in cache
    {
        let mut cache = SOURCE_CACHE.write().unwrap();
        cache.insert(path.to_string(), source.clone());
    }

    Ok(source)
}

/// Extract import specifiers from JavaScript source code.
///
/// This is a best-effort extraction that handles common import patterns.
/// It uses simple string matching and may not handle all edge cases such as:
/// - Imports within template literals or multiline strings
/// - Dynamically constructed import strings
/// - Malformed syntax
///
/// This is acceptable for the optimization's purpose: to prefetch modules that can be
/// statically determined. Missing some edge cases means we fall back to sequential loading
/// for those modules, which is the baseline behavior.
fn extract_import_specifiers(source: &str) -> Vec<String> {
    let mut specifiers = Vec::new();

    // Match import statements with various patterns:
    // import ... from 'specifier'
    // import ... from "specifier"
    // import('specifier')
    // import("specifier")
    // export ... from 'specifier'
    // export ... from "specifier"

    // Static imports: import ... from 'specifier'
    for line in source.lines() {
        let line = line.trim();

        // Skip single-line comments (basic check)
        // Note: This doesn't handle all cases (inline comments, multi-line comments)
        // but catches the most common cases
        if line.starts_with("//") || line.starts_with("/*") {
            continue;
        }

        // Match: import/export ... from 'specifier' or import/export ... from "specifier"
        // We check that the line starts with import/export to avoid false positives
        if (line.starts_with("import ") || line.starts_with("export "))
            && line.contains(" from ")
            && let Some(from_idx) = line.rfind(" from ")
        {
            let after_from = &line[from_idx + 6..].trim();
            if let Some(spec) = extract_quoted_string(after_from) {
                specifiers.push(spec);
            }
        }
    }

    // Dynamic imports: import('specifier')
    let mut pos = 0;
    while let Some(idx) = source[pos..].find("import(") {
        let start = pos + idx + 7; // after "import("
        if let Some(spec) = extract_quoted_string(&source[start..]) {
            specifiers.push(spec);
        }
        // Move past this import( to avoid infinite loops
        pos = start + 1;
    }

    specifiers
}

/// Extract a string from quotes (single or double)
fn extract_quoted_string(s: &str) -> Option<String> {
    let s = s.trim();
    if let Some(stripped) = s.strip_prefix('\'')
        && let Some(end) = stripped.find('\'')
    {
        return Some(stripped[..end].to_string());
    } else if let Some(stripped) = s.strip_prefix('"')
        && let Some(end) = stripped.find('"')
    {
        return Some(stripped[..end].to_string());
    }
    None
}

/// Prefetch modules and their dependencies in parallel.
/// This populates the SOURCE_CACHE before V8 compilation begins.
///
/// Note: This function creates one thread per module in each batch. For typical projects,
/// batches are small (usually < 20 modules per dependency level), making this approach
/// efficient. The threads are short-lived and only perform file I/O operations.
fn prefetch_modules_parallel(root_path: &str, _referrer_path: &str) {
    let mut to_fetch = vec![root_path.to_string()];
    let mut fetched = HashSet::new();

    while !to_fetch.is_empty() {
        // Take current batch (use std::mem::take to avoid drain-collect)
        let current_batch = std::mem::take(&mut to_fetch);
        let mut next_batch = Vec::new();

        // Process batch in parallel using threads (one per module in this batch)
        // Each thread performs file I/O and import discovery
        let handles: Vec<_> = current_batch
            .into_iter()
            .filter(|path| !path.starts_with("node:")) // Skip built-in modules
            .filter(|path| fetched.insert(path.clone())) // Only process new paths
            .map(|path| {
                std::thread::spawn(move || {
                    // Read the file (will cache it)
                    if let Ok(source) = read_source_cached(&path) {
                        // Extract imports from this module
                        let specifiers = extract_import_specifiers(&source);

                        // Resolve specifiers to absolute paths
                        let resolved: Vec<String> = specifiers
                            .into_iter()
                            .filter_map(|spec| {
                                if spec.starts_with("node:") {
                                    None // Skip built-in modules
                                } else {
                                    // Resolve relative to the current module
                                    Some(normalize_path(&path, &spec))
                                }
                            })
                            .collect();

                        resolved
                    } else {
                        Vec::new()
                    }
                })
            })
            .collect();

        // Collect results
        for handle in handles {
            if let Ok(resolved_paths) = handle.join() {
                next_batch.extend(resolved_paths);
            }
        }

        to_fetch = next_batch;
    }
}

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
        // Prefetch modules in parallel before starting V8 compilation
        // This populates the SOURCE_CACHE to avoid sequential I/O during module resolution
        if !specifier.starts_with("node:") {
            let root_path = normalize_path(referrer, specifier);
            prefetch_modules_parallel(&root_path, referrer);
        }

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
        match read_source_cached(&requested_abs_path) {
            Ok(json_content) => {
                // Create a synthetic module that exports the JSON as the default export
                format!("export default {};", json_content)
            }
            Err(e) => {
                let msg = v8::String::new(
                    scope,
                    &format!("Cannot read JSON file '{}': {}", requested_abs_path, e),
                )
                .unwrap();
                let exception = v8::Exception::error(scope, msg);
                scope.throw_exception(exception);
                return None;
            }
        }
    } else {
        match read_source_cached(&requested_abs_path) {
            Ok(content) => content,
            Err(e) => {
                let msg = v8::String::new(
                    scope,
                    &format!("Cannot read file '{}': {}", requested_abs_path, e),
                )
                .unwrap();
                let exception = v8::Exception::error(scope, msg);
                scope.throw_exception(exception);
                return None;
            }
        }
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

    // Get the parent directory of the referrer, or use current directory if no parent
    let ref_dir = Path::new(referrer_path)
        .parent()
        .unwrap_or_else(|| Path::new("."));

    // Join and canonicalize the path
    let joined = ref_dir.join(req_path);
    match joined.canonicalize() {
        Ok(normalized) => {
            // Convert the normalized path to a string
            // If conversion fails (non-UTF-8 path), fall back to the joined path string
            normalized
                .to_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| joined.display().to_string())
        }
        Err(_) => {
            // If canonicalize fails (e.g., file doesn't exist), return the joined path as a string
            // This allows the caller to provide a better error message
            joined.display().to_string()
        }
    }
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

pub(crate) fn host_import_module_dynamically_callback<'s>(
    scope: &mut v8::PinScope<'s, '_>,
    _host_defined_options: v8::Local<'s, v8::Data>,
    resource_name: v8::Local<'s, v8::Value>,
    specifier: v8::Local<'s, v8::String>,
    _import_attributes: v8::Local<'s, v8::FixedArray>,
) -> Option<v8::Local<'s, v8::Promise>> {
    // Create a promise resolver
    let resolver = v8::PromiseResolver::new(scope)?;
    let promise = resolver.get_promise(scope);

    // Get the referrer path from the resource_name
    // Try to cast it to a string first
    let referrer = match v8::Local::<v8::String>::try_from(resource_name) {
        Ok(resource_str) => resource_str.to_rust_string_lossy(scope),
        Err(_) => {
            // If resource_name is not a string (e.g., an object or undefined),
            // use the current working directory as the referrer.
            // This means relative imports will be resolved from the process's
            // current working directory, which is the expected behavior when
            // there's no clear module context.
            std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| ".".to_string())
        }
    };

    let specifier_str = specifier.to_rust_string_lossy(scope);

    // Use a TryCatch scope to properly handle exceptions
    v8::tc_scope!(let tc, scope);

    // Try to resolve and load the module
    match resolve(tc, &referrer, &specifier_str) {
        Some(module) => {
            // Instantiate the module if not already instantiated
            let status = module.get_status();
            if status == v8::ModuleStatus::Uninstantiated {
                match module.instantiate_module(tc, module_resolve_callback) {
                    Some(_) => {
                        // Instantiation successful, continue
                    }
                    None => {
                        // Instantiation failed, reject the promise
                        let exception = if tc.has_caught() {
                            tc.exception().unwrap()
                        } else {
                            let msg = v8::String::new(tc, "Module instantiation failed").unwrap();
                            v8::Exception::error(tc, msg)
                        };
                        resolver.reject(tc, exception);
                        return Some(promise);
                    }
                }
            }

            // Evaluate the module if not already evaluated
            let status = module.get_status();
            if status == v8::ModuleStatus::Instantiated {
                match module.evaluate(tc) {
                    Some(_) => {
                        // Evaluation successful, continue
                    }
                    None => {
                        // Evaluation failed, reject the promise
                        let exception = if tc.has_caught() {
                            tc.exception().unwrap()
                        } else {
                            let msg = v8::String::new(tc, "Module evaluation failed").unwrap();
                            v8::Exception::error(tc, msg)
                        };
                        resolver.reject(tc, exception);
                        return Some(promise);
                    }
                }
            }

            // Get the module namespace object
            let module_namespace = module.get_module_namespace();

            // Resolve the promise with the module namespace
            resolver.resolve(tc, module_namespace);
        }
        None => {
            // Module resolution failed, reject the promise
            let exception = if tc.has_caught() {
                tc.exception().unwrap()
            } else {
                let msg = v8::String::new(tc, &format!("Cannot find module '{}'", specifier_str))
                    .unwrap();
                v8::Exception::error(tc, msg)
            };
            resolver.reject(tc, exception);
        }
    }

    Some(promise)
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

        // Use cached "url" string
        let cache = state.borrow().string_cache.clone();
        let mut cache_borrow = cache.borrow_mut();
        let url_key = crate::get_or_create_cached_string!(scope, cache_borrow, url, "url");
        drop(cache_borrow);

        let url_value = v8::String::new(scope, &url).unwrap();
        meta.set(scope, url_key.into(), url_value.into());
    }
}
