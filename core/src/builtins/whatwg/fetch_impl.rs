pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(fetch_send),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(fetch_read_chunk),
        },
    ]
}

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    let name = v8::String::new(scope, "fetchSend").unwrap();
    let value = v8::Function::new(scope, fetch_send).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "fetchReadChunk").unwrap();
    let value = v8::Function::new(scope, fetch_read_chunk).unwrap();
    bindings.set(scope, name.into(), value.into());
}

// Helper to convert v8 string to Rust string
fn to_rust_string(scope: &mut v8::PinScope, value: v8::Local<v8::Value>) -> String {
    crate::error::try_to_rust_string(scope, value, "value").unwrap_or_default()
}

fn fetch_send(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    // Extract arguments: url, method, headers, body
    let url = to_rust_string(scope, args.get(0));
    let method = to_rust_string(scope, args.get(1));

    // Extract headers array - use pooled vector
    let headers_array_val = args.get(2);
    let headers_array = match crate::error::try_get_array_result(headers_array_val) {
        Ok(arr) => arr,
        Err(msg) => {
            crate::error::throw_type_error(scope, msg);
            return;
        }
    };
    let headers_len = headers_array.length();

    // Get a pooled vector for headers (pool should return cleared vectors)
    let isolate: &mut v8::Isolate = scope;
    let state = crate::IsolateState::get(isolate);
    let header_pool = state.borrow().header_vec_pool.clone();
    let mut headers = header_pool.get(|| Vec::with_capacity(headers_len as usize));

    for i in 0..headers_len {
        let Some(entry) = headers_array.get_index(scope, i) else {
            continue;
        };
        let entry_array = match crate::error::try_get_array_result(entry) {
            Ok(arr) => arr,
            Err(msg) => {
                crate::error::throw_type_error(scope, msg);
                return;
            }
        };
        let Some(key_val) = entry_array.get_index(scope, 0) else {
            continue;
        };
        let Some(value_val) = entry_array.get_index(scope, 1) else {
            continue;
        };
        let key = to_rust_string(scope, key_val);
        let value = to_rust_string(scope, value_val);
        headers.push((key, value));
    }

    // Extract body (optional)
    let body_arg = args.get(3);
    let body = if body_arg.is_null() || body_arg.is_undefined() {
        None
    } else {
        Some(to_rust_string(scope, body_arg))
    };

    // Create a promise
    let Some(resolver) = v8::PromiseResolver::new(scope) else {
        crate::error::throw_error(scope, "Failed to create promise");
        return;
    };
    let promise = resolver.get_promise(scope);

    // Store the fetch request
    let fetch_request = crate::isolate_state::FetchRequest {
        url,
        method,
        headers,
        body,
        resolver: v8::Global::new(scope, resolver),
    };

    let state = crate::IsolateState::get(scope);
    state
        .borrow()
        .pending_fetches
        .borrow_mut()
        .push(fetch_request);

    // Return the promise
    rv.set(promise.into());
}

// Native function to read a chunk from a streaming fetch
fn fetch_read_chunk(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    // Get stream ID
    let stream_id_val = args.get(0);
    let stream_id = if let Some(num) = stream_id_val.number_value(scope) {
        num as u64
    } else {
        crate::error::throw_type_error(scope, "Stream ID must be a number");
        return;
    };

    // Create a promise for the chunk
    let Some(resolver) = v8::PromiseResolver::new(scope) else {
        crate::error::throw_error(scope, "Failed to create promise");
        return;
    };
    let promise = resolver.get_promise(scope);

    // Get the streaming fetch from state
    let isolate: &mut v8::Isolate = scope;
    let state = crate::IsolateState::get(isolate);
    let streaming_fetches = state.borrow().streaming_fetches.clone();
    let mut fetches_borrow = streaming_fetches.borrow_mut();

    if let Some(mut streaming_fetch) = fetches_borrow.remove(&stream_id) {
        drop(fetches_borrow);

        // Read a chunk from the buffered body data
        const CHUNK_SIZE: usize = 65536; // 64KB chunks

        let remaining = streaming_fetch.body_data.len() - streaming_fetch.offset;

        if remaining == 0 {
            // End of stream - don't put it back
            let obj = v8::Object::new(scope);

            let done_key = v8::String::new(scope, "done").unwrap();
            let done_value = v8::Boolean::new(scope, true);
            obj.set(scope, done_key.into(), done_value.into());

            let value_key = v8::String::new(scope, "value").unwrap();
            let value_value = v8::undefined(scope);
            obj.set(scope, value_key.into(), value_value.into());

            let _ = resolver.resolve(scope, obj.into());
        } else {
            // We have data
            let bytes_to_read = std::cmp::min(CHUNK_SIZE, remaining);
            let chunk_data = streaming_fetch.body_data
                [streaming_fetch.offset..streaming_fetch.offset + bytes_to_read]
                .to_vec();
            streaming_fetch.offset += bytes_to_read;

            // Put the streaming fetch back
            let state = crate::IsolateState::get(isolate);
            let streaming_fetches = state.borrow().streaming_fetches.clone();
            streaming_fetches
                .borrow_mut()
                .insert(stream_id, streaming_fetch);

            // Return the chunk
            let obj = v8::Object::new(scope);

            let done_key = v8::String::new(scope, "done").unwrap();
            let done_value = v8::Boolean::new(scope, false);
            obj.set(scope, done_key.into(), done_value.into());

            let value_key = v8::String::new(scope, "value").unwrap();
            // Create Uint8Array with the data
            let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(chunk_data);
            let backing_store = backing_store.make_shared();
            let array_buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);
            let uint8_array = v8::Uint8Array::new(scope, array_buffer, 0, bytes_to_read).unwrap();
            obj.set(scope, value_key.into(), uint8_array.into());

            let _ = resolver.resolve(scope, obj.into());
        }
    } else {
        // Stream not found
        let error_msg = v8::String::new(scope, "Stream not found").unwrap();
        let error = v8::Exception::error(scope, error_msg);
        let _ = resolver.reject(scope, error);
    }

    rv.set(promise.into());
}
