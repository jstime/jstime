use crate::IsolateState;
use rustc_hash::FxHashMap;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{OnceLock, RwLock};

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
static SOURCE_CACHE: OnceLock<RwLock<FxHashMap<String, String>>> = OnceLock::new();

fn get_source_cache() -> &'static RwLock<FxHashMap<String, String>> {
    SOURCE_CACHE.get_or_init(|| RwLock::new(FxHashMap::default()))
}

/// Clear the global source code cache.
/// This is useful in testing or development scenarios where modules may change.
#[allow(dead_code)]
pub(crate) fn clear_source_cache() {
    get_source_cache().write().unwrap().clear();
}

/// Read source code from file, using cache if available
fn read_source_cached(path: &str) -> std::io::Result<String> {
    // Try to read from cache first
    {
        let cache = get_source_cache().read().unwrap();
        if let Some(source) = cache.get(path) {
            return Ok(source.clone());
        }
    }

    // Not in cache, read from file
    let source = std::fs::read_to_string(path)?;

    // Store in cache
    {
        let mut cache = get_source_cache().write().unwrap();
        cache.insert(path.to_string(), source.clone());
    }

    Ok(source)
}

/// Check if a specifier is a package import (starts with `#`).
/// Package imports are defined in the `imports` field of package.json.
///
/// Examples:
/// - "#utils" -> true
/// - "#internal/helpers" -> true
/// - "lodash" -> false
/// - "./foo.js" -> false
#[inline]
fn is_package_import(specifier: &str) -> bool {
    specifier.starts_with('#')
}

/// Check if a specifier is a bare specifier (not relative, absolute, or a built-in module).
/// Bare specifiers are package names that should be resolved from node_modules.
///
/// Examples:
/// - "lodash" -> true (bare specifier)
/// - "@scope/package" -> true (scoped bare specifier)
/// - "./foo.js" -> false (relative)
/// - "../bar.js" -> false (relative)
/// - "/abs/path.js" -> false (absolute)
/// - "node:fs" -> false (built-in)
/// - "#internal" -> false (package import)
#[inline]
fn is_bare_specifier(specifier: &str) -> bool {
    // Not a bare specifier if:
    // - Starts with './' or '../' (relative)
    // - Starts with '/' (absolute)
    // - Starts with 'node:' (built-in)
    // - Starts with '#' (package import)
    // - Contains '://' (URL-like)
    !specifier.starts_with("./")
        && !specifier.starts_with("../")
        && !specifier.starts_with('/')
        && !specifier.starts_with("node:")
        && !specifier.starts_with('#')
        && !specifier.contains("://")
}

/// Parse the package name and subpath from a bare specifier.
/// Returns (package_name, subpath) where subpath may be empty.
///
/// Examples:
/// - "lodash" -> ("lodash", "")
/// - "lodash/fp" -> ("lodash", "fp")
/// - "@scope/package" -> ("@scope/package", "")
/// - "@scope/package/sub" -> ("@scope/package", "sub")
fn parse_package_specifier(specifier: &str) -> (&str, &str) {
    if specifier.starts_with('@') {
        // Scoped package: @scope/name or @scope/name/subpath
        // Find the second '/' to separate package from subpath
        let mut slash_count = 0;
        for (i, c) in specifier.char_indices() {
            if c == '/' {
                slash_count += 1;
                if slash_count == 2 {
                    return (&specifier[..i], &specifier[i + 1..]);
                }
            }
        }
        // No second slash, entire specifier is the package name
        (specifier, "")
    } else {
        // Regular package: name or name/subpath
        match specifier.find('/') {
            Some(i) => (&specifier[..i], &specifier[i + 1..]),
            None => (specifier, ""),
        }
    }
}

