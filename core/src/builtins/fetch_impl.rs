pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![v8::ExternalReference {
        function: v8::MapFnTo::map_fn_to(fetch_send),
    }]
}

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    let name = v8::String::new(scope, "fetchSend").unwrap();
    let value = v8::Function::new(scope, fetch_send).unwrap();
    bindings.set(scope, name.into(), value.into());
}

// Helper to convert v8 string to Rust string
fn to_rust_string(scope: &mut v8::PinScope, value: v8::Local<v8::Value>) -> String {
    value.to_string(scope).unwrap().to_rust_string_lossy(scope)
}

fn fetch_send(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    // Extract arguments: url, method, headers, body
    let url = to_rust_string(scope, args.get(0));
    let method = to_rust_string(scope, args.get(1));

    // Extract headers array
    let headers_array = v8::Local::<v8::Array>::try_from(args.get(2)).unwrap();
    let headers_len = headers_array.length();
    let mut headers = Vec::new();

    for i in 0..headers_len {
        let entry = headers_array.get_index(scope, i).unwrap();
        let entry_array = v8::Local::<v8::Array>::try_from(entry).unwrap();
        let key = to_rust_string(scope, entry_array.get_index(scope, 0).unwrap());
        let value = to_rust_string(scope, entry_array.get_index(scope, 1).unwrap());
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
    let resolver = v8::PromiseResolver::new(scope).unwrap();
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
