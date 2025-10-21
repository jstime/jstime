use std::fs;

pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(read_file),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(read_dir),
        },
    ]
}

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    let name = v8::String::new(scope, "readFile").unwrap();
    let value = v8::Function::new(scope, read_file).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "readDir").unwrap();
    let value = v8::Function::new(scope, read_dir).unwrap();
    bindings.set(scope, name.into(), value.into());
}

fn read_file(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    let arg_len = args.length();
    if arg_len < 1 {
        let msg = v8::String::new(scope, "path is required").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    }

    // Get the file path
    let path_arg = args.get(0);
    let isolate: &v8::Isolate = scope;
    let path_str = path_arg
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(isolate);

    // Get the encoding if provided (second argument)
    let encoding = if arg_len >= 2 {
        let encoding_arg = args.get(1);
        if !encoding_arg.is_null_or_undefined() {
            Some(
                encoding_arg
                    .to_string(scope)
                    .unwrap()
                    .to_rust_string_lossy(isolate),
            )
        } else {
            None
        }
    } else {
        None
    };

    // Read the file
    match fs::read(&path_str) {
        Ok(data) => {
            if let Some(enc) = encoding {
                if enc == "utf8" || enc == "utf-8" {
                    // Return as string
                    match String::from_utf8(data) {
                        Ok(text) => {
                            let result = v8::String::new(scope, &text).unwrap();
                            retval.set(result.into());
                        }
                        Err(e) => {
                            let msg =
                                v8::String::new(scope, &format!("Invalid UTF-8: {}", e)).unwrap();
                            let exception = v8::Exception::error(scope, msg);
                            scope.throw_exception(exception);
                        }
                    }
                } else {
                    let msg =
                        v8::String::new(scope, &format!("Unsupported encoding: {}", enc)).unwrap();
                    let exception = v8::Exception::error(scope, msg);
                    scope.throw_exception(exception);
                }
            } else {
                // Return as Uint8Array (Buffer)
                let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(data).make_shared();
                let array_buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);
                let uint8_array =
                    v8::Uint8Array::new(scope, array_buffer, 0, backing_store.len()).unwrap();
                retval.set(uint8_array.into());
            }
        }
        Err(e) => {
            let msg = v8::String::new(scope, &format!("Failed to read file: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn read_dir(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    let arg_len = args.length();
    if arg_len < 1 {
        let msg = v8::String::new(scope, "path is required").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    }

    // Get the directory path
    let path_arg = args.get(0);
    let isolate: &v8::Isolate = scope;
    let path_str = path_arg
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(isolate);

    // Read the directory
    match fs::read_dir(&path_str) {
        Ok(entries) => {
            let array = v8::Array::new(scope, 0);
            let mut index = 0;

            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let file_name = entry.file_name();
                        let file_name_str = file_name.to_string_lossy();
                        let name = v8::String::new(scope, &file_name_str).unwrap();
                        array.set_index(scope, index, name.into());
                        index += 1;
                    }
                    Err(e) => {
                        let msg = v8::String::new(scope, &format!("Failed to read entry: {}", e))
                            .unwrap();
                        let exception = v8::Exception::error(scope, msg);
                        scope.throw_exception(exception);
                        return;
                    }
                }
            }

            retval.set(array.into());
        }
        Err(e) => {
            let msg = v8::String::new(scope, &format!("Failed to read directory: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}
