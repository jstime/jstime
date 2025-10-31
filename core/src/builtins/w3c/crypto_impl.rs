use ring::aead::BoundKey;
use ring::digest;
use ring::rand::SecureRandom;
use ring::{aead, hmac};

pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(crypto_get_random_values),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(crypto_random_uuid),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(crypto_subtle_digest),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(crypto_subtle_sign),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(crypto_subtle_verify),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(crypto_subtle_encrypt),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(crypto_subtle_decrypt),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(crypto_subtle_generate_key),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(crypto_subtle_import_key),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(crypto_subtle_export_key),
        },
    ]
}

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    let name = v8::String::new(scope, "cryptoGetRandomValues").unwrap();
    let value = v8::Function::new(scope, crypto_get_random_values).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "cryptoRandomUUID").unwrap();
    let value = v8::Function::new(scope, crypto_random_uuid).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "cryptoSubtleDigest").unwrap();
    let value = v8::Function::new(scope, crypto_subtle_digest).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "cryptoSubtleSign").unwrap();
    let value = v8::Function::new(scope, crypto_subtle_sign).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "cryptoSubtleVerify").unwrap();
    let value = v8::Function::new(scope, crypto_subtle_verify).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "cryptoSubtleEncrypt").unwrap();
    let value = v8::Function::new(scope, crypto_subtle_encrypt).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "cryptoSubtleDecrypt").unwrap();
    let value = v8::Function::new(scope, crypto_subtle_decrypt).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "cryptoSubtleGenerateKey").unwrap();
    let value = v8::Function::new(scope, crypto_subtle_generate_key).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "cryptoSubtleImportKey").unwrap();
    let value = v8::Function::new(scope, crypto_subtle_import_key).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "cryptoSubtleExportKey").unwrap();
    let value = v8::Function::new(scope, crypto_subtle_export_key).unwrap();
    bindings.set(scope, name.into(), value.into());
}

// crypto.getRandomValues(typedArray)
fn crypto_get_random_values(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 1, "crypto.getRandomValues") {
        return;
    }

    let array = args.get(0);

    // Check if it's a TypedArray
    if !array.is_typed_array() {
        crate::error::throw_type_error(
            scope,
            "crypto.getRandomValues: argument 1 is not a TypedArray",
        );
        return;
    }

    let typed_array = v8::Local::<v8::TypedArray>::try_from(array).unwrap();

    // Check size (spec says max 65536 bytes)
    if typed_array.byte_length() > 65536 {
        crate::error::throw_error(
            scope,
            "crypto.getRandomValues: array size exceeds 65536 bytes",
        );
        return;
    }

    // Get the underlying ArrayBuffer and fill it with random data
    let buffer = typed_array.buffer(scope).unwrap();
    let backing_store = buffer.get_backing_store();
    let data = unsafe {
        std::slice::from_raw_parts_mut(
            backing_store.data().unwrap().as_ptr() as *mut u8,
            typed_array.byte_length(),
        )
    };

    // Fill with random bytes using cached SystemRandom
    let state = crate::isolate_state::IsolateState::get(scope);
    let state_ref = state.borrow();
    if state_ref.system_random.fill(data).is_err() {
        drop(state_ref);
        crate::error::throw_error(scope, "Failed to generate random values");
        return;
    }
    drop(state_ref);

    // Return the same array
    rv.set(array);
}

// crypto.randomUUID()
fn crypto_random_uuid(
    scope: &mut v8::PinScope,
    _args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let state = crate::isolate_state::IsolateState::get(scope);
    let state_ref = state.borrow();

    let mut bytes = [0u8; 16];
    if state_ref.system_random.fill(&mut bytes).is_err() {
        drop(state_ref);
        crate::error::throw_error(scope, "Failed to generate random UUID");
        return;
    }
    drop(state_ref);

    // Set version to 4 (random)
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    // Set variant to RFC 4122
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    // Format as UUID string
    let uuid = format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
        bytes[8],
        bytes[9],
        bytes[10],
        bytes[11],
        bytes[12],
        bytes[13],
        bytes[14],
        bytes[15]
    );

    let result = v8::String::new(scope, &uuid).unwrap();
    rv.set(result.into());
}

