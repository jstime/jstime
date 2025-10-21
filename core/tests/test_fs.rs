use jstime_core as jstime;

mod common;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readfile_as_buffer() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-readfile-buffer.js");
        let result = jstime.run_script("globalThis.testReadFileBuffer", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_readfile_as_string() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-readfile-string.js");
        let result = jstime.run_script("globalThis.testReadFileString", "test");
        assert_eq!(result.unwrap(), "Hello, jstime!");
    }

    #[test]
    fn test_readfile_with_encoding_option() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-readfile-encoding.js");
        let result = jstime.run_script("globalThis.testReadFileEncoding", "test");
        assert_eq!(result.unwrap(), "Test encoding option");
    }

    #[test]
    fn test_readfile_nonexistent_file() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-readfile-error.js");
        let result = jstime.run_script("globalThis.testReadFileError", "test");
        assert_eq!(result.unwrap(), "error_caught");
    }

    #[test]
    fn test_readdir() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-readdir.js");
        let result = jstime.run_script("globalThis.testReadDir", "test");
        assert_eq!(result.unwrap(), "file1.txt,file2.txt,file3.txt");
    }

    #[test]
    fn test_readdir_empty_directory() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-readdir-empty.js");
        let result = jstime.run_script("globalThis.testReadDirEmpty", "test");
        assert_eq!(result.unwrap(), "0");
    }

    #[test]
    fn test_readdir_nonexistent_directory() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-readdir-error.js");
        let result = jstime.run_script("globalThis.testReadDirError", "test");
        assert_eq!(result.unwrap(), "error_caught");
    }

    #[test]
    fn test_fs_module_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-module-exists.js");
        let result = jstime.run_script("globalThis.testFsModuleExists", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_named_imports() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-named-imports.js");
        let result = jstime.run_script("globalThis.testNamedImports", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_writefile() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-writefile.js");
        let result = jstime.run_script("globalThis.testWriteFile", "test");
        assert_eq!(result.unwrap(), "true");

        // Cleanup
        std::fs::remove_file("./tests/fixtures/fs/test-writefile-output.txt").ok();
    }

    #[test]
    fn test_mkdir() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-mkdir.js");
        let result = jstime.run_script("globalThis.testMkdir", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_unlink() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-unlink.js");
        let result = jstime.run_script("globalThis.testUnlink", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_rename() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-rename.js");
        let result = jstime.run_script("globalThis.testRename", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_copyfile() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-copyfile.js");
        let result = jstime.run_script("globalThis.testCopyFile", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_stat() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-stat.js");
        let result = jstime.run_script("globalThis.testStat", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_access() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-access.js");
        let result = jstime.run_script("globalThis.testAccess", "test");
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_constants() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let _result = jstime.import("./tests/fixtures/fs/test-constants.js");
        let result = jstime.run_script("globalThis.testConstants", "test");
        assert_eq!(result.unwrap(), "true");
    }
}
