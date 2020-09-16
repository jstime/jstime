use jstime_core as jstime;
use std::env;
use std::process;
use structopt::StructOpt;

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

struct JSTimeCompletions {
    global_builtins: Vec<String>,
}

// TODO: implement get_global_builtins
fn get_global_builtins() -> Vec<String> {
    vec!["Array".to_owned()]
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
        repl(jstime);
    }
}

fn repl(mut jstime: jstime::JSTime) {
    use dirs::home_dir;
    use rustyline::{error::ReadlineError, Editor};

    struct ReplReadlineHelper {}

    // rustyline Helper, requires a validator, hinter, highlighter and a completer
    // all of the implementations to the hinter trait can be empty
    impl rustyline::Helper for ReplReadlineHelper {}
    impl rustyline::hint::Hinter for ReplReadlineHelper {}
    impl rustyline::validate::Validator for ReplReadlineHelper {}
    impl rustyline::highlight::Highlighter for ReplReadlineHelper {}

    impl rustyline::completion::Completer for ReplReadlineHelper {
        // Candidate is an assosciated type, that helps the readlineHelper know the type of completion.
        // Here the candidate refers to the "closest possible match" to a given string from all the completions / candidates available.
        type Candidate = String;

        fn complete(
            &self,
            _line: &str,
            _pos: usize,
            _ctx: &rustyline::Context<'_>,
        ) -> std::result::Result<(usize, Vec<String>), rustyline::error::ReadlineError> {
            let completions = JSTimeCompletions {
                global_builtins: get_global_builtins(),
            };

            Ok((0, completions.global_builtins))
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

    let mut rl = Editor::<ReplReadlineHelper>::new();

    println!("Welcome to jstime v{}!", env!("CARGO_PKG_VERSION"));

    // home_dir() returns a PathBuf, which allows for mutating the path in place.
    // here $HOME/.jstime_repl_history is set as the path for jstime's history file.
    // PathBuf is similar to (but not the same as ) node's path lib

    let history_path = home_dir().map(|mut p| {
        p.push(".jstime_repl_history");
        let _ = rl.load_history(&p);
        p
    });

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match jstime.run_script(&line, "REPL") {
                    Ok(v) => println!("{}", v),
                    Err(e) => eprintln!("Uncaught: {}", e),
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

    if let Some(history_path) = history_path {
        let _ = rl.save_history(&history_path);
    }
}
