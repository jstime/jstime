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
            function: v8::MapFnTo::map_fn_to(append_file),
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
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(rm),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(truncate),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(realpath),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(chmod),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(mkdtemp),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(readlink),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(symlink),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(lstat),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(chown),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(utimes),
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

    let name = v8::String::new(scope, "appendFile").unwrap();
    let value = v8::Function::new(scope, append_file).unwrap();
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

    let name = v8::String::new(scope, "rm").unwrap();
    let value = v8::Function::new(scope, rm).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "truncate").unwrap();
    let value = v8::Function::new(scope, truncate).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "realpath").unwrap();
    let value = v8::Function::new(scope, realpath).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "chmod").unwrap();
    let value = v8::Function::new(scope, chmod).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "mkdtemp").unwrap();
    let value = v8::Function::new(scope, mkdtemp).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "readlink").unwrap();
    let value = v8::Function::new(scope, readlink).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "symlink").unwrap();
    let value = v8::Function::new(scope, symlink).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "lstat").unwrap();
    let value = v8::Function::new(scope, lstat).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "chown").unwrap();
    let value = v8::Function::new(scope, chown).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "utimes").unwrap();
    let value = v8::Function::new(scope, utimes).unwrap();
    bindings.set(scope, name.into(), value.into());
}

fn read_file(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 1, "readFile") {
        return;
    }

    // Get the file path
    let path_arg = args.get(0);
    let Some(path_str) = crate::error::to_rust_string_or_throw(scope, path_arg, "path") else {
        return;
    };

    // Get the encoding if provided (second argument)
    let arg_len = args.length();
    let encoding = if arg_len >= 2 {
        let encoding_arg = args.get(1);
        if !encoding_arg.is_null_or_undefined() {
            crate::error::to_rust_string_or_throw(scope, encoding_arg, "encoding")
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
                            crate::error::throw_error(scope, &format!("Invalid UTF-8: {}", e));
                        }
                    }
                } else {
                    crate::error::throw_error(scope, &format!("Unsupported encoding: {}", enc));
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
            crate::error::throw_error(scope, &format!("Failed to read file: {}", e));
        }
    }
}

fn read_dir(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 1, "readDir") {
        return;
    }

    // Get the directory path
    let path_arg = args.get(0);
    let Some(path_str) = crate::error::to_rust_string_or_throw(scope, path_arg, "path") else {
        return;
    };

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
                        crate::error::throw_error(scope, &format!("Failed to read entry: {}", e));
                        return;
                    }
                }
            }

            retval.set(array.into());
        }
        Err(e) => {
            crate::error::throw_error(scope, &format!("Failed to read directory: {}", e));
        }
    }
}

fn write_file(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 2, "writeFile") {
        return;
    }

    let path_arg = args.get(0);
    let Some(path_str) = crate::error::to_rust_string_or_throw(scope, path_arg, "path") else {
        return;
    };

    let data_arg = args.get(1);
    let data = if data_arg.is_uint8_array() {
        let Some(uint8_array) = v8::Local::<v8::Uint8Array>::try_from(data_arg).ok() else {
            crate::error::throw_type_error(scope, "Failed to convert to Uint8Array");
            return;
        };
        let mut buffer = vec![0u8; uint8_array.byte_length()];
        let copied = uint8_array.copy_contents(&mut buffer);
        if copied != buffer.len() {
            crate::error::throw_error(scope, "Failed to copy buffer data");
            return;
        }
        buffer
    } else {
        let Some(data_str) = crate::error::to_rust_string_or_throw(scope, data_arg, "data") else {
            return;
        };
        data_str.into_bytes()
    };

    match fs::write(&path_str, &data) {
        Ok(_) => {}
        Err(e) => {
            crate::error::throw_error(scope, &format!("Failed to write file: {}", e));
        }
    }
}

