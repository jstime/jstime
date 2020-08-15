use std::env;
use std::fs;

use rusty_v8 as v8;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn run(js: &str) -> String {
  let platform = v8::new_default_platform().unwrap();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();

  let isolate = &mut v8::Isolate::new(Default::default());

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
  let contents = fs::read_to_string(filepath)
      .expect("Something went wrong reading the file");
  
  let result = run(&contents);
  println!("{}", &result);
}

fn repl() {
  let mut rl = Editor::<()>::new();
  loop {
    let readline = rl.readline(">> ");
    match readline {
      Ok(line) => {
        let result = run(&line);
        println!("{}", &result);
      },
      Err(ReadlineError::Interrupted) => {
        println!("Thanks for stopping by!");
        break
      },
      Err(ReadlineError::Eof) => {
        println!("Eof'd");
        break
      },
      Err(err) => {
        println!("Error: {:?}", err);
        break
      }
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let len = args.len();
  
  match len {
    1 => repl(),
    2 => run_file(&args[1]),
    _ => println!("Woopsie Doodles")
  }
}
