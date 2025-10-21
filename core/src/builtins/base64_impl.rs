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
    if args.length() == 0 {
        let message = v8::String::new(scope, "atob requires at least 1 argument").unwrap();
        let exception = v8::Exception::type_error(scope, message);
        scope.throw_exception(exception);
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
        let message = v8::String::new(scope, "Failed to convert argument to string").unwrap();
        let exception = v8::Exception::type_error(scope, message);
        scope.throw_exception(exception);
        return;
    }

    // Decode base64
    let decoded = match base64_decode(&input_str) {
        Ok(bytes) => bytes,
        Err(e) => {
            let message = v8::String::new(scope, &e).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
            return;
        }
    };

    // Convert bytes to string
    let result_str = match String::from_utf8(decoded.clone()) {
        Ok(s) => s,
        Err(_) => {
            // If it's not valid UTF-8, convert bytes to Latin-1 string
            // This matches browser behavior for atob
            decoded.iter().map(|&b| b as char).collect()
        }
    };

    let result = v8::String::new(scope, &result_str).unwrap();
    rv.set(result.into());
}

// btoa: encode binary string to base64
fn btoa(scope: &mut v8::PinScope, args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue) {
    if args.length() == 0 {
        let message = v8::String::new(scope, "btoa requires at least 1 argument").unwrap();
        let exception = v8::Exception::type_error(scope, message);
        scope.throw_exception(exception);
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
        let message = v8::String::new(scope, "Failed to convert argument to string").unwrap();
        let exception = v8::Exception::type_error(scope, message);
        scope.throw_exception(exception);
        return;
    }

    // Check if string contains characters outside the Latin-1 range
    for ch in input_str.chars() {
        if ch as u32 > 0xFF {
            let message = v8::String::new(
                scope,
                "The string to be encoded contains characters outside of the Latin1 range.",
            )
            .unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
            return;
        }
    }

    // Convert string to bytes (Latin-1 encoding)
    let bytes: Vec<u8> = input_str.chars().map(|c| c as u8).collect();

    // Encode to base64
    let encoded = base64_encode(&bytes);

    let result = v8::String::new(scope, &encoded).unwrap();
    rv.set(result.into());
}

// Base64 encoding/decoding helper functions
fn base64_encode(input: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    let mut i = 0;
    while i < input.len() {
        let b1 = input[i];
        let b2 = if i + 1 < input.len() { input[i + 1] } else { 0 };
        let b3 = if i + 2 < input.len() { input[i + 2] } else { 0 };

        let enc1 = b1 >> 2;
        let enc2 = ((b1 & 0x03) << 4) | (b2 >> 4);
        let enc3 = ((b2 & 0x0F) << 2) | (b3 >> 6);
        let enc4 = b3 & 0x3F;

        result.push(ALPHABET[enc1 as usize] as char);
        result.push(ALPHABET[enc2 as usize] as char);

        if i + 1 < input.len() {
            result.push(ALPHABET[enc3 as usize] as char);
        } else {
            result.push('=');
        }

        if i + 2 < input.len() {
            result.push(ALPHABET[enc4 as usize] as char);
        } else {
            result.push('=');
        }

        i += 3;
    }

    result
}

fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    // Remove whitespace
    let input: String = input.chars().filter(|c| !c.is_whitespace()).collect();

    if input.is_empty() {
        return Ok(Vec::new());
    }

    // Validate length (must be multiple of 4)
    if !input.len().is_multiple_of(4) {
        return Err("Invalid base64 string length".to_string());
    }

    let mut result = Vec::new();

    for chunk in input.as_bytes().chunks(4) {
        let b1 = decode_char(chunk[0] as char)?;
        let b2 = decode_char(chunk[1] as char)?;
        let b3 = if chunk[2] == b'=' {
            0
        } else {
            decode_char(chunk[2] as char)?
        };
        let b4 = if chunk[3] == b'=' {
            0
        } else {
            decode_char(chunk[3] as char)?
        };

        result.push((b1 << 2) | (b2 >> 4));
        if chunk[2] != b'=' {
            result.push(((b2 & 0x0F) << 4) | (b3 >> 2));
        }
        if chunk[3] != b'=' {
            result.push(((b3 & 0x03) << 6) | b4);
        }
    }

    Ok(result)
}

fn decode_char(c: char) -> Result<u8, String> {
    match c {
        'A'..='Z' => Ok((c as u8) - b'A'),
        'a'..='z' => Ok((c as u8) - b'a' + 26),
        '0'..='9' => Ok((c as u8) - b'0' + 52),
        '+' => Ok(62),
        '/' => Ok(63),
        '=' => Ok(0),
        _ => Err(format!("Invalid base64 character: {}", c)),
    }
}
