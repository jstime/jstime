use std::env;
use std::process;

mod bootstrap;
mod repl;
mod script;
mod binding;

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
    2 => script::run_file(&args[1]),
    _ => println!("Woopsie Doodles")
  }
}