// crypto.subtle.digest(algorithm, data)
fn crypto_subtle_digest(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 2, "crypto.subtle.digest") {
        return;
    }

    // Get algorithm name
    let algorithm = args.get(0);
    let algorithm_str = {
        v8::tc_scope!(let tc, scope);
        match algorithm.to_string(tc) {
            Some(s) => s.to_rust_string_lossy(tc),
            None => {
                // tc_scope dropped here, so we can borrow scope mutably
                String::new()
            }
        }
    };

    if algorithm_str.is_empty() && !algorithm.is_string() {
        crate::error::throw_type_error(scope, "Invalid algorithm argument");
        return;
    }

    // Get data
    let data = args.get(1);
    let data_bytes = if data.is_array_buffer_view() {
        let view = v8::Local::<v8::ArrayBufferView>::try_from(data).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let backing_store = buffer.get_backing_store();
        let byte_offset = view.byte_offset();
        let byte_length = view.byte_length();

        if byte_length == 0 {
            &[]
        } else {
            unsafe {
                std::slice::from_raw_parts(
                    (backing_store.data().unwrap().as_ptr() as *const u8).add(byte_offset),
                    byte_length,
                )
            }
        }
    } else if data.is_array_buffer() {
        let buffer = v8::Local::<v8::ArrayBuffer>::try_from(data).unwrap();
        let backing_store = buffer.get_backing_store();
        let byte_length = backing_store.byte_length();

        if byte_length == 0 {
            &[]
        } else {
            unsafe {
                std::slice::from_raw_parts(
                    backing_store.data().unwrap().as_ptr() as *const u8,
                    byte_length,
                )
            }
        }
    } else {
        crate::error::throw_type_error(
            scope,
            "crypto.subtle.digest: data must be an ArrayBuffer or ArrayBufferView",
        );
        return;
    };

    // Perform hashing
    let hash_result = match algorithm_str.as_str() {
        "SHA-256" => digest::digest(&digest::SHA256, data_bytes),
        "SHA-384" => digest::digest(&digest::SHA384, data_bytes),
        "SHA-512" => digest::digest(&digest::SHA512, data_bytes),
        _ => {
            crate::error::throw_error(scope, &format!("Unsupported algorithm: {}", algorithm_str));
            return;
        }
    };

    // Create ArrayBuffer with the hash result
    let hash_bytes = hash_result.as_ref();
    let backing_store =
        v8::ArrayBuffer::new_backing_store_from_vec(hash_bytes.to_vec()).make_shared();
    let array_buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);

    // Create a promise and resolve it with the array buffer
    let resolver = v8::PromiseResolver::new(scope).unwrap();
    let promise = resolver.get_promise(scope);
    resolver.resolve(scope, array_buffer.into());

    rv.set(promise.into());
}

// Helper function to extract bytes from ArrayBuffer or ArrayBufferView
fn get_buffer_data<'a, 'b>(
    scope: &mut v8::PinScope<'a, 'b>,
    value: v8::Local<v8::Value>,
) -> Option<&'a [u8]> {
    if value.is_array_buffer_view() {
        let view = v8::Local::<v8::ArrayBufferView>::try_from(value).ok()?;
        let buffer = view.buffer(scope)?;
        let backing_store = buffer.get_backing_store();
        let byte_offset = view.byte_offset();
        let byte_length = view.byte_length();

        if byte_length == 0 {
            Some(&[])
        } else {
            Some(unsafe {
                std::slice::from_raw_parts(
                    (backing_store.data()?.as_ptr() as *const u8).add(byte_offset),
                    byte_length,
                )
            })
        }
    } else if value.is_array_buffer() {
        let buffer = v8::Local::<v8::ArrayBuffer>::try_from(value).ok()?;
        let backing_store = buffer.get_backing_store();
        let byte_length = backing_store.byte_length();

        if byte_length == 0 {
            Some(&[])
        } else {
            Some(unsafe {
                std::slice::from_raw_parts(backing_store.data()?.as_ptr() as *const u8, byte_length)
            })
        }
    } else {
        None
    }
}

