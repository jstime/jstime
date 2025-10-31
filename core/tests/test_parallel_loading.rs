use jstime_core as jstime;

mod common;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parallel_loading_complex_graph() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Get absolute path to main module
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/fixtures/parallel-modules/main.js");

        let result = jstime.import(path.to_str().unwrap());
        assert!(result.is_ok(), "Failed to import: {:?}", result);

        // Verify the computation results are correct
        let x = jstime.run_script("globalThis.parallelTestResult.x", "test");
        let y = jstime.run_script("globalThis.parallelTestResult.y", "test");
        let c = jstime.run_script("globalThis.parallelTestResult.c", "test");

        assert_eq!(
            x.unwrap(),
            "30",
            "x should be 30: processA(processB(5)) = (5+10)*2 = 30"
        );
        assert_eq!(
            y.unwrap(),
            "15",
            "y should be 15: processB(processC(10)) = (10-5)+10 = 15"
        );
        assert_eq!(c.unwrap(), "15", "c should be 15: processC(20) = 20-5 = 15");
    }

    #[test]
    fn test_parallel_loading_with_shared_dependency() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // This tests that util-b is loaded correctly even though it's imported by both lib-x and lib-y
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/fixtures/parallel-modules/main.js");

        let result = jstime.import(path.to_str().unwrap());
        assert!(
            result.is_ok(),
            "Failed to import with shared dependencies: {:?}",
            result
        );
    }
}
