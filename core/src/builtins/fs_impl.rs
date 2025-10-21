use std::fs;

pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(read_file),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(read_dir),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(write_file),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(mkdir),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(rmdir),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(unlink),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(rename),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(copy_file),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(stat),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(access),
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

    let name = v8::String::new(scope, "writeFile").unwrap();
    let value = v8::Function::new(scope, write_file).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "mkdir").unwrap();
    let value = v8::Function::new(scope, mkdir).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "rmdir").unwrap();
    let value = v8::Function::new(scope, rmdir).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "unlink").unwrap();
    let value = v8::Function::new(scope, unlink).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "rename").unwrap();
    let value = v8::Function::new(scope, rename).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "copyFile").unwrap();
    let value = v8::Function::new(scope, copy_file).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "stat").unwrap();
    let value = v8::Function::new(scope, stat).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "access").unwrap();
    let value = v8::Function::new(scope, access).unwrap();
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

fn write_file(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _retval: v8::ReturnValue,
) {
    let arg_len = args.length();
    if arg_len < 2 {
        let msg = v8::String::new(scope, "path and data are required").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    }

    let path_arg = args.get(0);
    let isolate: &v8::Isolate = scope;
    let path_str = path_arg
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(isolate);

    let data_arg = args.get(1);
    let data = if data_arg.is_uint8_array() {
        let uint8_array = v8::Local::<v8::Uint8Array>::try_from(data_arg).unwrap();
        let mut buffer = vec![0u8; uint8_array.byte_length()];
        let copied = uint8_array.copy_contents(&mut buffer);
        if copied != buffer.len() {
            let msg = v8::String::new(scope, "Failed to copy buffer data").unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
            return;
        }
        buffer
    } else {
        let data_str = data_arg
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(isolate);
        data_str.into_bytes()
    };

    match fs::write(&path_str, &data) {
        Ok(_) => {}
        Err(e) => {
            let msg = v8::String::new(scope, &format!("Failed to write file: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn mkdir(scope: &mut v8::PinScope, args: v8::FunctionCallbackArguments, _retval: v8::ReturnValue) {
    let arg_len = args.length();
    if arg_len < 1 {
        let msg = v8::String::new(scope, "path is required").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    }

    let path_arg = args.get(0);
    let isolate: &v8::Isolate = scope;
    let path_str = path_arg
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(isolate);

    let recursive = if arg_len >= 2 {
        let options_arg = args.get(1);
        if options_arg.is_object() {
            let options = options_arg.to_object(scope).unwrap();
            let recursive_key = v8::String::new(scope, "recursive").unwrap();
            if let Some(recursive_val) = options.get(scope, recursive_key.into()) {
                recursive_val.is_true()
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    };

    let result = if recursive {
        fs::create_dir_all(&path_str)
    } else {
        fs::create_dir(&path_str)
    };

    match result {
        Ok(_) => {}
        Err(e) => {
            let msg =
                v8::String::new(scope, &format!("Failed to create directory: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn rmdir(scope: &mut v8::PinScope, args: v8::FunctionCallbackArguments, _retval: v8::ReturnValue) {
    let arg_len = args.length();
    if arg_len < 1 {
        let msg = v8::String::new(scope, "path is required").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    }

    let path_arg = args.get(0);
    let isolate: &v8::Isolate = scope;
    let path_str = path_arg
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(isolate);

    let recursive = if arg_len >= 2 {
        let options_arg = args.get(1);
        if options_arg.is_object() {
            let options = options_arg.to_object(scope).unwrap();
            let recursive_key = v8::String::new(scope, "recursive").unwrap();
            if let Some(recursive_val) = options.get(scope, recursive_key.into()) {
                recursive_val.is_true()
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    };

    let result = if recursive {
        fs::remove_dir_all(&path_str)
    } else {
        fs::remove_dir(&path_str)
    };

    match result {
        Ok(_) => {}
        Err(e) => {
            let msg =
                v8::String::new(scope, &format!("Failed to remove directory: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn unlink(scope: &mut v8::PinScope, args: v8::FunctionCallbackArguments, _retval: v8::ReturnValue) {
    let arg_len = args.length();
    if arg_len < 1 {
        let msg = v8::String::new(scope, "path is required").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    }

    let path_arg = args.get(0);
    let isolate: &v8::Isolate = scope;
    let path_str = path_arg
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(isolate);

    match fs::remove_file(&path_str) {
        Ok(_) => {}
        Err(e) => {
            let msg = v8::String::new(scope, &format!("Failed to remove file: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn rename(scope: &mut v8::PinScope, args: v8::FunctionCallbackArguments, _retval: v8::ReturnValue) {
    let arg_len = args.length();
    if arg_len < 2 {
        let msg = v8::String::new(scope, "oldPath and newPath are required").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    }

    let old_path_arg = args.get(0);
    let new_path_arg = args.get(1);
    let isolate: &v8::Isolate = scope;
    let old_path = old_path_arg
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(isolate);
    let new_path = new_path_arg
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(isolate);

    match fs::rename(&old_path, &new_path) {
        Ok(_) => {}
        Err(e) => {
            let msg = v8::String::new(scope, &format!("Failed to rename: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn copy_file(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _retval: v8::ReturnValue,
) {
    let arg_len = args.length();
    if arg_len < 2 {
        let msg = v8::String::new(scope, "src and dest are required").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    }

    let src_arg = args.get(0);
    let dest_arg = args.get(1);
    let isolate: &v8::Isolate = scope;
    let src_path = src_arg
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(isolate);
    let dest_path = dest_arg
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(isolate);

    match fs::copy(&src_path, &dest_path) {
        Ok(_) => {}
        Err(e) => {
            let msg = v8::String::new(scope, &format!("Failed to copy file: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn stat(
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

    let path_arg = args.get(0);
    let isolate: &v8::Isolate = scope;
    let path_str = path_arg
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(isolate);

    match fs::metadata(&path_str) {
        Ok(metadata) => {
            let stats = v8::Object::new(scope);

            let is_file = v8::Boolean::new(scope, metadata.is_file());
            let is_file_key = v8::String::new(scope, "isFile").unwrap();
            stats.set(scope, is_file_key.into(), is_file.into());

            let is_dir = v8::Boolean::new(scope, metadata.is_dir());
            let is_dir_key = v8::String::new(scope, "isDirectory").unwrap();
            stats.set(scope, is_dir_key.into(), is_dir.into());

            let is_symlink = v8::Boolean::new(scope, metadata.is_symlink());
            let is_symlink_key = v8::String::new(scope, "isSymbolicLink").unwrap();
            stats.set(scope, is_symlink_key.into(), is_symlink.into());

            let size = v8::Number::new(scope, metadata.len() as f64);
            let size_key = v8::String::new(scope, "size").unwrap();
            stats.set(scope, size_key.into(), size.into());

            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
                    let mtime_ms = v8::Number::new(scope, duration.as_millis() as f64);
                    let mtime_key = v8::String::new(scope, "mtimeMs").unwrap();
                    stats.set(scope, mtime_key.into(), mtime_ms.into());
                }
            }

            retval.set(stats.into());
        }
        Err(e) => {
            let msg = v8::String::new(scope, &format!("Failed to stat file: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn access(scope: &mut v8::PinScope, args: v8::FunctionCallbackArguments, _retval: v8::ReturnValue) {
    let arg_len = args.length();
    if arg_len < 1 {
        let msg = v8::String::new(scope, "path is required").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    }

    let path_arg = args.get(0);
    let isolate: &v8::Isolate = scope;
    let path_str = path_arg
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(isolate);

    match fs::metadata(&path_str) {
        Ok(_) => {}
        Err(e) => {
            let msg = v8::String::new(scope, &format!("Failed to access file: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}
