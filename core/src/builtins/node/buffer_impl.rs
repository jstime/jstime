//! Node.js Buffer API implementation
//! https://nodejs.org/api/buffer.html

pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(buffer_alloc),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(buffer_from),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(buffer_concat),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(buffer_byte_length),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(buffer_compare),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(buffer_is_encoding),
        },
    ]
}

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    let name = v8::String::new(scope, "bufferAlloc").unwrap();
    let value = v8::Function::new(scope, buffer_alloc).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "bufferFrom").unwrap();
    let value = v8::Function::new(scope, buffer_from).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "bufferConcat").unwrap();
    let value = v8::Function::new(scope, buffer_concat).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "bufferByteLength").unwrap();
    let value = v8::Function::new(scope, buffer_byte_length).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "bufferCompare").unwrap();
    let value = v8::Function::new(scope, buffer_compare).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "bufferIsEncoding").unwrap();
    let value = v8::Function::new(scope, buffer_is_encoding).unwrap();
    bindings.set(scope, name.into(), value.into());
}

/// Check if the encoding is supported
#[inline]
fn is_valid_encoding(encoding: &str) -> bool {
    matches!(
        encoding.to_lowercase().as_str(),
        "utf8" | "utf-8" | "hex" | "base64" | "base64url" | "latin1" | "binary" | "ascii"
    )
}

/// Buffer.alloc(size[, fill[, encoding]]) - Allocates a new Buffer of size bytes
#[inline]
fn buffer_alloc(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 1, "Buffer.alloc") {
        return;
    }

    let size_arg = args.get(0);
    if !size_arg.is_number() {
        crate::error::throw_type_error(scope, "The \"size\" argument must be of type number");
        return;
    }

    let size = size_arg.number_value(scope).unwrap_or(0.0) as usize;

    // Check for maximum buffer size (same as Node.js: 2^32 - 1 bytes)
    const MAX_BUFFER_SIZE: usize = 0xFFFFFFFF;
    if size > MAX_BUFFER_SIZE {
        crate::error::throw_error(
            scope,
            &format!(
                "Cannot create a Buffer larger than {} bytes",
                MAX_BUFFER_SIZE
            ),
        );
        return;
    }

    // Check for fill value
    let fill_value = if args.length() >= 2 && !args.get(1).is_undefined() {
        let fill_arg = args.get(1);
        if fill_arg.is_number() {
            Some(fill_arg.number_value(scope).unwrap_or(0.0) as u8)
        } else if fill_arg.is_string() {
            // For string fill, we just use the first byte for simplicity
            let fill_str = {
                v8::tc_scope!(let tc, scope);
                fill_arg.to_string(tc).map(|s| s.to_rust_string_lossy(tc))
            };
            fill_str.and_then(|s| s.bytes().next())
        } else {
            None
        }
    } else {
        None
    };

    // Create the buffer
    let data = if let Some(fill) = fill_value {
        vec![fill; size]
    } else {
        vec![0u8; size]
    };

    let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(data).make_shared();
    let array_buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);
    let uint8_array = v8::Uint8Array::new(scope, array_buffer, 0, size).unwrap();

    retval.set(uint8_array.into());
}

