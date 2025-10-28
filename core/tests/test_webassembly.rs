use jstime_core as jstime;

mod common;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn webassembly_exists() {
        let result = common::get_type_of("WebAssembly");
        assert_eq!(result.unwrap(), "object");
    }

    #[test]
    fn webassembly_module_exists() {
        let result = common::get_type_of("WebAssembly.Module");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn webassembly_instance_exists() {
        let result = common::get_type_of("WebAssembly.Instance");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn webassembly_memory_exists() {
        let result = common::get_type_of("WebAssembly.Memory");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn webassembly_table_exists() {
        let result = common::get_type_of("WebAssembly.Table");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn webassembly_compile_exists() {
        let result = common::get_type_of("WebAssembly.compile");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn webassembly_instantiate_exists() {
        let result = common::get_type_of("WebAssembly.instantiate");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn webassembly_validate_exists() {
        let result = common::get_type_of("WebAssembly.validate");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn webassembly_validate_simple_module() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let script = r#"
            const wasmCode = new Uint8Array([
                0x00, 0x61, 0x73, 0x6d, // WASM_BINARY_MAGIC
                0x01, 0x00, 0x00, 0x00, // WASM_BINARY_VERSION
            ]);
            WebAssembly.validate(wasmCode);
        "#;

        let result = jstime.run_script(script, "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_validate_invalid_module() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let script = r#"
            const invalidCode = new Uint8Array([0x00, 0x01, 0x02, 0x03]);
            WebAssembly.validate(invalidCode);
        "#;

        let result = jstime.run_script(script, "test");
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn webassembly_module_add_function() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let script = r#"
            // Create a simple WebAssembly module that adds two numbers
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
            
            const wasmModule = new WebAssembly.Module(wasmCode);
            const wasmInstance = new WebAssembly.Instance(wasmModule);
            wasmInstance.exports.add(5, 7);
        "#;

        let result = jstime.run_script(script, "test");
        assert_eq!(result.unwrap(), "12");
    }

    #[test]
    fn webassembly_memory_creation() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let script = r#"
            const memory = new WebAssembly.Memory({ initial: 1 });
            memory.buffer instanceof ArrayBuffer;
        "#;

        let result = jstime.run_script(script, "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_memory_grow() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let script = r#"
            const memory = new WebAssembly.Memory({ initial: 1, maximum: 10 });
            const oldSize = memory.grow(2);
            oldSize === 1 && memory.buffer.byteLength === 3 * 65536;
        "#;

        let result = jstime.run_script(script, "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_table_creation() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let script = r#"
            const table = new WebAssembly.Table({ initial: 2, element: 'anyfunc' });
            table.length;
        "#;

        let result = jstime.run_script(script, "test");
        assert_eq!(result.unwrap(), "2");
    }

    #[test]
    fn webassembly_module_exports() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let script = r#"
            // Create a simple WebAssembly module that exports a function
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
            
            const wasmModule = new WebAssembly.Module(wasmCode);
            const wasmInstance = new WebAssembly.Instance(wasmModule);
            typeof wasmInstance.exports.add === 'function';
        "#;

        let result = jstime.run_script(script, "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn webassembly_compile_async() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let script = r#"
            const wasmCode = new Uint8Array([
                0x00, 0x61, 0x73, 0x6d, // WASM_BINARY_MAGIC
                0x01, 0x00, 0x00, 0x00, // WASM_BINARY_VERSION
            ]);
            
            WebAssembly.compile(wasmCode).then(module => {
                return module instanceof WebAssembly.Module;
            });
        "#;

        let result = jstime.run_script(script, "test");
        // Returns a promise
        assert!(result.unwrap().contains("Promise"));
    }

    #[test]
    fn webassembly_instantiate_async() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let script = r#"
            const wasmCode = new Uint8Array([
                0x00, 0x61, 0x73, 0x6d, // WASM_BINARY_MAGIC
                0x01, 0x00, 0x00, 0x00, // WASM_BINARY_VERSION
            ]);
            
            WebAssembly.instantiate(wasmCode).then(result => {
                return result.instance instanceof WebAssembly.Instance && 
                       result.module instanceof WebAssembly.Module;
            });
        "#;

        let result = jstime.run_script(script, "test");
        // Returns a promise
        assert!(result.unwrap().contains("Promise"));
    }
}