fn append_file(
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

    // Use OpenOptions to append to file
    use std::io::Write;
    match fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path_str)
    {
        Ok(mut file) => match file.write_all(&data) {
            Ok(_) => {}
            Err(e) => {
                let msg =
                    v8::String::new(scope, &format!("Failed to append to file: {}", e)).unwrap();
                let exception = v8::Exception::error(scope, msg);
                scope.throw_exception(exception);
            }
        },
        Err(e) => {
            let msg = v8::String::new(scope, &format!("Failed to open file: {}", e)).unwrap();
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

            if let Ok(modified) = metadata.modified()
                && let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH)
            {
                let mtime_ms = v8::Number::new(scope, duration.as_millis() as f64);
                let mtime_key = v8::String::new(scope, "mtimeMs").unwrap();
                stats.set(scope, mtime_key.into(), mtime_ms.into());
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

fn rm(scope: &mut v8::PinScope, args: v8::FunctionCallbackArguments, _retval: v8::ReturnValue) {
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

    // Check if path is a file or directory
    match fs::metadata(&path_str) {
        Ok(metadata) => {
            if metadata.is_file() {
                match fs::remove_file(&path_str) {
                    Ok(_) => {}
                    Err(e) => {
                        let msg = v8::String::new(scope, &format!("Failed to remove file: {}", e))
                            .unwrap();
                        let exception = v8::Exception::error(scope, msg);
                        scope.throw_exception(exception);
                    }
                }
            } else if metadata.is_dir() {
                let result = if recursive {
                    fs::remove_dir_all(&path_str)
                } else {
                    fs::remove_dir(&path_str)
                };
                match result {
                    Ok(_) => {}
                    Err(e) => {
                        let msg =
                            v8::String::new(scope, &format!("Failed to remove directory: {}", e))
                                .unwrap();
                        let exception = v8::Exception::error(scope, msg);
                        scope.throw_exception(exception);
                    }
                }
            }
        }
        Err(e) => {
            let msg = v8::String::new(scope, &format!("Failed to remove: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn truncate(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _retval: v8::ReturnValue,
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

    let len = if arg_len >= 2 {
        let len_arg = args.get(1);
        if len_arg.is_number() {
            len_arg.number_value(scope).unwrap() as u64
        } else {
            0
        }
    } else {
        0
    };

    use std::fs::OpenOptions;
    match OpenOptions::new().write(true).open(&path_str) {
        Ok(file) => match file.set_len(len) {
            Ok(_) => {}
            Err(e) => {
                let msg =
                    v8::String::new(scope, &format!("Failed to truncate file: {}", e)).unwrap();
                let exception = v8::Exception::error(scope, msg);
                scope.throw_exception(exception);
            }
        },
        Err(e) => {
            let msg = v8::String::new(scope, &format!("Failed to open file: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn realpath(
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

    match fs::canonicalize(&path_str) {
        Ok(absolute_path) => {
            let path_string = absolute_path.to_string_lossy();
            let result = v8::String::new(scope, &path_string).unwrap();
            retval.set(result.into());
        }
        Err(e) => {
            let msg = v8::String::new(scope, &format!("Failed to resolve path: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn chmod(scope: &mut v8::PinScope, args: v8::FunctionCallbackArguments, _retval: v8::ReturnValue) {
    let arg_len = args.length();
    if arg_len < 2 {
        let msg = v8::String::new(scope, "path and mode are required").unwrap();
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

    let mode_arg = args.get(1);
    let mode = if mode_arg.is_number() {
        mode_arg.number_value(scope).unwrap() as u32
    } else {
        let msg = v8::String::new(scope, "mode must be a number").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    };

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        match fs::metadata(&path_str) {
            Ok(metadata) => {
                let mut permissions = metadata.permissions();
                permissions.set_mode(mode);
                match fs::set_permissions(&path_str, permissions) {
                    Ok(_) => {}
                    Err(e) => {
                        let msg =
                            v8::String::new(scope, &format!("Failed to change permissions: {}", e))
                                .unwrap();
                        let exception = v8::Exception::error(scope, msg);
                        scope.throw_exception(exception);
                    }
                }
            }
            Err(e) => {
                let msg =
                    v8::String::new(scope, &format!("Failed to get metadata: {}", e)).unwrap();
                let exception = v8::Exception::error(scope, msg);
                scope.throw_exception(exception);
            }
        }
    }

    #[cfg(not(unix))]
    {
        let msg = v8::String::new(scope, "chmod is not supported on this platform").unwrap();
        let exception = v8::Exception::error(scope, msg);
        scope.throw_exception(exception);
    }
}

fn mkdtemp(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    let arg_len = args.length();
    if arg_len < 1 {
        let msg = v8::String::new(scope, "prefix is required").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    }

    let prefix_arg = args.get(0);
    let isolate: &v8::Isolate = scope;
    let prefix = prefix_arg
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(isolate);

    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let random_suffix = format!("{:x}", timestamp);
    let dir_path = format!("{}{}", prefix, &random_suffix[random_suffix.len() - 6..]);

    match fs::create_dir(&dir_path) {
        Ok(_) => {
            let result = v8::String::new(scope, &dir_path).unwrap();
            retval.set(result.into());
        }
        Err(e) => {
            let msg =
                v8::String::new(scope, &format!("Failed to create temp directory: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn readlink(
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

    match fs::read_link(&path_str) {
        Ok(target) => {
            let target_str = target.to_string_lossy();
            let result = v8::String::new(scope, &target_str).unwrap();
            retval.set(result.into());
        }
        Err(e) => {
            let msg = v8::String::new(scope, &format!("Failed to read link: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn symlink(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _retval: v8::ReturnValue,
) {
    let arg_len = args.length();
    if arg_len < 2 {
        let msg = v8::String::new(scope, "target and path are required").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    }

    let target_arg = args.get(0);
    let path_arg = args.get(1);
    let isolate: &v8::Isolate = scope;
    let target = target_arg
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(isolate);
    let path = path_arg
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(isolate);

    #[cfg(unix)]
    {
        match std::os::unix::fs::symlink(&target, &path) {
            Ok(_) => {}
            Err(e) => {
                let msg =
                    v8::String::new(scope, &format!("Failed to create symlink: {}", e)).unwrap();
                let exception = v8::Exception::error(scope, msg);
                scope.throw_exception(exception);
            }
        }
    }

    #[cfg(windows)]
    {
        // On Windows, we need to determine if target is a file or directory
        let is_dir = fs::metadata(&target).map(|m| m.is_dir()).unwrap_or(false);
        let result = if is_dir {
            std::os::windows::fs::symlink_dir(&target, &path)
        } else {
            std::os::windows::fs::symlink_file(&target, &path)
        };

        match result {
            Ok(_) => {}
            Err(e) => {
                let msg =
                    v8::String::new(scope, &format!("Failed to create symlink: {}", e)).unwrap();
                let exception = v8::Exception::error(scope, msg);
                scope.throw_exception(exception);
            }
        }
    }

    #[cfg(not(any(unix, windows)))]
    {
        let msg = v8::String::new(scope, "symlink is not supported on this platform").unwrap();
        let exception = v8::Exception::error(scope, msg);
        scope.throw_exception(exception);
    }
}

fn lstat(
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

    match fs::symlink_metadata(&path_str) {
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

            if let Ok(modified) = metadata.modified()
                && let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH)
            {
                let mtime_ms = v8::Number::new(scope, duration.as_millis() as f64);
                let mtime_key = v8::String::new(scope, "mtimeMs").unwrap();
                stats.set(scope, mtime_key.into(), mtime_ms.into());
            }

            retval.set(stats.into());
        }
        Err(e) => {
            let msg = v8::String::new(scope, &format!("Failed to lstat file: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn chown(scope: &mut v8::PinScope, args: v8::FunctionCallbackArguments, _retval: v8::ReturnValue) {
    let arg_len = args.length();
    if arg_len < 3 {
        let msg = v8::String::new(scope, "path, uid, and gid are required").unwrap();
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

    let uid_arg = args.get(1);
    let gid_arg = args.get(2);

    let uid = if uid_arg.is_number() {
        uid_arg.number_value(scope).unwrap() as u32
    } else {
        let msg = v8::String::new(scope, "uid must be a number").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    };

    let gid = if gid_arg.is_number() {
        gid_arg.number_value(scope).unwrap() as u32
    } else {
        let msg = v8::String::new(scope, "gid must be a number").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    };

    #[cfg(unix)]
    {
        use std::os::unix::fs::chown as unix_chown;
        match unix_chown(&path_str, Some(uid), Some(gid)) {
            Ok(_) => {}
            Err(e) => {
                let msg =
                    v8::String::new(scope, &format!("Failed to change ownership: {}", e)).unwrap();
                let exception = v8::Exception::error(scope, msg);
                scope.throw_exception(exception);
            }
        }
    }

    #[cfg(not(unix))]
    {
        let msg = v8::String::new(scope, "chown is not supported on this platform").unwrap();
        let exception = v8::Exception::error(scope, msg);
        scope.throw_exception(exception);
    }
}

fn utimes(scope: &mut v8::PinScope, args: v8::FunctionCallbackArguments, _retval: v8::ReturnValue) {
    let arg_len = args.length();
    if arg_len < 3 {
        let msg = v8::String::new(scope, "path, atime, and mtime are required").unwrap();
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

    let atime_arg = args.get(1);
    let mtime_arg = args.get(2);

    let atime_ms = if atime_arg.is_number() || atime_arg.is_date() {
        atime_arg.number_value(scope).unwrap()
    } else {
        let msg = v8::String::new(scope, "atime must be a number or Date").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    };

    let mtime_ms = if mtime_arg.is_number() || mtime_arg.is_date() {
        mtime_arg.number_value(scope).unwrap()
    } else {
        let msg = v8::String::new(scope, "mtime must be a number or Date").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    };

    use std::time::{Duration, UNIX_EPOCH};
    let atime = UNIX_EPOCH + Duration::from_millis(atime_ms as u64);
    let mtime = UNIX_EPOCH + Duration::from_millis(mtime_ms as u64);

    match filetime::set_file_times(&path_str, atime.into(), mtime.into()) {
        Ok(_) => {}
        Err(e) => {
            let msg = v8::String::new(scope, &format!("Failed to set file times: {}", e)).unwrap();
            let exception = v8::Exception::error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}
