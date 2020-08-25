// heavily inspired by deno bindings
// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.

// https://github.com/denoland/deno/blob/93e2bfe22e5cc782e7d502616dad1fd70d26ae37/core/bindings.rs#L323-L351

use rusty_v8 as v8;

pub(crate) fn initialize_context<'s>(
    scope: &mut v8::HandleScope<'s, ()>,
) -> v8::Local<'s, v8::Context> {
    let scope = &mut v8::EscapableHandleScope::new(scope);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);
    scope.escape(context)
}
