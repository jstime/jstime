use dirs::home_dir;
use jstime_core as jstime;
use regex::Regex;
use rusty_v8 as v8;
use std::env;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

#[macro_use]
extern crate lazy_static;

#[derive(StructOpt)]
#[structopt(name = "jstime", rename_all = "kebab-case")]
struct Opt {
    /// File to read from, or "-" to read from stdin. Interactive mode if a tty
    #[structopt()]
    filename: Option<String>,

    /// Prints version information
    #[structopt(short, long)]
    version: bool,

    /// Options for V8
    #[structopt(long)]
    v8_options: Option<String>,
}

// A rust implementation of the complete function from node repl.
// https://github.com/nodejs/node/blob/c205f672e9cf0c70ea26f87eb97342947a244e18/lib/repl.js#L1136

struct JSTimeCompletions {
    builtins: Vec<String>,
    contextual_completions: Vec<String>,
}

impl JSTimeCompletions {
    fn complete(line: &str) {
        // lazy_static! {
        //     static ref SIMPLE_RE: Regex =
        //         Regex::new(r"/(?:[a-zA-Z_$](?:\w|\$)*\??\.)*[a-zA-Z_$](?:\w|\$)*\??\.?$/").unwrap();
        // }

        let SIMPLE_EXPRESSION_RE =
            Regex::new(r"(?m)(?:[a-zA-Z_$](?:\w|\$)*\??\.)*[a-zA-Z_$](?:\w|\$)*\??\.?$").unwrap();

        // TODO: handle regex capture panic
        let captures = SIMPLE_EXPRESSION_RE.captures(line).unwrap();
        let complete_on = &captures[0];
        let mut expr = String::from("");
        let mut filter = "";
        // if line ends_with '.'
        if line.ends_with(".") {
            // expr = complete_on.split()
        } else {
            let mut bits: Vec<&str> = complete_on.split(".").collect();
            filter = bits.pop().unwrap();
            expr = bits.join(".");
        }

        if (expr.is_empty()) {

        }
    }
}
fn get_global_builtins(mut runtime: jstime::JSTime) -> Vec<String> {
    // let context = jstime::IsolateState::get(self.isolate()).borrow().context();
    // let scope = &mut v8::HandleScope::with_context(self.isolate(), context);

    // let scope = v8::HandleScope::with_context()

    // TODO: This should also include builtins from the runtime, and not just global ones.

    let builtins = runtime.run_script("Object.getOwnPropertyNames(global)", "jstime-repl");

    vec!["okay".to_owned()]
}

fn main() {
    let opt = Opt::from_args();

    if opt.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    jstime::init(
        opt.v8_options
            .map(|o| o.split(' ').map(|s| s.to_owned()).collect()),
    );

    let mut options = jstime::Options::default();
    options.snapshot = Some(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/snapshot_data.blob"
    )));

    let mut jstime = jstime::JSTime::new(options);

    if let Some(filename) = opt.filename {
        std::process::exit(match jstime.import(&filename) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("{:?}", e);
                1
            }
        });
    } else {
        // home_dir() returns a PathBuf, which allows for mutating the path in place.
        // here $HOME/.jstime_repl_history is set as the path for jstime's history file.
        // PathBuf is similar to (but not the same as ) node's path lib

        let history_file = home_dir().map(|mut p| {
            p.push(".jstime_repl_history");
            p
        });

        Repl::new(jstime, history_file.unwrap());
    }
}

pub struct Repl {
    runtime: jstime::JSTime,
    history_file: PathBuf,
}

impl Repl {
    pub fn new(runtime: jstime::JSTime, history_file: PathBuf) -> Self {
        let repl = Self {
            runtime,
            history_file,
        };
        repl
    }

    pub fn readline(&mut self) {
        use rustyline::{error::ReadlineError, Editor};

        struct JstimeRustylineHelper {
            builtins: Vec<String>,
        }

        // Rustyline Helper requires a validator, hinter, highlighter and a completer
        // all of the implementations to the hinter trait can be empty
        impl rustyline::Helper for JstimeRustylineHelper {}
        impl rustyline::hint::Hinter for JstimeRustylineHelper {}
        impl rustyline::validate::Validator for JstimeRustylineHelper {}
        impl rustyline::highlight::Highlighter for JstimeRustylineHelper {}

        impl rustyline::completion::Completer for JstimeRustylineHelper {
            // Candidate is an assosciated type, that helps the readlineHelper know the type of completion.
            // Here the candidate refers to the "closest possible match" to a given string from all the completions / candidates available.
            type Candidate = String;

            fn complete(
                &self,
                line: &str,
                _pos: usize,
                _ctx: &rustyline::Context<'_>,
            ) -> std::result::Result<(usize, Vec<String>), rustyline::error::ReadlineError>
            {
                // Similar to the node repl
                // <tab>: When pressed on a blank line, displays global and local (scope) variables.
                // When pressed while entering other input, displays relevant autocompletion options.

                if line.is_empty() {
                    Ok((0, self.builtins))
                } else {
                    Ok((0, vec![]))
                }
            }

            fn update(
                &self,
                line: &mut rustyline::line_buffer::LineBuffer,
                start: usize,
                elected: &str,
            ) {
                let end = line.pos();
                line.replace(start..end, elected)
            }
        }

        // let mut builtins =
        let helper = JstimeRustylineHelper {
            builtins: vec!["".to_owned()],
        };

        let mut rl = Editor::<JstimeRustylineHelper>::new();
        rl.set_helper(Some(helper));
        rl.load_history(&self.history_file);

        println!("Welcome to jstime v{}!", env!("CARGO_PKG_VERSION"));

        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    match line.as_str() {
                        ".exit" => {
                            println!("Thanks for stopping by!");
                            break;
                        }
                        _ => match self.runtime.run_script(&line, ".repl") {
                            Ok(v) => println!("{}", v),
                            Err(e) => eprintln!("Uncaught: {}", e),
                        },
                    }
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

        // if let Some(&self.history_file) = self.history_file {
        //     let _ = rl.save_history(&self.history_file);
        // }
    }
}