// crypto.subtle.sign(algorithm, key, data)
fn crypto_subtle_sign(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 3, "crypto.subtle.sign") {
        return;
    }

    // Get algorithm
    let algorithm = args.get(0);
    let algorithm_name = if algorithm.is_string() {
        v8::tc_scope!(let tc, scope);
        algorithm.to_string(tc).unwrap().to_rust_string_lossy(tc)
    } else if algorithm.is_object() {
        let obj = v8::Local::<v8::Object>::try_from(algorithm).unwrap();
        let name_key = v8::String::new(scope, "name").unwrap();
        let name_value = obj.get(scope, name_key.into()).unwrap();
        v8::tc_scope!(let tc, scope);
        name_value.to_string(tc).unwrap().to_rust_string_lossy(tc)
    } else {
        crate::error::throw_type_error(scope, "Invalid algorithm");
        return;
    };

    // Get key (must be an object with algorithm and key data)
    let key = args.get(1);
    if !key.is_object() {
        crate::error::throw_type_error(scope, "Key must be an object");
        return;
    }

    let key_obj = v8::Local::<v8::Object>::try_from(key).unwrap();
    let key_data_prop = v8::String::new(scope, "_keyData").unwrap();
    let key_data_val = key_obj.get(scope, key_data_prop.into()).unwrap();

    let key_bytes = match get_buffer_data(scope, key_data_val) {
        Some(bytes) => bytes,
        None => {
            crate::error::throw_type_error(scope, "Invalid key data");
            return;
        }
    };

    // Get data to sign
    let data = args.get(2);
    let data_bytes = match get_buffer_data(scope, data) {
        Some(bytes) => bytes,
        None => {
            crate::error::throw_type_error(scope, "Data must be an ArrayBuffer or ArrayBufferView");
            return;
        }
    };

    // Perform signing based on algorithm
    let signature = match algorithm_name.as_str() {
        "HMAC" => {
            // Get hash algorithm from key
            let key_alg_prop = v8::String::new(scope, "algorithm").unwrap();
            let key_alg_obj = key_obj.get(scope, key_alg_prop.into()).unwrap();

            if !key_alg_obj.is_object() {
                crate::error::throw_error(scope, "Invalid key algorithm");
                return;
            }

            let key_alg = v8::Local::<v8::Object>::try_from(key_alg_obj).unwrap();
            let hash_prop = v8::String::new(scope, "hash").unwrap();
            let hash_obj = key_alg.get(scope, hash_prop.into()).unwrap();

            let hash_name = if hash_obj.is_string() {
                v8::tc_scope!(let tc, scope);
                hash_obj.to_string(tc).unwrap().to_rust_string_lossy(tc)
            } else if hash_obj.is_object() {
                let hash_obj = v8::Local::<v8::Object>::try_from(hash_obj).unwrap();
                let name_key = v8::String::new(scope, "name").unwrap();
                let name_val = hash_obj.get(scope, name_key.into()).unwrap();
                v8::tc_scope!(let tc, scope);
                name_val.to_string(tc).unwrap().to_rust_string_lossy(tc)
            } else {
                crate::error::throw_error(scope, "Invalid hash algorithm");
                return;
            };

            let hmac_algorithm = match hash_name.as_str() {
                "SHA-256" => hmac::HMAC_SHA256,
                "SHA-384" => hmac::HMAC_SHA384,
                "SHA-512" => hmac::HMAC_SHA512,
                _ => {
                    crate::error::throw_error(scope, &format!("Unsupported hash: {}", hash_name));
                    return;
                }
            };

            let key = hmac::Key::new(hmac_algorithm, key_bytes);
            let tag = hmac::sign(&key, data_bytes);
            tag.as_ref().to_vec()
        }
        _ => {
            crate::error::throw_error(
                scope,
                &format!("Unsupported signing algorithm: {}", algorithm_name),
            );
            return;
        }
    };

    // Create ArrayBuffer with signature
    let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(signature).make_shared();
    let array_buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);

    // Create and resolve promise
    let resolver = v8::PromiseResolver::new(scope).unwrap();
    let promise = resolver.get_promise(scope);
    resolver.resolve(scope, array_buffer.into());

    rv.set(promise.into());
}