/// Read and parse a package.json file, returning the main entry point if found.
/// Handles both "main" and "exports" fields according to Node.js resolution.
fn read_package_json_main(package_dir: &Path, subpath: &str) -> Option<PathBuf> {
    let package_json_path = package_dir.join("package.json");
    let content = std::fs::read_to_string(&package_json_path).ok()?;

    // Parse the JSON using a simple approach
    // We look for "main" and "exports" fields

    // First check if there's a subpath - if so, we need to resolve it
    if !subpath.is_empty() {
        return resolve_package_subpath(package_dir, &content, subpath);
    }

    // Try to find "exports" field first (modern approach)
    if let Some(entry) = resolve_exports_main(&content, package_dir) {
        return Some(entry);
    }

    // Fall back to "main" field
    if let Some(main) = extract_json_string_field(&content, "main") {
        let main_path = package_dir.join(&main);
        // Try the path as-is first
        if main_path.exists() {
            return Some(main_path);
        }
        // Try adding .js extension
        let with_js = package_dir.join(format!("{}.js", main));
        if with_js.exists() {
            return Some(with_js);
        }
        // Try as directory with index.js
        let as_dir = main_path.join("index.js");
        if as_dir.exists() {
            return Some(as_dir);
        }
    }

    // Default to index.js
    let index_path = package_dir.join("index.js");
    if index_path.exists() {
        return Some(index_path);
    }

    None
}

/// Resolve the "exports" field for the main entry point.
/// Handles both simple string exports and conditional exports objects.
fn resolve_exports_main(content: &str, package_dir: &Path) -> Option<PathBuf> {
    // Find the exports field
    let exports_start = content.find("\"exports\"")?;
    let after_exports = &content[exports_start + 9..];
    let colon_pos = after_exports.find(':')?;
    let value_start = &after_exports[colon_pos + 1..].trim_start();

    if value_start.starts_with('"') {
        // Simple string export: "exports": "./dist/index.js"
        if let Some(entry) = extract_quoted_string(value_start) {
            let entry_path = package_dir.join(entry.trim_start_matches("./"));
            if entry_path.exists() {
                return Some(entry_path);
            }
        }
    } else if value_start.starts_with('{') {
        // Object export: "exports": { ".": "./dist/index.js" } or conditional exports
        // Look for "." entry or "default" entry
        if let Some(main_entry) = resolve_exports_dot_entry(value_start, package_dir) {
            return Some(main_entry);
        }
    }

    None
}

/// Resolve the "." entry in an exports object.
fn resolve_exports_dot_entry(exports_obj: &str, package_dir: &Path) -> Option<PathBuf> {
    // Look for ".": which is the main entry point
    // This handles: { ".": "./index.js" } or { ".": { "default": "./index.js" } }

    let dot_patterns = ["\".\": ", "\".\": {"];
    for pattern in &dot_patterns {
        if let Some(pos) = exports_obj.find(pattern) {
            let after_dot = &exports_obj[pos + pattern.len() - 1..].trim_start();
            if after_dot.starts_with('"') {
                // Direct string value
                if let Some(entry) = extract_quoted_string(after_dot) {
                    let entry_path = package_dir.join(entry.trim_start_matches("./"));
                    if entry_path.exists() {
                        return Some(entry_path);
                    }
                }
            } else if after_dot.starts_with('{') {
                // Conditional exports object, look for "default" or "import"
                if let Some(entry) = extract_conditional_export(after_dot, package_dir) {
                    return Some(entry);
                }
            }
        }
    }

    // Also check for "default" at the top level for simpler exports
    if let Some(entry) = extract_conditional_export(exports_obj, package_dir) {
        return Some(entry);
    }

    None
}

/// Extract the "default" or "import" entry from a conditional exports object.
fn extract_conditional_export(obj: &str, package_dir: &Path) -> Option<PathBuf> {
    // Priority: "import" > "default" > "require" (for ESM resolution)
    for key in &["\"import\"", "\"default\"", "\"require\""] {
        if let Some(pos) = obj.find(key) {
            let after_key = &obj[pos + key.len()..];
            let colon_pos = after_key.find(':')?;
            let value = after_key[colon_pos + 1..].trim_start();
            if let Some(entry) = extract_quoted_string(value) {
                let entry_path = package_dir.join(entry.trim_start_matches("./"));
                if entry_path.exists() {
                    return Some(entry_path);
                }
            }
        }
    }
    None
}

