use rusty_v8 as v8;

use crate::script;

pub(crate) fn init() {
    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}

#[allow(dead_code)]
pub(crate) fn set_flags(args: Vec<String>) -> Vec<String> {
    v8::V8::set_flags_from_command_line(args)
}

pub(crate) fn set_globals(scope: &mut v8::HandleScope) {
    let console = include_str!("./console.js");
    script::run_js_in_scope(scope, console, "bootstrap");
}