// crypto.subtle.verify(algorithm, key, signature, data)
fn crypto_subtle_verify(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 4, "crypto.subtle.verify") {
        return;
    }

    // Get algorithm
    let algorithm = args.get(0);
    let algorithm_name = if algorithm.is_string() {
        v8::tc_scope!(let tc, scope);
        algorithm.to_string(tc).unwrap().to_rust_string_lossy(tc)
    } else if algorithm.is_object() {
        let obj = v8::Local::<v8::Object>::try_from(algorithm).unwrap();
        let name_key = v8::String::new(scope, "name").unwrap();
        let name_value = obj.get(scope, name_key.into()).unwrap();
        v8::tc_scope!(let tc, scope);
        name_value.to_string(tc).unwrap().to_rust_string_lossy(tc)
    } else {
        crate::error::throw_type_error(scope, "Invalid algorithm");
        return;
    };

    // Get key
    let key = args.get(1);
    if !key.is_object() {
        crate::error::throw_type_error(scope, "Key must be an object");
        return;
    }

    let key_obj = v8::Local::<v8::Object>::try_from(key).unwrap();
    let key_data_prop = v8::String::new(scope, "_keyData").unwrap();
    let key_data_val = key_obj.get(scope, key_data_prop.into()).unwrap();

    let key_bytes = match get_buffer_data(scope, key_data_val) {
        Some(bytes) => bytes,
        None => {
            crate::error::throw_type_error(scope, "Invalid key data");
            return;
        }
    };

    // Get signature
    let signature = args.get(2);
    let signature_bytes = match get_buffer_data(scope, signature) {
        Some(bytes) => bytes,
        None => {
            crate::error::throw_type_error(
                scope,
                "Signature must be an ArrayBuffer or ArrayBufferView",
            );
            return;
        }
    };

    // Get data
    let data = args.get(3);
    let data_bytes = match get_buffer_data(scope, data) {
        Some(bytes) => bytes,
        None => {
            crate::error::throw_type_error(scope, "Data must be an ArrayBuffer or ArrayBufferView");
            return;
        }
    };

    // Perform verification
    let is_valid = match algorithm_name.as_str() {
        "HMAC" => {
            // Get hash algorithm from key
            let key_alg_prop = v8::String::new(scope, "algorithm").unwrap();
            let key_alg_obj = key_obj.get(scope, key_alg_prop.into()).unwrap();

            if !key_alg_obj.is_object() {
                crate::error::throw_error(scope, "Invalid key algorithm");
                return;
            }

            let key_alg = v8::Local::<v8::Object>::try_from(key_alg_obj).unwrap();
            let hash_prop = v8::String::new(scope, "hash").unwrap();
            let hash_obj = key_alg.get(scope, hash_prop.into()).unwrap();

            let hash_name = if hash_obj.is_string() {
                v8::tc_scope!(let tc, scope);
                hash_obj.to_string(tc).unwrap().to_rust_string_lossy(tc)
            } else if hash_obj.is_object() {
                let hash_obj = v8::Local::<v8::Object>::try_from(hash_obj).unwrap();
                let name_key = v8::String::new(scope, "name").unwrap();
                let name_val = hash_obj.get(scope, name_key.into()).unwrap();
                v8::tc_scope!(let tc, scope);
                name_val.to_string(tc).unwrap().to_rust_string_lossy(tc)
            } else {
                crate::error::throw_error(scope, "Invalid hash algorithm");
                return;
            };

            let hmac_algorithm = match hash_name.as_str() {
                "SHA-256" => hmac::HMAC_SHA256,
                "SHA-384" => hmac::HMAC_SHA384,
                "SHA-512" => hmac::HMAC_SHA512,
                _ => {
                    crate::error::throw_error(scope, &format!("Unsupported hash: {}", hash_name));
                    return;
                }
            };

            let key = hmac::Key::new(hmac_algorithm, key_bytes);
            hmac::verify(&key, data_bytes, signature_bytes).is_ok()
        }
        _ => {
            crate::error::throw_error(
                scope,
                &format!("Unsupported verification algorithm: {}", algorithm_name),
            );
            return;
        }
    };

    // Create and resolve promise with boolean result
    let result = v8::Boolean::new(scope, is_valid);
    let resolver = v8::PromiseResolver::new(scope).unwrap();
    let promise = resolver.get_promise(scope);
    resolver.resolve(scope, result.into());

    rv.set(promise.into());
}

