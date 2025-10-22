pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(text_encoder_encode),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(text_encoder_encode_into),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(text_decoder_decode),
        },
    ]
}

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    let name = v8::String::new(scope, "textEncoderEncode").unwrap();
    let value = v8::Function::new(scope, text_encoder_encode).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "textEncoderEncodeInto").unwrap();
    let value = v8::Function::new(scope, text_encoder_encode_into).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "textDecoderDecode").unwrap();
    let value = v8::Function::new(scope, text_decoder_decode).unwrap();
    bindings.set(scope, name.into(), value.into());
}

// TextEncoder.encode(): Encodes a string into a Uint8Array using UTF-8
fn text_encoder_encode(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() == 0 {
        let message =
            v8::String::new(scope, "TextEncoder.encode requires at least 1 argument").unwrap();
        let exception = v8::Exception::type_error(scope, message);
        scope.throw_exception(exception);
        return;
    }

    let input = args.get(0);

    // Convert input to string
    let input_str = {
        v8::tc_scope!(let tc, scope);
        match input.to_string(tc) {
            Some(s) => s.to_rust_string_lossy(tc),
            None => String::new(),
        }
    };

    // Encode the string as UTF-8
    let bytes = input_str.as_bytes().to_vec();

    // Create a Uint8Array with the encoded bytes
    let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(bytes).make_shared();
    let array_buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);
    let uint8_array = v8::Uint8Array::new(scope, array_buffer, 0, backing_store.len()).unwrap();
    rv.set(uint8_array.into());
}

// TextEncoder.encodeInto(): Encodes a string into an existing Uint8Array
fn text_encoder_encode_into(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 2 {
        let message = v8::String::new(
            scope,
            "TextEncoder.encodeInto requires at least 2 arguments",
        )
        .unwrap();
        let exception = v8::Exception::type_error(scope, message);
        scope.throw_exception(exception);
        return;
    }

    let input = args.get(0);
    let destination = args.get(1);

    // Convert input to string
    let input_str = {
        v8::tc_scope!(let tc, scope);
        match input.to_string(tc) {
            Some(s) => s.to_rust_string_lossy(tc),
            None => String::new(),
        }
    };

    // Ensure destination is a Uint8Array
    if !destination.is_uint8_array() {
        let message = v8::String::new(scope, "Destination must be a Uint8Array").unwrap();
        let exception = v8::Exception::type_error(scope, message);
        scope.throw_exception(exception);
        return;
    }

    let uint8_array = v8::Local::<v8::Uint8Array>::try_from(destination).unwrap();
    let byte_length = uint8_array.byte_length();
    let data = uint8_array.data();

    // Encode the string as UTF-8
    let bytes = input_str.as_bytes();

    // Copy as many bytes as will fit
    let bytes_to_copy = bytes.len().min(byte_length);

    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), data as *mut u8, bytes_to_copy);
    }

    // Count how many UTF-16 code units were read (for the result object)
    // For simplicity, we'll count complete characters that fit
    let mut chars_read = 0;
    let mut bytes_written = 0;

    for ch in input_str.chars() {
        let char_len = ch.len_utf8();
        if bytes_written + char_len <= byte_length {
            chars_read += ch.len_utf16();
            bytes_written += char_len;
        } else {
            break;
        }
    }

    // Return { read: number, written: number }
    let result = v8::Object::new(scope);

    let read_key = v8::String::new(scope, "read").unwrap();
    let read_value = v8::Number::new(scope, chars_read as f64);
    result.set(scope, read_key.into(), read_value.into());

    let written_key = v8::String::new(scope, "written").unwrap();
    let written_value = v8::Number::new(scope, bytes_written as f64);
    result.set(scope, written_key.into(), written_value.into());

    rv.set(result.into());
}

// TextDecoder.decode(): Decodes a Uint8Array into a string using UTF-8
fn text_decoder_decode(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    // If no argument provided, return empty string
    if args.length() == 0 {
        let result = v8::String::new(scope, "").unwrap();
        rv.set(result.into());
        return;
    }

    let input = args.get(0);

    // Handle undefined/null
    if input.is_null_or_undefined() {
        let result = v8::String::new(scope, "").unwrap();
        rv.set(result.into());
        return;
    }

    // Ensure input is a typed array or ArrayBuffer
    let bytes = if input.is_array_buffer_view() {
        let view = v8::Local::<v8::ArrayBufferView>::try_from(input).unwrap();
        let byte_length = view.byte_length();

        if byte_length == 0 {
            &[]
        } else {
            let data = view.data();
            unsafe { std::slice::from_raw_parts(data as *const u8, byte_length) }
        }
    } else if input.is_array_buffer() {
        let array_buffer = v8::Local::<v8::ArrayBuffer>::try_from(input).unwrap();
        let backing_store = array_buffer.get_backing_store();
        let byte_length = array_buffer.byte_length();

        if byte_length == 0 {
            &[]
        } else {
            match backing_store.data() {
                Some(data) => unsafe {
                    std::slice::from_raw_parts(data.as_ptr() as *const u8, byte_length)
                },
                None => {
                    // Empty or detached ArrayBuffer
                    &[]
                }
            }
        }
    } else {
        let message =
            v8::String::new(scope, "Input must be an ArrayBuffer or ArrayBufferView").unwrap();
        let exception = v8::Exception::type_error(scope, message);
        scope.throw_exception(exception);
        return;
    };

    // Decode the bytes as UTF-8
    let decoded = match std::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => {
            // Use lossy conversion for invalid UTF-8
            let owned = String::from_utf8_lossy(bytes);
            let result = v8::String::new(scope, &owned).unwrap();
            rv.set(result.into());
            return;
        }
    };

    let result = v8::String::new(scope, decoded).unwrap();
    rv.set(result.into());
}
