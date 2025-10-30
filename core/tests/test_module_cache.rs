use jstime_core as jstime;

mod common;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_module_caching_basic() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Get absolute path
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/fixtures/module-cache/main.js");

        let result = jstime.import(path.to_str().unwrap());
        assert!(result.is_ok(), "Failed to import: {:?}", result);

        // If module caching works correctly, shared.js should only be evaluated once
        // and both module-a and module-b should see the same counter state
        let count = jstime.run_script("globalThis.testResult.count", "test");
        assert_eq!(
            count.unwrap(),
            "2",
            "Counter should be 2 if modules share state"
        );
    }
}

#[test]
fn test_module_file_read_count() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);

    // Create a module that tracks how many times it's been executed
    let temp_dir = std::env::temp_dir();
    let counter_file = temp_dir.join("execution_counter.js");

    std::fs::write(
        &counter_file,
        r#"
if (!globalThis.executionCount) {
    globalThis.executionCount = 0;
}
globalThis.executionCount++;
export const count = globalThis.executionCount;
"#,
    )
    .unwrap();

    let import_a = temp_dir.join("import_a.js");
    std::fs::write(
        &import_a,
        format!(
            "import {{ count }} from '{}';\nexport const countA = count;",
            counter_file.to_str().unwrap()
        ),
    )
    .unwrap();

    let import_b = temp_dir.join("import_b.js");
    std::fs::write(
        &import_b,
        format!(
            "import {{ count }} from '{}';\nexport const countB = count;",
            counter_file.to_str().unwrap()
        ),
    )
    .unwrap();

    let main_file = temp_dir.join("main_counter.js");
    std::fs::write(
            &main_file,
            format!(
                "import {{ countA }} from '{}';\nimport {{ countB }} from '{}';\nglobalThis.resultA = countA;\nglobalThis.resultB = countB;",
                import_a.to_str().unwrap(),
                import_b.to_str().unwrap()
            ),
        )
        .unwrap();

    let result = jstime.import(main_file.to_str().unwrap());
    assert!(result.is_ok(), "Failed to import: {:?}", result);

    // If caching works, the module should only execute once
    let count_a = jstime.run_script("globalThis.resultA", "test");
    let count_b = jstime.run_script("globalThis.resultB", "test");
    let total_count = jstime.run_script("globalThis.executionCount", "test");

    assert_eq!(count_a.unwrap(), "1", "countA should be 1");
    assert_eq!(count_b.unwrap(), "1", "countB should be 1 (same module)");
    assert_eq!(
        total_count.unwrap(),
        "1",
        "Module should execute exactly once"
    );

    // Cleanup
    std::fs::remove_file(&counter_file).ok();
    std::fs::remove_file(&import_a).ok();
    std::fs::remove_file(&import_b).ok();
    std::fs::remove_file(&main_file).ok();
}

#[test]
fn test_module_caching_across_instances() {
    let _setup_guard = common::setup();

    // Create test file
    let temp_dir = std::env::temp_dir();
    let test_module = temp_dir.join("instance_test.js");
    std::fs::write(&test_module, "export const value = 'hello';").unwrap();

    // First instance
    {
        let options = jstime::Options::default();
        let mut jstime1 = jstime::JSTime::new(options);
        let result = jstime1.import(test_module.to_str().unwrap());
        assert!(result.is_ok());
    }

    // Second instance - should NOT have cached module
    {
        let options = jstime::Options::default();
        let mut jstime2 = jstime::JSTime::new(options);
        let result = jstime2.import(test_module.to_str().unwrap());
        assert!(result.is_ok()); // Should still work, just reads from disk again
    }

    std::fs::remove_file(&test_module).ok();
}

#[test]
fn test_source_cache_persistence() {
    let _setup_guard = common::setup();

    // Create a test file
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("cache_persistence_test.js");
    std::fs::write(&test_file, "export const testValue = 'cached';").unwrap();

    // First instance loads the module (populates cache)
    {
        let options = jstime::Options::default();
        let mut jstime1 = jstime::JSTime::new(options);
        let result = jstime1.import(test_file.to_str().unwrap());
        assert!(result.is_ok());
    }

    // Delete the file from disk
    std::fs::remove_file(&test_file).unwrap();

    // Second instance should still work because source is cached
    {
        let options = jstime::Options::default();
        let mut jstime2 = jstime::JSTime::new(options);
        let result = jstime2.import(test_file.to_str().unwrap());
        // This should succeed because the source code is in the cache
        assert!(
            result.is_ok(),
            "Should load from cache even though file was deleted"
        );
    }
}
