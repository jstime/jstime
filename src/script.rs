use std::process;
use std::fs;

use rusty_v8 as v8;

use crate::binding;
use crate::bootstrap;

pub fn run_js_in_scope(scope: &mut v8::HandleScope, js: &str) -> String {
  let code = v8::String::new(scope, js).unwrap();
  let script = v8::Script::compile(scope, code, None).unwrap();
  let result = script.run(scope).unwrap();
  let result = result.to_string(scope).unwrap();
  return result.to_rust_string_lossy(scope);
}

pub fn run(js: &str) -> String {
  let isolate = &mut v8::Isolate::new(Default::default());
  let scope = &mut v8::HandleScope::new(isolate);
  let context = binding::initialize_context(scope);
  let scope = &mut v8::ContextScope::new(scope, context);
  bootstrap::set_globals(scope);
  return run_js_in_scope(scope, js);
}

pub fn run_file(filepath: &str) {
  let stat = fs::metadata(filepath);
  match stat {
    Ok(_stat)=> {
      let contents = fs::read_to_string(filepath)
          .expect("Something went wrong reading the file");
      run(&contents);
    },
    Err(_e) => {
      eprintln!("Error: file doesn't exist");
      process::exit(1);
    }
  }
}