// crypto.subtle.encrypt(algorithm, key, data)
fn crypto_subtle_encrypt(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 3, "crypto.subtle.encrypt") {
        return;
    }

    // Get algorithm
    let algorithm = args.get(0);
    if !algorithm.is_object() {
        crate::error::throw_type_error(scope, "Algorithm must be an object");
        return;
    }

    let alg_obj = v8::Local::<v8::Object>::try_from(algorithm).unwrap();
    let name_key = v8::String::new(scope, "name").unwrap();
    let name_value = alg_obj.get(scope, name_key.into()).unwrap();
    let algorithm_name = {
        v8::tc_scope!(let tc, scope);
        name_value.to_string(tc).unwrap().to_rust_string_lossy(tc)
    };

    // Get key
    let key = args.get(1);
    if !key.is_object() {
        crate::error::throw_type_error(scope, "Key must be an object");
        return;
    }

    let key_obj = v8::Local::<v8::Object>::try_from(key).unwrap();
    let key_data_prop = v8::String::new(scope, "_keyData").unwrap();
    let key_data_val = key_obj.get(scope, key_data_prop.into()).unwrap();

    let key_bytes = match get_buffer_data(scope, key_data_val) {
        Some(bytes) => bytes,
        None => {
            crate::error::throw_type_error(scope, "Invalid key data");
            return;
        }
    };

    // Get data to encrypt
    let data = args.get(2);
    let data_bytes = match get_buffer_data(scope, data) {
        Some(bytes) => bytes,
        None => {
            crate::error::throw_type_error(scope, "Data must be an ArrayBuffer or ArrayBufferView");
            return;
        }
    };

    // Perform encryption
    let encrypted_data = match algorithm_name.as_str() {
        "AES-GCM" => {
            // Get IV from algorithm
            let iv_key = v8::String::new(scope, "iv").unwrap();
            let iv_value = alg_obj.get(scope, iv_key.into()).unwrap();
            let iv_bytes = match get_buffer_data(scope, iv_value) {
                Some(bytes) => bytes,
                None => {
                    crate::error::throw_type_error(
                        scope,
                        "IV must be an ArrayBuffer or ArrayBufferView",
                    );
                    return;
                }
            };

            // Get optional additional data
            let ad_key = v8::String::new(scope, "additionalData").unwrap();
            let ad_value = alg_obj.get(scope, ad_key.into());
            let ad_bytes = if let Some(ad_val) = ad_value {
                get_buffer_data(scope, ad_val).unwrap_or(&[])
            } else {
                &[]
            };

            // Create unbound key
            let unbound_key = match aead::UnboundKey::new(&aead::AES_256_GCM, key_bytes) {
                Ok(key) => key,
                Err(_) => {
                    crate::error::throw_error(
                        scope,
                        "Invalid key length for AES-256-GCM (expected 32 bytes)",
                    );
                    return;
                }
            };

            // Create nonce sequence (single use)
            struct SingleNonce(Option<aead::Nonce>);
            impl aead::NonceSequence for SingleNonce {
                fn advance(&mut self) -> Result<aead::Nonce, ring::error::Unspecified> {
                    self.0.take().ok_or(ring::error::Unspecified)
                }
            }

            let nonce = match aead::Nonce::try_assume_unique_for_key(iv_bytes) {
                Ok(n) => n,
                Err(_) => {
                    crate::error::throw_error(
                        scope,
                        "Invalid IV length (expected 12 bytes for AES-GCM)",
                    );
                    return;
                }
            };

            let mut sealing_key = aead::SealingKey::new(unbound_key, SingleNonce(Some(nonce)));

            // Prepare data for encryption (need mutable buffer with space for tag)
            let mut in_out = data_bytes.to_vec();
            let tag_len = aead::AES_256_GCM.tag_len();
            in_out.resize(in_out.len() + tag_len, 0);

            // Encrypt
            let aad = aead::Aad::from(ad_bytes);
            match sealing_key.seal_in_place_separate_tag(aad, &mut in_out[..data_bytes.len()]) {
                Ok(tag) => {
                    // Append tag to encrypted data
                    let data_len = data_bytes.len();
                    in_out[data_len..data_len + tag.as_ref().len()].copy_from_slice(tag.as_ref());
                    in_out.truncate(data_len + tag.as_ref().len());
                    in_out
                }
                Err(_) => {
                    crate::error::throw_error(scope, "Encryption failed");
                    return;
                }
            }
        }
        _ => {
            crate::error::throw_error(
                scope,
                &format!("Unsupported encryption algorithm: {}", algorithm_name),
            );
            return;
        }
    };

    // Create ArrayBuffer with encrypted data
    let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(encrypted_data).make_shared();
    let array_buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);

    // Create and resolve promise
    let resolver = v8::PromiseResolver::new(scope).unwrap();
    let promise = resolver.get_promise(scope);
    resolver.resolve(scope, array_buffer.into());

    rv.set(promise.into());
}

