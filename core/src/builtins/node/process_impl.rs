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
