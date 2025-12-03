// WebAssembly Module Import Tests
// Tests the ability to import .wasm files as ES modules

use jstime_core as jstime;

mod common;

#[cfg(test)]
mod wasm_module_tests {
    use super::*;

    /// Get the path to a wasm fixture file
    fn fixture_path(name: &str) -> String {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        format!("{}/tests/fixtures/wasm/{}", manifest_dir, name)
    }

    #[test]
    fn import_wasm_module_default_export() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_wasm_import.js");
        let wasm_path = fixture_path("add.wasm");

        std::fs::write(
            &test_file,
            format!(
                "import wasmExports from '{}';\nglobalThis.result = wasmExports.add(10, 20);",
                wasm_path
            ),
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Import failed: {:?}", result.err());

        let result = jstime.run_script("globalThis.result", "test");
        assert_eq!(result.unwrap(), "30");

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn import_wasm_module_function_call() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_wasm_func.js");
        let wasm_path = fixture_path("add.wasm");

        std::fs::write(
            &test_file,
            format!(
                r#"import wasmModule from '{}';
globalThis.add = wasmModule.add;
globalThis.result1 = wasmModule.add(5, 7);
globalThis.result2 = wasmModule.add(-10, 20);
globalThis.result3 = wasmModule.add(0, 0);"#,
                wasm_path
            ),
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Import failed: {:?}", result.err());

        let result = jstime.run_script("globalThis.result1", "test");
        assert_eq!(result.unwrap(), "12");

        let result = jstime.run_script("globalThis.result2", "test");
        assert_eq!(result.unwrap(), "10");

        let result = jstime.run_script("globalThis.result3", "test");
        assert_eq!(result.unwrap(), "0");

        let result = jstime.run_script("typeof globalThis.add", "test");
        assert_eq!(result.unwrap(), "function");

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn import_wasm_module_minimal() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_wasm_minimal.js");
        let wasm_path = fixture_path("minimal.wasm");

        std::fs::write(
            &test_file,
            format!(
                "import wasmModule from '{}';\nglobalThis.result = typeof wasmModule;",
                wasm_path
            ),
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Import failed: {:?}", result.err());

        let result = jstime.run_script("globalThis.result", "test");
        assert_eq!(result.unwrap(), "object");

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn import_wasm_module_relative_path() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Create test files in the fixtures directory to test relative imports
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let fixtures_dir = format!("{}/tests/fixtures/wasm", manifest_dir);
        let test_file_path = format!("{}/test_relative_import.js", fixtures_dir);

        std::fs::write(
            &test_file_path,
            r#"import wasmModule from './add.wasm';
globalThis.relativeImportResult = wasmModule.add(100, 200);"#,
        )
        .unwrap();

        let result = jstime.import(&test_file_path);
        assert!(result.is_ok(), "Import failed: {:?}", result.err());

        let result = jstime.run_script("globalThis.relativeImportResult", "test");
        assert_eq!(result.unwrap(), "300");

        std::fs::remove_file(&test_file_path).ok();
    }

    #[test]
    fn import_wasm_module_reexport() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let wrapper_file = temp_dir.join("wasm_wrapper.js");
        let test_file = temp_dir.join("test_wasm_reexport.js");
        let wasm_path = fixture_path("add.wasm");

        // Create a wrapper module that re-exports the wasm module
        std::fs::write(
            &wrapper_file,
            format!("export {{ default as wasm }} from '{}';", wasm_path),
        )
        .unwrap();

        std::fs::write(
            &test_file,
            format!(
                "import {{ wasm }} from '{}';\nglobalThis.reexportResult = wasm.add(50, 50);",
                wrapper_file.to_str().unwrap()
            ),
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Import failed: {:?}", result.err());

        let result = jstime.run_script("globalThis.reexportResult", "test");
        assert_eq!(result.unwrap(), "100");

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&wrapper_file).ok();
    }

    #[test]
    fn import_wasm_module_not_found() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_wasm_not_found.js");

        std::fs::write(
            &test_file,
            "import wasmModule from './nonexistent.wasm';\nglobalThis.result = wasmModule;",
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_err(), "Expected error for non-existent wasm file");

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn import_wasm_module_exports_object_type() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_wasm_exports_type.js");
        let wasm_path = fixture_path("add.wasm");

        std::fs::write(
            &test_file,
            format!(
                r#"import wasmExports from '{}';
globalThis.exportsType = typeof wasmExports;
globalThis.hasAdd = 'add' in wasmExports;
globalThis.addType = typeof wasmExports.add;"#,
                wasm_path
            ),
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Import failed: {:?}", result.err());

        let result = jstime.run_script("globalThis.exportsType", "test");
        assert_eq!(result.unwrap(), "object");

        let result = jstime.run_script("globalThis.hasAdd", "test");
        assert_eq!(result.unwrap(), "true");

        let result = jstime.run_script("globalThis.addType", "test");
        assert_eq!(result.unwrap(), "function");

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn import_wasm_module_multiple_imports() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_wasm_multi_import.js");
        let wasm_path = fixture_path("add.wasm");

        // Import the same wasm module twice (should use cached version)
        std::fs::write(
            &test_file,
            format!(
                r#"import wasm1 from '{}';
import wasm2 from '{}';
globalThis.result1 = wasm1.add(1, 2);
globalThis.result2 = wasm2.add(3, 4);
globalThis.sameModule = wasm1 === wasm2;"#,
                wasm_path, wasm_path
            ),
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Import failed: {:?}", result.err());

        let result = jstime.run_script("globalThis.result1", "test");
        assert_eq!(result.unwrap(), "3");

        let result = jstime.run_script("globalThis.result2", "test");
        assert_eq!(result.unwrap(), "7");

        // The same module should be returned for both imports
        let result = jstime.run_script("globalThis.sameModule", "test");
        assert_eq!(result.unwrap(), "true");

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn dynamic_import_wasm_module() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_wasm_dynamic.js");
        let wasm_path = fixture_path("add.wasm");

        std::fs::write(
            &test_file,
            format!(
                r#"const wasmModule = await import('{}');
globalThis.dynamicResult = wasmModule.default.add(7, 8);"#,
                wasm_path
            ),
        )
        .unwrap();

        let result = jstime.import(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Import failed: {:?}", result.err());

        let result = jstime.run_script("globalThis.dynamicResult", "test");
        assert_eq!(result.unwrap(), "15");

        std::fs::remove_file(&test_file).ok();
    }
}
