use rusty_v8 as v8;

use crate::script;

pub fn init () {
  let platform = v8::new_default_platform().unwrap();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();
}

pub fn set_globals<'s>(
  scope: &mut v8::HandleScope,
) {
  let console = include_str!("./console.js");
  script::run_js_in_scope(scope, console);
}