// crypto.subtle.decrypt(algorithm, key, data)
fn crypto_subtle_decrypt(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 3, "crypto.subtle.decrypt") {
        return;
    }

    // Get algorithm
    let algorithm = args.get(0);
    if !algorithm.is_object() {
        crate::error::throw_type_error(scope, "Algorithm must be an object");
        return;
    }

    let alg_obj = v8::Local::<v8::Object>::try_from(algorithm).unwrap();
    let name_key = v8::String::new(scope, "name").unwrap();
    let name_value = alg_obj.get(scope, name_key.into()).unwrap();
    let algorithm_name = {
        v8::tc_scope!(let tc, scope);
        name_value.to_string(tc).unwrap().to_rust_string_lossy(tc)
    };

    // Get key
    let key = args.get(1);
    if !key.is_object() {
        crate::error::throw_type_error(scope, "Key must be an object");
        return;
    }

    let key_obj = v8::Local::<v8::Object>::try_from(key).unwrap();
    let key_data_prop = v8::String::new(scope, "_keyData").unwrap();
    let key_data_val = key_obj.get(scope, key_data_prop.into()).unwrap();

    let key_bytes = match get_buffer_data(scope, key_data_val) {
        Some(bytes) => bytes,
        None => {
            crate::error::throw_type_error(scope, "Invalid key data");
            return;
        }
    };

    // Get data to decrypt
    let data = args.get(2);
    let data_bytes = match get_buffer_data(scope, data) {
        Some(bytes) => bytes,
        None => {
            crate::error::throw_type_error(scope, "Data must be an ArrayBuffer or ArrayBufferView");
            return;
        }
    };

    // Perform decryption
    let decrypted_data = match algorithm_name.as_str() {
        "AES-GCM" => {
            // Get IV from algorithm
            let iv_key = v8::String::new(scope, "iv").unwrap();
            let iv_value = alg_obj.get(scope, iv_key.into()).unwrap();
            let iv_bytes = match get_buffer_data(scope, iv_value) {
                Some(bytes) => bytes,
                None => {
                    crate::error::throw_type_error(
                        scope,
                        "IV must be an ArrayBuffer or ArrayBufferView",
                    );
                    return;
                }
            };

            // Get optional additional data
            let ad_key = v8::String::new(scope, "additionalData").unwrap();
            let ad_value = alg_obj.get(scope, ad_key.into());
            let ad_bytes = if let Some(ad_val) = ad_value {
                get_buffer_data(scope, ad_val).unwrap_or(&[])
            } else {
                &[]
            };

            // Create unbound key
            let unbound_key = match aead::UnboundKey::new(&aead::AES_256_GCM, key_bytes) {
                Ok(key) => key,
                Err(_) => {
                    crate::error::throw_error(
                        scope,
                        "Invalid key length for AES-256-GCM (expected 32 bytes)",
                    );
                    return;
                }
            };

            // Create nonce sequence (single use)
            struct SingleNonce(Option<aead::Nonce>);
            impl aead::NonceSequence for SingleNonce {
                fn advance(&mut self) -> Result<aead::Nonce, ring::error::Unspecified> {
                    self.0.take().ok_or(ring::error::Unspecified)
                }
            }

            let nonce = match aead::Nonce::try_assume_unique_for_key(iv_bytes) {
                Ok(n) => n,
                Err(_) => {
                    crate::error::throw_error(
                        scope,
                        "Invalid IV length (expected 12 bytes for AES-GCM)",
                    );
                    return;
                }
            };

            let mut opening_key = aead::OpeningKey::new(unbound_key, SingleNonce(Some(nonce)));

            // Prepare data for decryption (need mutable buffer)
            let mut in_out = data_bytes.to_vec();

            // Decrypt
            let aad = aead::Aad::from(ad_bytes);
            match opening_key.open_in_place(aad, &mut in_out) {
                Ok(plaintext) => plaintext.to_vec(),
                Err(_) => {
                    crate::error::throw_error(scope, "Decryption failed");
                    return;
                }
            }
        }
        _ => {
            crate::error::throw_error(
                scope,
                &format!("Unsupported decryption algorithm: {}", algorithm_name),
            );
            return;
        }
    };

    // Create ArrayBuffer with decrypted data
    let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(decrypted_data).make_shared();
    let array_buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);

    // Create and resolve promise
    let resolver = v8::PromiseResolver::new(scope).unwrap();
    let promise = resolver.get_promise(scope);
    resolver.resolve(scope, array_buffer.into());

    rv.set(promise.into());
}

