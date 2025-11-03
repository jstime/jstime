use jstime_core as jstime;

mod common;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_import_es_module() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create temporary test files
        let temp_dir = std::env::temp_dir();
        let export_file = temp_dir.join("test_dynamic_export.mjs");
        let test_file = temp_dir.join("test_dynamic_import.mjs");

        std::fs::write(
            &export_file,
            r#"
export const greeting = "Hello from dynamic import!";
export function greet(name) {
    return `Hello, ${name}!`;
}
export default { message: "Default export" };
            "#,
        )
        .unwrap();

        std::fs::write(
            &test_file,
            format!(
                r#"
const module = await import('{}');
globalThis.dynamicImportGreeting = module.greeting;
globalThis.dynamicImportGreet = module.greet("World");
globalThis.dynamicImportDefault = module.default.message;
                "#,
                export_file.to_str().unwrap()
            ),
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Failed to import: {:?}", result);

        let result = jstime.run_script("globalThis.dynamicImportGreeting", "test");
        assert_eq!(result.unwrap(), "Hello from dynamic import!");

        let result = jstime.run_script("globalThis.dynamicImportGreet", "test");
        assert_eq!(result.unwrap(), "Hello, World!");

        let result = jstime.run_script("globalThis.dynamicImportDefault", "test");
        assert_eq!(result.unwrap(), "Default export");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&export_file).ok();
    }

    #[test]
    fn test_dynamic_import_json() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create temporary test files
        let temp_dir = std::env::temp_dir();
        let json_file = temp_dir.join("test_dynamic_data.json");
        let test_file = temp_dir.join("test_dynamic_json_import.mjs");

        std::fs::write(
            &json_file,
            r#"{"name": "dynamic test", "value": 42, "active": true}"#,
        )
        .unwrap();

        std::fs::write(
            &test_file,
            format!(
                r#"
const data = await import('{}');
globalThis.dynamicJsonName = data.default.name;
globalThis.dynamicJsonValue = data.default.value;
globalThis.dynamicJsonActive = data.default.active;
                "#,
                json_file.to_str().unwrap()
            ),
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Failed to import: {:?}", result);

        let result = jstime.run_script("globalThis.dynamicJsonName", "test");
        assert_eq!(result.unwrap(), "dynamic test");

        let result = jstime.run_script("globalThis.dynamicJsonValue", "test");
        assert_eq!(result.unwrap(), "42");

        let result = jstime.run_script("globalThis.dynamicJsonActive", "test");
        assert_eq!(result.unwrap(), "true");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&json_file).ok();
    }

    #[test]
    fn test_dynamic_import_builtin() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create temporary test file
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_dynamic_builtin.mjs");

        std::fs::write(
            &test_file,
            r#"
const fs = await import('node:fs/promises');
globalThis.hasReadFile = typeof fs.readFile === 'function';
globalThis.hasWriteFile = typeof fs.writeFile === 'function';
            "#,
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Failed to import: {:?}", result);

        let result = jstime.run_script("globalThis.hasReadFile", "test");
        assert_eq!(result.unwrap(), "true");

        let result = jstime.run_script("globalThis.hasWriteFile", "test");
        assert_eq!(result.unwrap(), "true");

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn test_dynamic_import_relative_path() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create temporary test files in the same directory
        let temp_dir = std::env::temp_dir();
        let export_file = temp_dir.join("test_relative_export.mjs");
        let test_file = temp_dir.join("test_relative_import.mjs");

        std::fs::write(
            &export_file,
            r#"
export const message = "Relative import works!";
            "#,
        )
        .unwrap();

        std::fs::write(
            &test_file,
            r#"
const module = await import('./test_relative_export.mjs');
globalThis.relativeMessage = module.message;
            "#,
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Failed to import: {:?}", result);

        let result = jstime.run_script("globalThis.relativeMessage", "test");
        assert_eq!(result.unwrap(), "Relative import works!");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&export_file).ok();
    }

    #[test]
    fn test_dynamic_import_error_handling() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create temporary test file that imports a non-existent module
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_dynamic_error.mjs");

        std::fs::write(
            &test_file,
            r#"
try {
    await import('/non/existent/module.mjs');
    globalThis.importFailed = false;
} catch (err) {
    globalThis.importFailed = true;
    globalThis.errorMessage = err.message;
}
            "#,
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Failed to import test file: {:?}", result);

        let result = jstime.run_script("globalThis.importFailed", "test");
        assert_eq!(result.unwrap(), "true");

        let result = jstime.run_script("globalThis.errorMessage", "test");
        let error_msg = result.unwrap();
        assert!(error_msg.contains("Cannot find module") || error_msg.contains("No such file"));

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn test_dynamic_import_in_promise() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create temporary test files
        let temp_dir = std::env::temp_dir();
        let export_file = temp_dir.join("test_promise_export.mjs");
        let test_file = temp_dir.join("test_promise_import.mjs");

        std::fs::write(
            &export_file,
            r#"
export const value = 123;
            "#,
        )
        .unwrap();

        std::fs::write(
            &test_file,
            format!(
                r#"
import('{}')
    .then(module => {{
        globalThis.promiseImportValue = module.value;
    }})
    .catch(err => {{
        globalThis.promiseImportError = err.message;
    }});
                "#,
                export_file.to_str().unwrap()
            ),
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Failed to import: {:?}", result);

        let result = jstime.run_script("globalThis.promiseImportValue", "test");
        assert_eq!(result.unwrap(), "123");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&export_file).ok();
    }

    #[test]
    fn test_dynamic_import_multiple() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create temporary test files
        let temp_dir = std::env::temp_dir();
        let export_file1 = temp_dir.join("test_multi_export1.mjs");
        let export_file2 = temp_dir.join("test_multi_export2.mjs");
        let test_file = temp_dir.join("test_multi_import.mjs");

        std::fs::write(&export_file1, r#"export const value1 = "first";"#).unwrap();
        std::fs::write(&export_file2, r#"export const value2 = "second";"#).unwrap();

        std::fs::write(
            &test_file,
            format!(
                r#"
const [mod1, mod2] = await Promise.all([
    import('{}'),
    import('{}')
]);
globalThis.multiValue1 = mod1.value1;
globalThis.multiValue2 = mod2.value2;
                "#,
                export_file1.to_str().unwrap(),
                export_file2.to_str().unwrap()
            ),
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Failed to import: {:?}", result);

        let result = jstime.run_script("globalThis.multiValue1", "test");
        assert_eq!(result.unwrap(), "first");

        let result = jstime.run_script("globalThis.multiValue2", "test");
        assert_eq!(result.unwrap(), "second");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&export_file1).ok();
        std::fs::remove_file(&export_file2).ok();
    }
}
