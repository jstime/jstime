use jstime_core as jstime;
mod common;

#[test]
fn test_crypto_basic() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script(
        r#"
        // Test crypto.getRandomValues
        const arr = new Uint8Array(16);
        crypto.getRandomValues(arr);
        
        // Test crypto.randomUUID
        const uuid = crypto.randomUUID();
        
        // Return true if both work
        arr.length === 16 && typeof uuid === 'string' && uuid.length === 36
        "#,
        "test",
    );
    assert_eq!(result.unwrap(), "true");
}

#[test]
fn test_crypto_subtle_digest_basic() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.import("./tests/fixtures/crypto/test_digest_sha256.js");
    assert!(result.is_ok());
}