/// Resolve a subpath import from a package's exports field.
/// This function is called once per import, not in a loop, so string allocation is acceptable.
fn resolve_package_subpath(package_dir: &Path, content: &str, subpath: &str) -> Option<PathBuf> {
    // First try direct file resolution (for packages without exports)
    let direct_path = package_dir.join(subpath);
    if direct_path.exists() {
        return Some(direct_path);
    }
    // Try with .js extension
    let with_js = package_dir.join(format!("{}.js", subpath));
    if with_js.exists() {
        return Some(with_js);
    }
    // Try as directory with index.js
    let as_dir = direct_path.join("index.js");
    if as_dir.exists() {
        return Some(as_dir);
    }

    // Try to resolve from exports field
    // Note: The format! call below allocates a small string, but this is acceptable
    // because this code path is only reached when direct resolution fails and
    // the function is called once per import, not in an inner loop.
    if let Some(exports_start) = content.find("\"exports\"") {
        let after_exports = &content[exports_start..];
        // Look for "./<subpath>" in exports
        let subpath_pattern = format!("\"./{}", subpath);
        if let Some(pos) = after_exports.find(&subpath_pattern) {
            let remaining = &after_exports[pos..];
            // Find the end of this key
            if let Some(quote_end) = remaining[1..].find('"') {
                let _key = &remaining[1..quote_end + 1];
                // Now find the value after the colon
                let after_key = &remaining[quote_end + 2..];
                if let Some(colon_pos) = after_key.find(':') {
                    let value = after_key[colon_pos + 1..].trim_start();
                    if let Some(entry) = extract_quoted_string(value) {
                        let entry_path = package_dir.join(entry.trim_start_matches("./"));
                        if entry_path.exists() {
                            return Some(entry_path);
                        }
                    } else if value.starts_with('{') {
                        // Conditional export for subpath
                        if let Some(entry) = extract_conditional_export(value, package_dir) {
                            return Some(entry);
                        }
                    }
                }
            }
        }
    }

    None
}

/// Extract a simple string value from a JSON field.
/// This is a simple parser that looks for "field": "value" patterns.
fn extract_json_string_field(json: &str, field: &str) -> Option<String> {
    let pattern = format!("\"{}\"", field);
    let field_pos = json.find(&pattern)?;
    let after_field = &json[field_pos + pattern.len()..];

    // Skip whitespace and colon
    let value_start = after_field.trim_start();
    if !value_start.starts_with(':') {
        return None;
    }
    let value_start = value_start[1..].trim_start();

    extract_quoted_string(value_start)
}

/// Resolve a self-referencing import.
/// This allows a package to import itself using its own name from the `exports` field.
///
/// Starting from the referrer's directory, walk up the directory tree
/// looking for a package.json whose `name` field matches the specifier's package name.
/// If found, resolve using the `exports` field of that package.json.
///
/// Examples:
/// - In package "my-lib", `import { foo } from 'my-lib'` resolves to the package's own exports
/// - In package "my-lib", `import { bar } from 'my-lib/utils'` resolves to the package's "./utils" export
fn resolve_self_reference(referrer_path: &str, specifier: &str) -> Option<String> {
    let (package_name, subpath) = parse_package_specifier(specifier);

    // Start from the referrer's directory
    let referrer = Path::new(referrer_path);
    let mut current_dir = if referrer.is_file() {
        referrer.parent()?
    } else {
        referrer
    };

    // Walk up the directory tree looking for package.json with matching name
    loop {
        let package_json_path = current_dir.join("package.json");
        if package_json_path.exists()
            && let Ok(content) = std::fs::read_to_string(&package_json_path)
            && let Some(name) = extract_json_string_field(&content, "name")
            && name == package_name
        {
            // Found a matching package.json, resolve using exports field
            if let Some(entry_path) = read_package_json_main(current_dir, subpath) {
                return entry_path.to_str().map(|s| s.to_string());
            }
        }

        // Move up to parent directory
        match current_dir.parent() {
            Some(parent) => current_dir = parent,
            None => break,
        }
    }

    None
}

