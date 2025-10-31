use base64::{Engine, engine::general_purpose::STANDARD};

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

    // Remove whitespace if present (spec: https://html.spec.whatwg.org/multipage/webappapis.html#atob)
    // Optimize by checking for whitespace and only allocating if necessary
    let input_bytes = input_str.as_bytes();

    // Decode base64 (handles whitespace in input)
    let decoded = if input_bytes.iter().any(|&b| b.is_ascii_whitespace()) {
        // Only allocate a new Vec if there's whitespace to filter
        let filtered: Vec<u8> = input_bytes
            .iter()
            .copied()
            .filter(|&b| !b.is_ascii_whitespace())
            .collect();
        match STANDARD.decode(&filtered) {
            Ok(bytes) => bytes,
            Err(e) => {
                crate::error::throw_error(scope, &format!("Invalid base64 string: {}", e));
                return;
            }
        }
    } else {
        // No whitespace, decode directly without allocation
        match STANDARD.decode(input_bytes) {
            Ok(bytes) => bytes,
            Err(e) => {
                crate::error::throw_error(scope, &format!("Invalid base64 string: {}", e));
                return;
            }
        }
    };

    // Convert bytes to Latin-1 string (each byte becomes a character)
    // This matches browser behavior for atob
    // Optimize by pre-allocating and avoiding iterator overhead
    let mut result_str = String::with_capacity(decoded.len());
    for &byte in &decoded {
        // SAFETY: Latin-1 bytes map directly to Unicode code points 0-255
        result_str.push(byte as char);
    }

    let result = v8::String::new(scope, &result_str).unwrap();
    rv.set(result.into());
}

// btoa: encode binary string to base64
fn btoa(scope: &mut v8::PinScope, args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue) {
    if !crate::error::check_arg_count(scope, &args, 1, "btoa") {
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

    // Check if conversion failed (skip for legitimate empty strings)
    if input.is_null_or_undefined() || input_str.is_empty() && !input.is_string() {
        crate::error::throw_type_error(scope, "Failed to convert argument to string");
        return;
    }

    // Check if string contains characters outside the Latin-1 range
    // and convert to bytes simultaneously
    // Use bytes() instead of chars() for better performance since we're checking ASCII/Latin-1
    let input_bytes = input_str.as_bytes();
    let mut bytes = Vec::with_capacity(input_bytes.len());

    // Fast path: if input is already ASCII, just copy it
    if input_str.is_ascii() {
        bytes.extend_from_slice(input_bytes);
    } else {
        // Slow path: validate UTF-8 characters are in Latin-1 range
        for ch in input_str.chars() {
            if ch as u32 > 0xFF {
                crate::error::throw_error(
                    scope,
                    "The string to be encoded contains characters outside of the Latin1 range.",
                );
                return;
            }
            bytes.push(ch as u8);
        }
    }

    // Encode to base64
    let encoded = STANDARD.encode(&bytes);

    let result = v8::String::new(scope, &encoded).unwrap();
    rv.set(result.into());
}