// crypto.subtle.generateKey(algorithm, extractable, keyUsages)
fn crypto_subtle_generate_key(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 3, "crypto.subtle.generateKey") {
        return;
    }

    // Get algorithm
    let algorithm = args.get(0);
    if !algorithm.is_object() {
        crate::error::throw_type_error(scope, "Algorithm must be an object");
        return;
    }

    let alg_obj = v8::Local::<v8::Object>::try_from(algorithm).unwrap();
    let name_key = v8::String::new(scope, "name").unwrap();
    let name_value = alg_obj.get(scope, name_key.into()).unwrap();
    let algorithm_name = {
        v8::tc_scope!(let tc, scope);
        name_value.to_string(tc).unwrap().to_rust_string_lossy(tc)
    };

    // Get extractable
    let extractable = args.get(1).boolean_value(scope);

    // Get key usages (array of strings)
    let usages = args.get(2);
    if !usages.is_array() {
        crate::error::throw_type_error(scope, "Key usages must be an array");
        return;
    }

    // Generate key based on algorithm
    let key_data = match algorithm_name.as_str() {
        "AES-GCM" => {
            // Get length from algorithm
            let length_key = v8::String::new(scope, "length").unwrap();
            let length_value = alg_obj.get(scope, length_key.into()).unwrap();
            let length = length_value.uint32_value(scope).unwrap_or(256);

            if length != 128 && length != 192 && length != 256 {
                crate::error::throw_error(scope, "AES key length must be 128, 192, or 256");
                return;
            }

            let byte_length = (length / 8) as usize;
            let mut key_bytes = vec![0u8; byte_length];
            let state = crate::isolate_state::IsolateState::get(scope);
            let state_ref = state.borrow();
            if state_ref.system_random.fill(&mut key_bytes).is_err() {
                drop(state_ref);
                crate::error::throw_error(scope, "Failed to generate random key");
                return;
            }
            drop(state_ref);
            key_bytes
        }
        "HMAC" => {
            // Get hash algorithm
            let hash_key = v8::String::new(scope, "hash").unwrap();
            let hash_value = alg_obj.get(scope, hash_key.into()).unwrap();

            let hash_name = if hash_value.is_string() {
                v8::tc_scope!(let tc, scope);
                hash_value.to_string(tc).unwrap().to_rust_string_lossy(tc)
            } else if hash_value.is_object() {
                let hash_obj = v8::Local::<v8::Object>::try_from(hash_value).unwrap();
                let name_key = v8::String::new(scope, "name").unwrap();
                let name_val = hash_obj.get(scope, name_key.into()).unwrap();
                v8::tc_scope!(let tc, scope);
                name_val.to_string(tc).unwrap().to_rust_string_lossy(tc)
            } else {
                crate::error::throw_error(scope, "Invalid hash algorithm");
                return;
            };

            // Get optional length
            let length_key = v8::String::new(scope, "length").unwrap();
            let length_value = alg_obj.get(scope, length_key.into());
            let byte_length = if let Some(len_val) = length_value {
                if len_val.is_number() {
                    (len_val.uint32_value(scope).unwrap_or(512) / 8) as usize
                } else {
                    // Default based on hash
                    match hash_name.as_str() {
                        "SHA-256" => 32,
                        "SHA-384" => 48,
                        "SHA-512" => 64,
                        _ => 32,
                    }
                }
            } else {
                // Default based on hash
                match hash_name.as_str() {
                    "SHA-256" => 32,
                    "SHA-384" => 48,
                    "SHA-512" => 64,
                    _ => 32,
                }
            };

            let mut key_bytes = vec![0u8; byte_length];
            let state = crate::isolate_state::IsolateState::get(scope);
            let state_ref = state.borrow();
            if state_ref.system_random.fill(&mut key_bytes).is_err() {
                drop(state_ref);
                crate::error::throw_error(scope, "Failed to generate random key");
                return;
            }
            drop(state_ref);
            key_bytes
        }
        _ => {
            crate::error::throw_error(
                scope,
                &format!("Unsupported key generation algorithm: {}", algorithm_name),
            );
            return;
        }
    };

    // Create key object
    // Store key data in ArrayBuffer
    let key_backing_store = v8::ArrayBuffer::new_backing_store_from_vec(key_data).make_shared();
    let key_buffer = v8::ArrayBuffer::with_backing_store(scope, &key_backing_store);

    let key_obj = v8::Object::new(scope);

    // Set algorithm
    let alg_key = v8::String::new(scope, "algorithm").unwrap();
    key_obj.set(scope, alg_key.into(), algorithm);

    // Set extractable
    let ext_key = v8::String::new(scope, "extractable").unwrap();
    let ext_val = v8::Boolean::new(scope, extractable);
    key_obj.set(scope, ext_key.into(), ext_val.into());

    // Set usages
    let usages_key = v8::String::new(scope, "usages").unwrap();
    key_obj.set(scope, usages_key.into(), usages);

    // Set type
    let type_key = v8::String::new(scope, "type").unwrap();
    let type_val = v8::String::new(scope, "secret").unwrap();
    key_obj.set(scope, type_key.into(), type_val.into());

    // Set key data (private property)
    let key_data_key = v8::String::new(scope, "_keyData").unwrap();
    key_obj.set(scope, key_data_key.into(), key_buffer.into());

    // Create and resolve promise
    let resolver = v8::PromiseResolver::new(scope).unwrap();
    let promise = resolver.get_promise(scope);
    resolver.resolve(scope, key_obj.into());

    rv.set(promise.into());
}

