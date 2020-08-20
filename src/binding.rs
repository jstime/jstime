// heavily inspired by deno bindings
// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.

// https://github.com/denoland/deno/blob/93e2bfe22e5cc782e7d502616dad1fd70d26ae37/core/bindings.rs#L323-L351

use rusty_v8 as v8;

pub fn initialize_context<'s>(scope: &mut v8::HandleScope<'s, ()>) -> v8::Local<'s, v8::Context> {
    let scope = &mut v8::EscapableHandleScope::new(scope);

    let context = v8::Context::new(scope);
    let global = context.global(scope);

    let scope = &mut v8::ContextScope::new(scope, context);

    let print_key = v8::String::new(scope, "printer").unwrap();
    let print_tmpl = v8::FunctionTemplate::new(scope, printer);
    let print_val = print_tmpl.get_function(scope).unwrap();
    global.set(scope, print_key.into(), print_val.into());

    scope.escape(context)
}

fn printer(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue) {
    let arg_len = args.length();
    assert!(arg_len >= 0 && arg_len <= 2);

    let obj = args.get(0);
    let is_err_arg = args.get(1);

    let mut is_err = false;
    if arg_len == 2 {
        let int_val = is_err_arg
            .integer_value(scope)
            .expect("Unable to convert to integer");
        is_err = int_val != 0;
    };
    let tc_scope = &mut v8::TryCatch::new(scope);
    let str_ = match obj.to_string(tc_scope) {
        Some(s) => s,
        None => v8::String::new(tc_scope, "").unwrap(),
    };
    if is_err {
        eprintln!("{}", str_.to_rust_string_lossy(tc_scope));
    } else {
        println!("{}", str_.to_rust_string_lossy(tc_scope));
    }
}
