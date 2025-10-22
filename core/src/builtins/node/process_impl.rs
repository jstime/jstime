pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(get_env),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(get_argv),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(get_cwd),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(exit),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(write_stdout),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(write_stderr),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(read_stdin),
        },
    ]
}

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    let name = v8::String::new(scope, "getEnv").unwrap();
    let value = v8::Function::new(scope, get_env).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "getArgv").unwrap();
    let value = v8::Function::new(scope, get_argv).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "getCwd").unwrap();
    let value = v8::Function::new(scope, get_cwd).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "exit").unwrap();
    let value = v8::Function::new(scope, exit).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "writeStdout").unwrap();
    let value = v8::Function::new(scope, write_stdout).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "writeStderr").unwrap();
    let value = v8::Function::new(scope, write_stderr).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "readStdin").unwrap();
    let value = v8::Function::new(scope, read_stdin).unwrap();
    bindings.set(scope, name.into(), value.into());
}

fn get_env(
    scope: &mut v8::PinScope,
    _args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    let env_obj = v8::Object::new(scope);

    // Get all environment variables
    for (key, value) in std::env::vars() {
        let Some(k) = v8::String::new(scope, &key) else {
            continue;
        };
        let Some(v) = v8::String::new(scope, &value) else {
            continue;
        };
        env_obj.set(scope, k.into(), v.into());
    }

    retval.set(env_obj.into());
}

fn get_argv(
    scope: &mut v8::PinScope,
    _args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    let state = crate::IsolateState::get(scope);
    let state_ref = state.borrow();

    let argv = &state_ref.process_argv;
    let array = v8::Array::new(scope, argv.len() as i32);

    for (i, arg) in argv.iter().enumerate() {
        let Some(arg_str) = v8::String::new(scope, arg) else {
            continue;
        };
        array.set_index(scope, i as u32, arg_str.into());
    }

    retval.set(array.into());
}

fn get_cwd(
    scope: &mut v8::PinScope,
    _args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    match std::env::current_dir() {
        Ok(path) => {
            let path_str = path.to_string_lossy();
            let Some(result) = v8::String::new(scope, &path_str) else {
                crate::error::throw_error(scope, "Failed to create string");
                return;
            };
            retval.set(result.into());
        }
        Err(e) => {
            crate::error::throw_error(scope, &format!("Failed to get current directory: {}", e));
        }
    }
}

fn exit(scope: &mut v8::PinScope, args: v8::FunctionCallbackArguments, _retval: v8::ReturnValue) {
    let code = if args.length() > 0 {
        let code_arg = args.get(0);
        if code_arg.is_number() {
            code_arg.number_value(scope).unwrap_or(0.0) as i32
        } else {
            0
        }
    } else {
        0
    };

    std::process::exit(code);
}

fn write_stdout(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if args.length() < 1 {
        crate::error::throw_error(scope, "writeStdout requires 1 argument");
        return;
    }

    let chunk = args.get(0);

    let output = if chunk.is_string() {
        // Handle string input
        v8::tc_scope!(let tc, scope);
        let str_val = chunk.to_string(tc).unwrap();
        str_val.to_rust_string_lossy(tc).into_bytes()
    } else if chunk.is_uint8_array() {
        // Handle Uint8Array input
        let Some(uint8_array) = v8::Local::<v8::Uint8Array>::try_from(chunk).ok() else {
            crate::error::throw_error(scope, "Failed to convert to Uint8Array");
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
        // For other types, convert to string
        v8::tc_scope!(let tc, scope);
        let str_val = chunk.to_string(tc).unwrap();
        str_val.to_rust_string_lossy(tc).into_bytes()
    };

    use std::io::Write;
    if let Err(e) = std::io::stdout().write_all(&output) {
        crate::error::throw_error(scope, &format!("Failed to write to stdout: {}", e));
        return;
    }

    // Flush to ensure immediate output
    if let Err(e) = std::io::stdout().flush() {
        crate::error::throw_error(scope, &format!("Failed to flush stdout: {}", e));
        return;
    }

    retval.set(v8::undefined(scope).into());
}

fn write_stderr(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if args.length() < 1 {
        crate::error::throw_error(scope, "writeStderr requires 1 argument");
        return;
    }

    let chunk = args.get(0);

    let output = if chunk.is_string() {
        // Handle string input
        v8::tc_scope!(let tc, scope);
        let str_val = chunk.to_string(tc).unwrap();
        str_val.to_rust_string_lossy(tc).into_bytes()
    } else if chunk.is_uint8_array() {
        // Handle Uint8Array input
        let Some(uint8_array) = v8::Local::<v8::Uint8Array>::try_from(chunk).ok() else {
            crate::error::throw_error(scope, "Failed to convert to Uint8Array");
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
        // For other types, convert to string
        v8::tc_scope!(let tc, scope);
        let str_val = chunk.to_string(tc).unwrap();
        str_val.to_rust_string_lossy(tc).into_bytes()
    };

    use std::io::Write;
    if let Err(e) = std::io::stderr().write_all(&output) {
        crate::error::throw_error(scope, &format!("Failed to write to stderr: {}", e));
        return;
    }

    // Flush to ensure immediate output
    if let Err(e) = std::io::stderr().flush() {
        crate::error::throw_error(scope, &format!("Failed to flush stderr: {}", e));
        return;
    }

    retval.set(v8::undefined(scope).into());
}

fn read_stdin(
    scope: &mut v8::PinScope,
    _args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    use std::io::Read;

    let mut buffer = String::new();
    match std::io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {
            let Some(result) = v8::String::new(scope, &buffer) else {
                crate::error::throw_error(scope, "Failed to create string from stdin");
                return;
            };
            retval.set(result.into());
        }
        Err(e) => {
            crate::error::throw_error(scope, &format!("Failed to read from stdin: {}", e));
        }
    }
}
