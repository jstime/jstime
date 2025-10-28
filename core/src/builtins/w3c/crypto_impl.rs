use ring::digest;
use ring::rand::{SecureRandom, SystemRandom};

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

    // Fill with random bytes
    let rng = SystemRandom::new();
    if rng.fill(data).is_err() {
        crate::error::throw_error(scope, "Failed to generate random values");
        return;
    }

    // Return the same array
    rv.set(array);
}

// crypto.randomUUID()
fn crypto_random_uuid(
    scope: &mut v8::PinScope,
    _args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let rng = SystemRandom::new();
    let mut bytes = [0u8; 16];
    if rng.fill(&mut bytes).is_err() {
        crate::error::throw_error(scope, "Failed to generate random UUID");
        return;
    }

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
