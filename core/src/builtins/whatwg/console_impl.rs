pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![v8::ExternalReference {
        function: v8::MapFnTo::map_fn_to(printer),
    }]
}

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    let name = v8::String::new(scope, "printer").unwrap();
    let value = v8::Function::new(scope, printer).unwrap();
    bindings.set(scope, name.into(), value.into());
}

fn printer(scope: &mut v8::PinScope, args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue) {
    let arg_len = args.length();
    assert!((0..=2).contains(&arg_len));

    let obj = args.get(0);
    let is_err_arg = args.get(1);

    let mut is_err = false;
    if arg_len == 2 {
        let int_val = is_err_arg
            .integer_value(scope)
            .expect("Unable to convert to integer");
        is_err = int_val != 0;
    };
    v8::tc_scope!(let tc, scope);
    let str_ = match obj.to_string(tc) {
        Some(s) => s,
        None => v8::String::new(tc, "").unwrap(),
    };
    if is_err {
        eprintln!("{}", str_.to_rust_string_lossy(tc));
    } else {
        println!("{}", str_.to_rust_string_lossy(tc));
    }
}
