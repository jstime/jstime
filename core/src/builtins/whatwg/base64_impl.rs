pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(atob),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(btoa),
        },
    ]
}

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    let name = v8::String::new(scope, "atob").unwrap();
    let value = v8::Function::new(scope, atob).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "btoa").unwrap();
    let value = v8::Function::new(scope, btoa).unwrap();
    bindings.set(scope, name.into(), value.into());
}

// atob: decode base64 string to binary string
fn atob(scope: &mut v8::PinScope, args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue) {
    if !crate::error::check_arg_count(scope, &args, 1, "atob") {
        return;
    }

    let input = args.get(0);

    // Convert input to string using tc_scope
    let input_str = {
        v8::tc_scope!(let tc, scope);
        match input.to_string(tc) {
            Some(s) => s.to_rust_string_lossy(tc),
            None => String::new(), // Return empty string on failure, check below
        }
    };

    // Check if conversion failed
    if input.is_null_or_undefined() || input_str.is_empty() && !input.is_string() {
        crate::error::throw_type_error(scope, "Failed to convert argument to string");
        return;
    }

    // Convert to bytes for in-place decoding
    // This follows the "forgiving base64" spec which removes ASCII whitespace
    let mut input_bytes = input_str.into_bytes();

    // Remove ASCII whitespace first (per forgiving base64 spec)
    input_bytes.retain(|&b| !b.is_ascii_whitespace());

    // Validate length is multiple of 4 (per WHATWG spec)
    if input_bytes.len() % 4 != 0 {
        crate::error::throw_error(scope, "Invalid base64 string length");
        return;
    }

    // Decode base64 in-place
    let decoded = match base64_simd::forgiving_decode_inplace(&mut input_bytes) {
        Ok(decoded_slice) => decoded_slice,
        Err(_) => {
            crate::error::throw_error(scope, "Invalid base64 string");
            return;
        }
    };

    // Convert bytes to Latin-1 string using V8's optimized one-byte string creation
    // This is much faster than converting to Rust String and pushing chars one by one
    let result = v8::String::new_from_one_byte(scope, decoded, v8::NewStringType::Normal).unwrap();
    rv.set(result.into());
}

// btoa: encode binary string to base64
fn btoa(scope: &mut v8::PinScope, args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue) {
    if !crate::error::check_arg_count(scope, &args, 1, "btoa") {
        return;
    }

    let input = args.get(0);

    // Check if conversion would fail
    if input.is_null_or_undefined() {
        crate::error::throw_type_error(scope, "Failed to convert argument to string");
        return;
    }

    // Convert input to string using tc_scope
    let input_str = {
        v8::tc_scope!(let tc, scope);
        input.to_string(tc)
    };

    let input_str = match input_str {
        Some(s) => s,
        None => {
            crate::error::throw_type_error(scope, "Failed to convert argument to string");
            return;
        }
    };

    let v8_str_len = input_str.length() as usize;

    // Fast path: if string contains only one-byte (Latin-1) characters, read directly
    if input_str.contains_only_onebyte() {
        // For one-byte strings, the V8 string length equals the byte count
        let mut bytes = vec![0u8; v8_str_len];
        input_str.write_one_byte_v2(scope, 0, &mut bytes, v8::WriteFlags::empty());

        // Encode to base64 using SIMD-optimized encoder
        let encoded = base64_simd::STANDARD.encode_to_string(&bytes);

        let result = v8::String::new(scope, &encoded).unwrap();
        rv.set(result.into());
    } else {
        // Slow path: string contains multi-byte characters, validate Latin-1 range
        let input_str_rust = input_str.to_rust_string_lossy(scope);
        // Reserve capacity for the byte vector. For Latin-1 strings, this will be
        // the character count, but we use str_len as a reasonable upper bound
        let mut bytes = Vec::with_capacity(v8_str_len);

        for ch in input_str_rust.chars() {
            if ch as u32 > 0xFF {
                crate::error::throw_error(
                    scope,
                    "The string to be encoded contains characters outside of the Latin1 range.",
                );
                return;
            }
            bytes.push(ch as u8);
        }

        // Encode to base64 using SIMD-optimized encoder
        let encoded = base64_simd::STANDARD.encode_to_string(&bytes);

        let result = v8::String::new(scope, &encoded).unwrap();
        rv.set(result.into());
    }
}
