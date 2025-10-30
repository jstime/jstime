use jstime_core as jstime;
mod common;

#[test]
fn test_crypto_exists() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script("typeof crypto", "test");
    assert_eq!(result.unwrap(), "object");
}

#[test]
fn test_crypto_get_random_values_uint8array() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script(
        r#"
        const arr = new Uint8Array(16);
        const result = crypto.getRandomValues(arr);
        // Check that it returns the same array
        result === arr && arr.length === 16
        "#,
        "test",
    );
    assert_eq!(result.unwrap(), "true");
}

#[test]
fn test_crypto_get_random_values_uint32array() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script(
        r#"
        const arr = new Uint32Array(4);
        crypto.getRandomValues(arr);
        arr.length === 4
        "#,
        "test",
    );
    assert_eq!(result.unwrap(), "true");
}

#[test]
fn test_crypto_get_random_values_non_zero() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script(
        r#"
        const arr = new Uint8Array(16);
        crypto.getRandomValues(arr);
        // Check that at least one byte is non-zero (statistically guaranteed)
        arr.some(x => x !== 0)
        "#,
        "test",
    );
    assert_eq!(result.unwrap(), "true");
}

#[test]
fn test_crypto_get_random_values_error_not_typed_array() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script(
        r#"
        try {
            crypto.getRandomValues([1, 2, 3]);
            false;
        } catch (e) {
            e instanceof TypeError
        }
        "#,
        "test",
    );
    assert_eq!(result.unwrap(), "true");
}

#[test]
fn test_crypto_get_random_values_error_too_large() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script(
        r#"
        try {
            const arr = new Uint8Array(65537);
            crypto.getRandomValues(arr);
            false;
        } catch (e) {
            e instanceof Error
        }
        "#,
        "test",
    );
    assert_eq!(result.unwrap(), "true");
}

#[test]
fn test_crypto_random_uuid() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script(
        r#"
        const uuid = crypto.randomUUID();
        // Check UUID v4 format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
        /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/.test(uuid)
        "#,
        "test",
    );
    assert_eq!(result.unwrap(), "true");
}

#[test]
fn test_crypto_random_uuid_unique() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script(
        r#"
        const uuid1 = crypto.randomUUID();
        const uuid2 = crypto.randomUUID();
        uuid1 !== uuid2
        "#,
        "test",
    );
    assert_eq!(result.unwrap(), "true");
}

#[test]
fn test_crypto_subtle_exists() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script("typeof crypto.subtle", "test");
    assert_eq!(result.unwrap(), "object");
}

#[test]
fn test_crypto_subtle_digest_exists() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script("typeof crypto.subtle.digest", "test");
    assert_eq!(result.unwrap(), "function");
}

#[test]
fn test_crypto_subtle_digest_returns_promise() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script(
        r#"
        const encoder = new TextEncoder();
        const data = encoder.encode('hello');
        const result = crypto.subtle.digest('SHA-256', data);
        result instanceof Promise
        "#,
        "test",
    );
    assert_eq!(result.unwrap(), "true");
}

#[test]
fn test_crypto_subtle_digest_sha256() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    // Use a module with top-level await to test digest
    let result = jstime.import("./tests/fixtures/crypto/test_digest_sha256.js");
    assert!(result.is_ok());
}

#[test]
fn test_crypto_subtle_digest_sha384() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.import("./tests/fixtures/crypto/test_digest_sha384.js");
    assert!(result.is_ok());
}

#[test]
fn test_crypto_subtle_digest_sha512() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.import("./tests/fixtures/crypto/test_digest_sha512.js");
    assert!(result.is_ok());
}

#[test]
fn test_crypto_subtle_digest_with_arraybuffer() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.import("./tests/fixtures/crypto/test_digest_arraybuffer.js");
    assert!(result.is_ok());
}

#[test]
fn test_crypto_subtle_digest_with_algorithm_object() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.import("./tests/fixtures/crypto/test_digest_algorithm_object.js");
    assert!(result.is_ok());
}

#[test]
fn test_crypto_subtle_digest_empty_data() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.import("./tests/fixtures/crypto/test_digest_empty.js");
    assert!(result.is_ok());
}

// New API tests

#[test]
fn test_crypto_subtle_sign_exists() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script("typeof crypto.subtle.sign", "test");
    assert_eq!(result.unwrap(), "function");
}

#[test]
fn test_crypto_subtle_verify_exists() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script("typeof crypto.subtle.verify", "test");
    assert_eq!(result.unwrap(), "function");
}

#[test]
fn test_crypto_subtle_encrypt_exists() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script("typeof crypto.subtle.encrypt", "test");
    assert_eq!(result.unwrap(), "function");
}

#[test]
fn test_crypto_subtle_decrypt_exists() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script("typeof crypto.subtle.decrypt", "test");
    assert_eq!(result.unwrap(), "function");
}

#[test]
fn test_crypto_subtle_generate_key_exists() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script("typeof crypto.subtle.generateKey", "test");
    assert_eq!(result.unwrap(), "function");
}

#[test]
fn test_crypto_subtle_import_key_exists() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script("typeof crypto.subtle.importKey", "test");
    assert_eq!(result.unwrap(), "function");
}

#[test]
fn test_crypto_subtle_export_key_exists() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.run_script("typeof crypto.subtle.exportKey", "test");
    assert_eq!(result.unwrap(), "function");
}

#[test]
fn test_crypto_subtle_generate_key_aes_gcm() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.import("./tests/fixtures/crypto/test_generate_key_aes.js");
    assert!(result.is_ok());
}

#[test]
fn test_crypto_subtle_generate_key_hmac() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.import("./tests/fixtures/crypto/test_generate_key_hmac.js");
    assert!(result.is_ok());
}

#[test]
fn test_crypto_subtle_hmac_sign_verify() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.import("./tests/fixtures/crypto/test_hmac_sign_verify.js");
    assert!(result.is_ok());
}

#[test]
fn test_crypto_subtle_aes_gcm_encrypt_decrypt() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.import("./tests/fixtures/crypto/test_aes_gcm_encrypt_decrypt.js");
    assert!(result.is_ok());
}

#[test]
fn test_crypto_subtle_key_import_export() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.import("./tests/fixtures/crypto/test_key_import_export.js");
    assert!(result.is_ok());
}

#[test]
fn test_crypto_subtle_hmac_sha384() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.import("./tests/fixtures/crypto/test_hmac_sha384.js");
    assert!(result.is_ok());
}

#[test]
fn test_crypto_subtle_hmac_sha512() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.import("./tests/fixtures/crypto/test_hmac_sha512.js");
    assert!(result.is_ok());
}

#[test]
fn test_crypto_subtle_export_non_extractable_key_fails() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.import("./tests/fixtures/crypto/test_export_non_extractable.js");
    assert!(result.is_ok());
}

#[test]
fn test_crypto_subtle_aes_gcm_with_additional_data() {
    let _setup_guard = common::setup();
    let options = jstime::Options::default();
    let mut jstime = jstime::JSTime::new(options);
    let result = jstime.import("./tests/fixtures/crypto/test_aes_gcm_with_aad.js");
    assert!(result.is_ok());
}
