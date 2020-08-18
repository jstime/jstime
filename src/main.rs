use std::env;
use std::fs;
use std::process;

use rusty_v8 as v8;

mod bootstrap;
mod binding;
mod repl;

fn run(isolate: &mut v8::Isolate, js: &str) -> String {
  let scope = &mut v8::HandleScope::new(isolate);
  let context = v8::Context::new(scope);
  let scope = &mut v8::ContextScope::new(scope, context);

  let code = v8::String::new(scope, js).unwrap();

  let script = v8::Script::compile(scope, code, None).unwrap();
  let result = script.run(scope).unwrap();
  let result = result.to_string(scope).unwrap();
  return result.to_rust_string_lossy(scope);
}

fn run_file(filepath: &str) {
  let stat = fs::metadata(filepath);
  match stat {
    Ok(_stat)=> {
      let isolate = &mut v8::Isolate::new(Default::default());
      let contents = fs::read_to_string(filepath)
          .expect("Something went wrong reading the file");

      let result = run(isolate, &contents);
      println!("{}", &result);
    },
    Err(_e) => {
      eprintln!("Error: file doesn't exist");
      process::exit(1);
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let len = args.len();

  if len > 2 {
    println!("Error: Too many arguments");
    process::exit(1);
  }
  
  bootstrap::init();
  
  match len {
    1 => repl::start(),
    2 => run_file(&args[1]),
    _ => println!("Woopsie Doodles")
  }
}
