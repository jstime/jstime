use jstime_core as jstime;

mod common;

#[cfg(test)]
mod conformance_webassembly {
    use super::*;

    #[test]
    fn webassembly_namespace_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "typeof WebAssembly === 'object' && WebAssembly !== null",
            "conformance_test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_module_constructor() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "typeof WebAssembly.Module === 'function'",
            "conformance_test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_instance_constructor() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "typeof WebAssembly.Instance === 'function'",
            "conformance_test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_memory_constructor() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "typeof WebAssembly.Memory === 'function'",
            "conformance_test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_table_constructor() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "typeof WebAssembly.Table === 'function'",
            "conformance_test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_compileerror_constructor() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "typeof WebAssembly.CompileError === 'function'",
            "conformance_test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_linkerror_constructor() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "typeof WebAssembly.LinkError === 'function'",
            "conformance_test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_runtimeerror_constructor() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "typeof WebAssembly.RuntimeError === 'function'",
            "conformance_test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_validate_function() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "typeof WebAssembly.validate === 'function'",
            "conformance_test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_compile_function() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "typeof WebAssembly.compile === 'function'",
            "conformance_test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_instantiate_function() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "typeof WebAssembly.instantiate === 'function'",
            "conformance_test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_module_imports_static() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "typeof WebAssembly.Module.imports === 'function'",
            "conformance_test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_module_exports_static() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "typeof WebAssembly.Module.exports === 'function'",
            "conformance_test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_module_custom_sections_static() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "typeof WebAssembly.Module.customSections === 'function'",
            "conformance_test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_instance_exports_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let script = r#"
            const wasmCode = new Uint8Array([
                0x00, 0x61, 0x73, 0x6d, // WASM_BINARY_MAGIC
                0x01, 0x00, 0x00, 0x00, // WASM_BINARY_VERSION
            ]);
            const module = new WebAssembly.Module(wasmCode);
            const instance = new WebAssembly.Instance(module);
            typeof instance.exports === 'object';
        "#;
        let result = jstime.run_script(script, "conformance_test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_memory_buffer_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let script = r#"
            const memory = new WebAssembly.Memory({ initial: 1 });
            memory.buffer instanceof ArrayBuffer;
        "#;
        let result = jstime.run_script(script, "conformance_test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_memory_grow_method() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let script = r#"
            const memory = new WebAssembly.Memory({ initial: 1 });
            typeof memory.grow === 'function';
        "#;
        let result = jstime.run_script(script, "conformance_test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_table_length_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let script = r#"
            const table = new WebAssembly.Table({ initial: 2, element: 'anyfunc' });
            typeof table.length === 'number';
        "#;
        let result = jstime.run_script(script, "conformance_test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_table_grow_method() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let script = r#"
            const table = new WebAssembly.Table({ initial: 2, element: 'anyfunc' });
            typeof table.grow === 'function';
        "#;
        let result = jstime.run_script(script, "conformance_test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_table_get_method() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let script = r#"
            const table = new WebAssembly.Table({ initial: 2, element: 'anyfunc' });
            typeof table.get === 'function';
        "#;
        let result = jstime.run_script(script, "conformance_test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_table_set_method() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let script = r#"
            const table = new WebAssembly.Table({ initial: 2, element: 'anyfunc' });
            typeof table.set === 'function';
        "#;
        let result = jstime.run_script(script, "conformance_test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_validate_returns_boolean() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let script = r#"
            const validWasm = new Uint8Array([
                0x00, 0x61, 0x73, 0x6d, // WASM_BINARY_MAGIC
                0x01, 0x00, 0x00, 0x00, // WASM_BINARY_VERSION
            ]);
            typeof WebAssembly.validate(validWasm) === 'boolean';
        "#;
        let result = jstime.run_script(script, "conformance_test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_validate_valid_module_returns_true() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let script = r#"
            const validWasm = new Uint8Array([
                0x00, 0x61, 0x73, 0x6d, // WASM_BINARY_MAGIC
                0x01, 0x00, 0x00, 0x00, // WASM_BINARY_VERSION
            ]);
            WebAssembly.validate(validWasm) === true;
        "#;
        let result = jstime.run_script(script, "conformance_test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_validate_invalid_module_returns_false() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let script = r#"
            const invalidWasm = new Uint8Array([0x00, 0x01, 0x02, 0x03]);
            WebAssembly.validate(invalidWasm) === false;
        "#;
        let result = jstime.run_script(script, "conformance_test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_compile_returns_promise() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let script = r#"
            const wasmCode = new Uint8Array([
                0x00, 0x61, 0x73, 0x6d,
                0x01, 0x00, 0x00, 0x00,
            ]);
            WebAssembly.compile(wasmCode) instanceof Promise;
        "#;
        let result = jstime.run_script(script, "conformance_test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_instantiate_returns_promise() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let script = r#"
            const wasmCode = new Uint8Array([
                0x00, 0x61, 0x73, 0x6d,
                0x01, 0x00, 0x00, 0x00,
            ]);
            WebAssembly.instantiate(wasmCode) instanceof Promise;
        "#;
        let result = jstime.run_script(script, "conformance_test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_error_hierarchy() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let script = r#"
            const compileError = new WebAssembly.CompileError();
            const linkError = new WebAssembly.LinkError();
            const runtimeError = new WebAssembly.RuntimeError();
            compileError instanceof Error && 
            linkError instanceof Error && 
            runtimeError instanceof Error;
        "#;
        let result = jstime.run_script(script, "conformance_test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_module_with_add_function() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let script = r#"
            const wasmCode = new Uint8Array([
                0x00, 0x61, 0x73, 0x6d, // WASM_BINARY_MAGIC
                0x01, 0x00, 0x00, 0x00, // WASM_BINARY_VERSION
                // Type section
                0x01, 0x07, 0x01,       // section code, section size, num types
                0x60, 0x02, 0x7f, 0x7f, // func type: (i32, i32) -> ...
                0x01, 0x7f,             // ... -> i32
                // Function section
                0x03, 0x02, 0x01, 0x00, // section code, section size, num functions, func 0 type
                // Export section
                0x07, 0x07, 0x01,       // section code, section size, num exports
                0x03, 0x61, 0x64, 0x64, // field_len, field_str "add"
                0x00, 0x00,             // export kind (func), export func index
                // Code section
                0x0a, 0x09, 0x01,       // section code, section size, num functions
                0x07, 0x00,             // body size, local decl count
                0x20, 0x00,             // local.get 0
                0x20, 0x01,             // local.get 1
                0x6a,                   // i32.add
                0x0b                    // end
            ]);
            const module = new WebAssembly.Module(wasmCode);
            const instance = new WebAssembly.Instance(module);
            instance.exports.add(10, 20) === 30;
        "#;
        let result = jstime.run_script(script, "conformance_test");
        assert_eq!(result.unwrap(), "true");
    }
}