/// Buffer.from(source[, encoding]) - Create a new Buffer from various sources
#[inline]
fn buffer_from(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if args.length() < 1 {
        crate::error::throw_type_error(
            scope,
            "The first argument must be of type string or an instance of Buffer, ArrayBuffer, or Array",
        );
        return;
    }

    let source = args.get(0);

    // Get encoding (default to utf8)
    let encoding = if args.length() >= 2 && !args.get(1).is_undefined() {
        let enc_arg = args.get(1);
        v8::tc_scope!(let tc, scope);
        enc_arg
            .to_string(tc)
            .map(|s| s.to_rust_string_lossy(tc))
            .unwrap_or_else(|| "utf8".to_string())
    } else {
        "utf8".to_string()
    };

    // Handle string source
    if source.is_string() {
        let source_str = {
            v8::tc_scope!(let tc, scope);
            source
                .to_string(tc)
                .map(|s| s.to_rust_string_lossy(tc))
                .unwrap_or_default()
        };

        let bytes = encode_string(&source_str, &encoding);
        match bytes {
            Ok(data) => {
                let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(data).make_shared();
                let len = backing_store.len();
                let array_buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);
                let uint8_array = v8::Uint8Array::new(scope, array_buffer, 0, len).unwrap();
                retval.set(uint8_array.into());
            }
            Err(e) => {
                crate::error::throw_error(scope, &e);
            }
        }
        return;
    }

    // Handle ArrayBuffer source
    if source.is_array_buffer() {
        let Some(array_buffer) = v8::Local::<v8::ArrayBuffer>::try_from(source).ok() else {
            crate::error::throw_type_error(scope, "Failed to convert to ArrayBuffer");
            return;
        };
        let byte_length = array_buffer.byte_length();

        // Copy the ArrayBuffer data
        let backing_store = array_buffer.get_backing_store();
        let mut data = vec![0u8; byte_length];
        if byte_length > 0
            && let Some(store_data) = backing_store.data()
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    store_data.as_ptr() as *const u8,
                    data.as_mut_ptr(),
                    byte_length,
                );
            }
        }

        let new_backing_store = v8::ArrayBuffer::new_backing_store_from_vec(data).make_shared();
        let new_array_buffer = v8::ArrayBuffer::with_backing_store(scope, &new_backing_store);
        let uint8_array =
            v8::Uint8Array::new(scope, new_array_buffer, 0, new_backing_store.len()).unwrap();
        retval.set(uint8_array.into());
        return;
    }

    // Handle TypedArray or DataView source (including Uint8Array/Buffer)
    if source.is_array_buffer_view() {
        let Some(view) = v8::Local::<v8::ArrayBufferView>::try_from(source).ok() else {
            crate::error::throw_type_error(scope, "Failed to convert to ArrayBufferView");
            return;
        };
        let byte_length = view.byte_length();

        // Copy the data
        let mut data = vec![0u8; byte_length];
        if byte_length > 0 {
            let view_data = view.data();
            unsafe {
                std::ptr::copy_nonoverlapping(
                    view_data as *const u8,
                    data.as_mut_ptr(),
                    byte_length,
                );
            }
        }

        let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(data).make_shared();
        let array_buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);
        let uint8_array = v8::Uint8Array::new(scope, array_buffer, 0, backing_store.len()).unwrap();
        retval.set(uint8_array.into());
        return;
    }

    // Handle Array-like source
    if source.is_array() {
        let Some(array) = v8::Local::<v8::Array>::try_from(source).ok() else {
            crate::error::throw_type_error(scope, "Failed to convert to Array");
            return;
        };
        let length = array.length() as usize;
        let mut data = Vec::with_capacity(length);

        for i in 0..length {
            if let Some(value) = array.get_index(scope, i as u32) {
                let byte = if value.is_number() {
                    (value.number_value(scope).unwrap_or(0.0) as i32 & 0xFF) as u8
                } else {
                    0
                };
                data.push(byte);
            } else {
                data.push(0);
            }
        }

        let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(data).make_shared();
        let len = backing_store.len();
        let array_buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);
        let uint8_array = v8::Uint8Array::new(scope, array_buffer, 0, len).unwrap();
        retval.set(uint8_array.into());
        return;
    }

    crate::error::throw_type_error(
        scope,
        "The first argument must be of type string or an instance of Buffer, ArrayBuffer, or Array",
    );
}

