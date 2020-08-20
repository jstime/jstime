use rusty_v8 as v8;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::binding;
use crate::bootstrap;
use crate::script;

pub fn start() {
    let isolate = &mut v8::Isolate::new(Default::default());
    let scope = &mut v8::HandleScope::new(isolate);
    let context = binding::initialize_context(scope);
    let scope = &mut v8::ContextScope::new(scope, context);
    bootstrap::set_globals(scope);

    let mut rl = Editor::<()>::new();

    println!("\nWelcome to jstime!\n");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let result = script::run_js_in_scope(scope, &line);
                println!("{}", &result);
            }
            Err(ReadlineError::Interrupted) => {
                println!("Thanks for stopping by!");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Eof'd");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}