/// Resolve a bare specifier by searching node_modules directories.
/// This implements the Node.js module resolution algorithm.
///
/// Starting from the referrer's directory, walk up the directory tree
/// looking for node_modules/<package_name>.
///
/// Also supports self-referencing: a package can import itself using its own name.
fn resolve_bare_specifier(referrer_path: &str, specifier: &str) -> Option<String> {
    // First, try to resolve as a self-reference
    // This allows a package to import itself using its own name
    if let Some(resolved) = resolve_self_reference(referrer_path, specifier) {
        return Some(resolved);
    }

    let (package_name, subpath) = parse_package_specifier(specifier);

    // Start from the referrer's directory
    let referrer = Path::new(referrer_path);
    let mut current_dir = if referrer.is_file() {
        referrer.parent()?
    } else {
        referrer
    };

    // Walk up the directory tree
    loop {
        let node_modules_dir = current_dir.join("node_modules");
        if node_modules_dir.is_dir() {
            let package_dir = node_modules_dir.join(package_name);
            if package_dir.is_dir() {
                // Found the package, resolve the entry point
                if let Some(entry_path) = read_package_json_main(&package_dir, subpath) {
                    return entry_path.to_str().map(|s| s.to_string());
                }
            }
        }

        // Move up to parent directory
        match current_dir.parent() {
            Some(parent) => current_dir = parent,
            None => break,
        }
    }

    None
}

/// Resolve a package import (specifiers starting with `#`).
/// Package imports are defined in the `imports` field of package.json.
///
/// Starting from the referrer's directory, walk up the directory tree
/// looking for a package.json with an `imports` field that matches the specifier.
fn resolve_package_import(referrer_path: &str, specifier: &str) -> Option<String> {
    // Start from the referrer's directory
    let referrer = Path::new(referrer_path);
    let mut current_dir = if referrer.is_file() {
        referrer.parent()?
    } else {
        referrer
    };

    // Walk up the directory tree looking for package.json with imports field
    loop {
        let package_json_path = current_dir.join("package.json");
        if package_json_path.exists()
            && let Ok(content) = std::fs::read_to_string(&package_json_path)
            && let Some(resolved) = resolve_imports_field(&content, specifier, current_dir)
        {
            return Some(resolved);
        }

        // Move up to parent directory
        match current_dir.parent() {
            Some(parent) => current_dir = parent,
            None => break,
        }
    }

    None
}

/// Resolve a specifier using the `imports` field in package.json.
/// Handles both exact matches and pattern matches (with `*` wildcards).
fn resolve_imports_field(content: &str, specifier: &str, package_dir: &Path) -> Option<String> {
    // Find the imports field
    let imports_start = content.find("\"imports\"")?;
    let after_imports = &content[imports_start + 9..];
    let colon_pos = after_imports.find(':')?;
    let value_start = &after_imports[colon_pos + 1..].trim_start();

    if !value_start.starts_with('{') {
        return None;
    }

    // First, try exact match for the specifier
    let exact_pattern = format!("\"{}\"", specifier);
    if let Some(pos) = value_start.find(&exact_pattern) {
        let after_key = &value_start[pos + exact_pattern.len()..];
        if let Some(colon_pos) = after_key.find(':') {
            let value = after_key[colon_pos + 1..].trim_start();
            if let Some(resolved) = resolve_import_target(value, package_dir) {
                return Some(resolved);
            }
        }
    }

    // Try pattern matching with wildcards (e.g., "#internal/*": "./src/internal/*.js")
    // Find all patterns in the imports field
    let mut search_pos = 0;
    while let Some(quote_pos) = value_start[search_pos..].find("\"#") {
        let start = search_pos + quote_pos + 1; // Skip the opening quote
        if let Some(end_quote) = value_start[start..].find('"') {
            let pattern = &value_start[start..start + end_quote];

            // Check if this is a wildcard pattern
            if pattern.contains('*')
                && let Some(matched) = match_import_pattern(pattern, specifier)
            {
                // Find the value for this pattern
                let after_pattern = &value_start[start + end_quote + 1..];
                if let Some(colon_pos) = after_pattern.find(':') {
                    let value = after_pattern[colon_pos + 1..].trim_start();
                    if let Some(target) = extract_quoted_string(value) {
                        // Replace * in target with matched portion
                        let resolved_target = target.replace('*', &matched);
                        let resolved_path =
                            package_dir.join(resolved_target.trim_start_matches("./"));
                        if resolved_path.exists() {
                            return resolved_path.to_str().map(|s| s.to_string());
                        }
                    }
                }
            }

            search_pos = start + end_quote + 1;
        } else {
            break;
        }
    }

    None
}