// crypto.subtle.importKey(format, keyData, algorithm, extractable, keyUsages)
fn crypto_subtle_import_key(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 5, "crypto.subtle.importKey") {
        return;
    }

    // Get format
    let format = args.get(0);
    let format_str = {
        v8::tc_scope!(let tc, scope);
        format.to_string(tc).unwrap().to_rust_string_lossy(tc)
    };

    // Get key data
    let key_data = args.get(1);
    let key_bytes = match get_buffer_data(scope, key_data) {
        Some(bytes) => bytes.to_vec(),
        None => {
            crate::error::throw_type_error(
                scope,
                "Key data must be an ArrayBuffer or ArrayBufferView",
            );
            return;
        }
    };

    // Get algorithm
    let algorithm = args.get(2);
    if !algorithm.is_object() {
        crate::error::throw_type_error(scope, "Algorithm must be an object");
        return;
    }

    // Get extractable
    let extractable = args.get(3).boolean_value(scope);

    // Get key usages
    let usages = args.get(4);
    if !usages.is_array() {
        crate::error::throw_type_error(scope, "Key usages must be an array");
        return;
    }

    // Validate format
    if format_str != "raw" {
        crate::error::throw_error(
            scope,
            &format!(
                "Unsupported key format: {} (only 'raw' is supported)",
                format_str
            ),
        );
        return;
    }

    // Create key object
    let key_backing_store = v8::ArrayBuffer::new_backing_store_from_vec(key_bytes).make_shared();
    let key_buffer = v8::ArrayBuffer::with_backing_store(scope, &key_backing_store);

    let key_obj = v8::Object::new(scope);

    // Set algorithm
    let alg_key = v8::String::new(scope, "algorithm").unwrap();
    key_obj.set(scope, alg_key.into(), algorithm);

    // Set extractable
    let ext_key = v8::String::new(scope, "extractable").unwrap();
    let ext_val = v8::Boolean::new(scope, extractable);
    key_obj.set(scope, ext_key.into(), ext_val.into());

    // Set usages
    let usages_key = v8::String::new(scope, "usages").unwrap();
    key_obj.set(scope, usages_key.into(), usages);

    // Set type
    let type_key = v8::String::new(scope, "type").unwrap();
    let type_val = v8::String::new(scope, "secret").unwrap();
    key_obj.set(scope, type_key.into(), type_val.into());

    // Set key data (private property)
    let key_data_key = v8::String::new(scope, "_keyData").unwrap();
    key_obj.set(scope, key_data_key.into(), key_buffer.into());

    // Create and resolve promise
    let resolver = v8::PromiseResolver::new(scope).unwrap();
    let promise = resolver.get_promise(scope);
    resolver.resolve(scope, key_obj.into());

    rv.set(promise.into());
}

// crypto.subtle.exportKey(format, key)
fn crypto_subtle_export_key(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 2, "crypto.subtle.exportKey") {
        return;
    }

    // Get format
    let format = args.get(0);
    let format_str = {
        v8::tc_scope!(let tc, scope);
        format.to_string(tc).unwrap().to_rust_string_lossy(tc)
    };

    // Get key
    let key = args.get(1);
    if !key.is_object() {
        crate::error::throw_type_error(scope, "Key must be an object");
        return;
    }

    let key_obj = v8::Local::<v8::Object>::try_from(key).unwrap();

    // Check if key is extractable
    let ext_key = v8::String::new(scope, "extractable").unwrap();
    let ext_value = key_obj.get(scope, ext_key.into()).unwrap();
    if !ext_value.boolean_value(scope) {
        crate::error::throw_error(scope, "Key is not extractable");
        return;
    }

    // Validate format
    if format_str != "raw" {
        crate::error::throw_error(
            scope,
            &format!(
                "Unsupported key format: {} (only 'raw' is supported)",
                format_str
            ),
        );
        return;
    }

    // Get key data
    let key_data_prop = v8::String::new(scope, "_keyData").unwrap();
    let key_data_val = key_obj.get(scope, key_data_prop.into()).unwrap();

    let key_bytes = match get_buffer_data(scope, key_data_val) {
        Some(bytes) => bytes.to_vec(),
        None => {
            crate::error::throw_type_error(scope, "Invalid key data");
            return;
        }
    };

    // Create ArrayBuffer with key data
    let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(key_bytes).make_shared();
    let array_buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);

    // Create and resolve promise
    let resolver = v8::PromiseResolver::new(scope).unwrap();
    let promise = resolver.get_promise(scope);
    resolver.resolve(scope, array_buffer.into());

    rv.set(promise.into());
}