/// Encode a string using the specified encoding
fn encode_string(input: &str, encoding: &str) -> Result<Vec<u8>, String> {
    let encoding_lower = encoding.to_lowercase();
    match encoding_lower.as_str() {
        "utf8" | "utf-8" => Ok(input.as_bytes().to_vec()),
        "latin1" | "binary" => {
            // Latin1 encoding - each character becomes a single byte
            Ok(input.chars().map(|c| (c as u32 & 0xFF) as u8).collect())
        }
        "ascii" => {
            // ASCII encoding - same as latin1 but mask to 7 bits
            Ok(input.chars().map(|c| (c as u32 & 0x7F) as u8).collect())
        }
        "hex" => {
            // Hex decoding
            if !input.len().is_multiple_of(2) {
                return Err("Invalid hex string".to_string());
            }
            let mut bytes = Vec::with_capacity(input.len() / 2);
            let chars: Vec<char> = input.chars().collect();
            for i in (0..chars.len()).step_by(2) {
                let high = chars[i]
                    .to_digit(16)
                    .ok_or_else(|| "Invalid hex character".to_string())?
                    as u8;
                let low = chars[i + 1]
                    .to_digit(16)
                    .ok_or_else(|| "Invalid hex character".to_string())?
                    as u8;
                bytes.push((high << 4) | low);
            }
            Ok(bytes)
        }
        "base64" => {
            // Base64 decoding
            base64_decode(input)
        }
        "base64url" => {
            // Base64url decoding - replace URL-safe characters
            let standard = input.replace('-', "+").replace('_', "/");
            base64_decode(&standard)
        }
        _ => Err(format!("Unknown encoding: {}", encoding)),
    }
}

/// Decode base64 string to bytes
fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    // Remove whitespace
    let clean: String = input.chars().filter(|c| !c.is_whitespace()).collect();

    // Pad if necessary
    let padded = match clean.len() % 4 {
        2 => format!("{}==", clean),
        3 => format!("{}=", clean),
        _ => clean,
    };

    // Use base64-simd for decoding
    let mut input_bytes = padded.into_bytes();
    match base64_simd::forgiving_decode_inplace(&mut input_bytes) {
        Ok(decoded) => Ok(decoded.to_vec()),
        Err(_) => Err("Invalid base64 string".to_string()),
    }
}

/// Buffer.concat(list[, totalLength]) - Concatenate multiple buffers
#[inline]
fn buffer_concat(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 1, "Buffer.concat") {
        return;
    }

    let list_arg = args.get(0);
    if !list_arg.is_array() {
        crate::error::throw_type_error(scope, "The \"list\" argument must be an instance of Array");
        return;
    }

    let Some(list) = v8::Local::<v8::Array>::try_from(list_arg).ok() else {
        crate::error::throw_type_error(scope, "Failed to convert to Array");
        return;
    };

    let list_length = list.length() as usize;

    // Calculate total length
    let total_length = if args.length() >= 2 && !args.get(1).is_undefined() {
        let len_arg = args.get(1);
        if !len_arg.is_number() {
            crate::error::throw_type_error(
                scope,
                "The \"totalLength\" argument must be of type number",
            );
            return;
        }
        len_arg.number_value(scope).unwrap_or(0.0) as usize
    } else {
        // Calculate from list
        let mut total = 0usize;
        for i in 0..list_length {
            if let Some(item) = list.get_index(scope, i as u32)
                && item.is_array_buffer_view()
            {
                let view = v8::Local::<v8::ArrayBufferView>::try_from(item).unwrap();
                total += view.byte_length();
            }
        }
        total
    };

    // Concatenate buffers
    let mut result = Vec::with_capacity(total_length);
    let mut offset = 0;

    for i in 0..list_length {
        if offset >= total_length {
            break;
        }
        if let Some(item) = list.get_index(scope, i as u32)
            && item.is_array_buffer_view()
        {
            let view = v8::Local::<v8::ArrayBufferView>::try_from(item).unwrap();
            let byte_length = view.byte_length();
            let copy_length = byte_length.min(total_length - offset);

            if copy_length > 0 {
                let view_data = view.data();
                unsafe {
                    let src = std::slice::from_raw_parts(view_data as *const u8, copy_length);
                    result.extend_from_slice(src);
                }
            }
            offset += copy_length;
        }
    }

    // Pad with zeros if necessary
    while result.len() < total_length {
        result.push(0);
    }

    let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(result).make_shared();
    let len = backing_store.len();
    let array_buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);
    let uint8_array = v8::Uint8Array::new(scope, array_buffer, 0, len).unwrap();

    retval.set(uint8_array.into());
}