/// Match a specifier against an import pattern with wildcards.
/// Returns the portion that matched the `*` if successful.
fn match_import_pattern(pattern: &str, specifier: &str) -> Option<String> {
    if let Some(star_pos) = pattern.find('*') {
        let prefix = &pattern[..star_pos];
        let suffix = &pattern[star_pos + 1..];

        if specifier.starts_with(prefix) && specifier.ends_with(suffix) {
            let matched_len = specifier.len() - prefix.len() - suffix.len();
            let matched = &specifier[prefix.len()..prefix.len() + matched_len];
            return Some(matched.to_string());
        }
    }
    None
}

/// Resolve an import target value, handling both string values and conditional objects.
fn resolve_import_target(value: &str, package_dir: &Path) -> Option<String> {
    let value = value.trim();

    if value.starts_with('"') {
        // Simple string value
        if let Some(target) = extract_quoted_string(value) {
            let resolved_path = package_dir.join(target.trim_start_matches("./"));
            if resolved_path.exists() {
                return resolved_path.to_str().map(|s| s.to_string());
            }
        }
    } else if value.starts_with('{') {
        // Conditional import object
        // Priority: "import" > "default" > "require"
        for key in &["\"import\"", "\"default\"", "\"require\""] {
            if let Some(pos) = value.find(key) {
                let after_key = &value[pos + key.len()..];
                if let Some(colon_pos) = after_key.find(':') {
                    let target_value = after_key[colon_pos + 1..].trim_start();
                    if let Some(target) = extract_quoted_string(target_value) {
                        let resolved_path = package_dir.join(target.trim_start_matches("./"));
                        if resolved_path.exists() {
                            return resolved_path.to_str().map(|s| s.to_string());
                        }
                    }
                }
            }
        }
    }

    None
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
                match m.instantiate_module(tc, module_resolve_callback) {
                    Some(_) => {
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
                        // Module instantiation failed (e.g., dependency resolution failed)
                        if tc.has_caught() {
                            Err(tc.exception().unwrap())
                        } else {
                            let msg = v8::String::new(tc, "Module instantiation failed").unwrap();
                            let exception = v8::Exception::error(tc, msg);
                            Err(exception)
                        }
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

    // Check file type by extension
    let is_json = requested_abs_path.ends_with(".json");
    let is_wasm = requested_abs_path.ends_with(".wasm");

    let js_src = if is_wasm {
        // For WebAssembly files, read the binary content and create a synthetic module
        match std::fs::read(&requested_abs_path) {
            Ok(wasm_bytes) => {
                // Encode binary as base64 to embed in JavaScript
                use base64_simd::STANDARD;
                let base64_encoded = STANDARD.encode_to_string(&wasm_bytes);

                // Create a synthetic module that:
                // 1. Decodes the base64 to get the WebAssembly bytes
                // 2. Synchronously compiles and instantiates the module
                // 3. Exports the instance's exports as the default export
                //
                // Users can import like:
                //   import wasmExports from './module.wasm';
                //   const result = wasmExports.add(1, 2);
                //
                // Error handling is included to provide clear error messages
                // if compilation or instantiation fails.
                format!(
                    r#"const __wasm_base64 = "{}";
const __wasm_binary = Uint8Array.from(atob(__wasm_base64), c => c.charCodeAt(0));
let __wasm_module, __wasm_instance;
try {{
    __wasm_module = new WebAssembly.Module(__wasm_binary);
}} catch (e) {{
    throw new WebAssembly.CompileError(`Failed to compile WebAssembly module '{}': ${{e.message}}`);
}}
try {{
    __wasm_instance = new WebAssembly.Instance(__wasm_module);
}} catch (e) {{
    throw new WebAssembly.LinkError(`Failed to instantiate WebAssembly module '{}': ${{e.message}}`);
}}
export default __wasm_instance.exports;"#,
                    base64_encoded,
                    requested_abs_path
                        .replace('\\', "\\\\")
                        .replace('"', "\\\""),
                    requested_abs_path
                        .replace('\\', "\\\\")
                        .replace('"', "\\\"")
                )
            }
            Err(e) => {
                let msg = v8::String::new(
                    scope,
                    &format!(
                        "Cannot read WebAssembly file '{}': {}",
                        requested_abs_path, e
                    ),
                )
                .unwrap();
                let exception = v8::Exception::error(scope, msg);
                scope.throw_exception(exception);
                return None;
            }
        }
    } else if is_json {
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
    // Different modules have different exports
    let js_src = match module_name {
        "fs/promises" | "fs" => format!(
            r#"
            const mod = globalThis.__node_modules['node:{}'];
            if (!mod) {{
                throw new Error('Built-in module not found: {}');
            }}
            export const {{ readFile, readdir, writeFile, appendFile, mkdir, rmdir, unlink, rename, copyFile, stat, access, rm, truncate, realpath, chmod, mkdtemp, readlink, symlink, lstat, chown, utimes, constants }} = mod;
            export default mod;
            "#,
            module_name, module_name
        ),
        "dgram" => format!(
            r#"
            const mod = globalThis.__node_modules['node:{}'];
            if (!mod) {{
                throw new Error('Built-in module not found: {}');
            }}
            export const {{ createSocket, Socket }} = mod;
            export default mod;
            "#,
            module_name, module_name
        ),
        "buffer" => format!(
            r#"
            const mod = globalThis.__node_modules['node:{}'];
            if (!mod) {{
                throw new Error('Built-in module not found: {}');
            }}
            export const {{ Buffer, kMaxLength, constants, INSPECT_MAX_BYTES, SlowBuffer }} = mod;
            export default mod;
            "#,
            module_name, module_name
        ),
        "events" => format!(
            r#"
            const mod = globalThis.__node_modules['node:{}'];
            if (!mod) {{
                throw new Error('Built-in module not found: {}');
            }}
            export const {{ EventEmitter, once, listenerCount, getEventListeners }} = mod;
            export default mod.EventEmitter;
            "#,
            module_name, module_name
        ),
        _ => format!(
            r#"
            const mod = globalThis.__node_modules['node:{}'];
            if (!mod) {{
                throw new Error('Built-in module not found: {}');
            }}
            export default mod;
            "#,
            module_name, module_name
        ),
    };

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

    // Check if this is a package import (e.g., "#utils", "#internal/helpers")
    // Package imports are resolved using the "imports" field in package.json
    if is_package_import(requested) {
        if let Some(resolved) = resolve_package_import(referrer_path, requested) {
            return resolved;
        }
        // If we can't resolve the package import, return it as-is
        // This will cause a proper error when trying to load the file
        return requested.to_string();
    }

    // Check if this is a bare specifier (e.g., "lodash", "@scope/package")
    // Bare specifiers need to be resolved from node_modules
    if is_bare_specifier(requested) {
        if let Some(resolved) = resolve_bare_specifier(referrer_path, requested) {
            return resolved;
        }
        // If we can't resolve the bare specifier, return it as-is
        // This will cause a proper error when trying to load the file
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