/// Buffer.byteLength(string[, encoding]) - Returns the byte length of a string
#[inline]
fn buffer_byte_length(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 1, "Buffer.byteLength") {
        return;
    }

    let source = args.get(0);

    // Handle TypedArray/Buffer
    if source.is_array_buffer_view() {
        let view = v8::Local::<v8::ArrayBufferView>::try_from(source).unwrap();
        let byte_length = v8::Number::new(scope, view.byte_length() as f64);
        retval.set(byte_length.into());
        return;
    }

    // Handle ArrayBuffer
    if source.is_array_buffer() {
        let array_buffer = v8::Local::<v8::ArrayBuffer>::try_from(source).unwrap();
        let byte_length = v8::Number::new(scope, array_buffer.byte_length() as f64);
        retval.set(byte_length.into());
        return;
    }

    // Handle string
    if source.is_string() {
        let source_str = {
            v8::tc_scope!(let tc, scope);
            source
                .to_string(tc)
                .map(|s| s.to_rust_string_lossy(tc))
                .unwrap_or_default()
        };

        let encoding = if args.length() >= 2 && !args.get(1).is_undefined() {
            let enc_arg = args.get(1);
            v8::tc_scope!(let tc, scope);
            enc_arg
                .to_string(tc)
                .map(|s| s.to_rust_string_lossy(tc))
                .unwrap_or_else(|| "utf8".to_string())
        } else {
            "utf8".to_string()
        };

        let byte_len = calculate_byte_length(&source_str, &encoding);
        let result = v8::Number::new(scope, byte_len as f64);
        retval.set(result.into());
        return;
    }

    crate::error::throw_type_error(
        scope,
        "The \"string\" argument must be of type string or an instance of Buffer or ArrayBuffer",
    );
}

/// Calculate byte length of a string in the specified encoding
fn calculate_byte_length(input: &str, encoding: &str) -> usize {
    let encoding_lower = encoding.to_lowercase();
    match encoding_lower.as_str() {
        "utf8" | "utf-8" => input.len(),
        "latin1" | "binary" | "ascii" => input.chars().count(),
        "hex" => input.len() / 2,
        "base64" | "base64url" => {
            // Base64 produces 4 bytes for every 3 input bytes
            // For decoding, we need to calculate the original length
            let clean_len = input.chars().filter(|c| !c.is_whitespace()).count();
            let padding = input.chars().filter(|&c| c == '=').count();
            (clean_len * 3 / 4).saturating_sub(padding)
        }
        _ => input.len(), // Default to UTF-8
    }
}

/// Buffer.compare(buf1, buf2) - Compare two buffers
#[inline]
fn buffer_compare(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 2, "Buffer.compare") {
        return;
    }

    let buf1 = args.get(0);
    let buf2 = args.get(1);

    if !buf1.is_array_buffer_view() || !buf2.is_array_buffer_view() {
        crate::error::throw_type_error(
            scope,
            "The \"buf1\" and \"buf2\" arguments must be one of type Buffer or Uint8Array",
        );
        return;
    }

    let view1 = v8::Local::<v8::ArrayBufferView>::try_from(buf1).unwrap();
    let view2 = v8::Local::<v8::ArrayBufferView>::try_from(buf2).unwrap();

    let len1 = view1.byte_length();
    let len2 = view2.byte_length();

    let data1 = if len1 > 0 {
        let ptr = view1.data();
        unsafe { std::slice::from_raw_parts(ptr as *const u8, len1) }
    } else {
        &[]
    };

    let data2 = if len2 > 0 {
        let ptr = view2.data();
        unsafe { std::slice::from_raw_parts(ptr as *const u8, len2) }
    } else {
        &[]
    };

    let result = data1.cmp(data2);
    let cmp_result = match result {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    };

    let result = v8::Number::new(scope, cmp_result as f64);
    retval.set(result.into());
}

/// Buffer.isEncoding(encoding) - Returns true if encoding is a supported encoding
#[inline]
fn buffer_is_encoding(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if args.length() < 1 {
        retval.set(v8::Boolean::new(scope, false).into());
        return;
    }

    let encoding_arg = args.get(0);
    if !encoding_arg.is_string() {
        retval.set(v8::Boolean::new(scope, false).into());
        return;
    }

    let encoding = {
        v8::tc_scope!(let tc, scope);
        encoding_arg
            .to_string(tc)
            .map(|s| s.to_rust_string_lossy(tc))
            .unwrap_or_default()
    };

    let is_valid = is_valid_encoding(&encoding);
    retval.set(v8::Boolean::new(scope, is_valid).into());
}
