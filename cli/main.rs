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

    let options = jstime::Options::new(Some(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/snapshot_data.blob"
    ))));

    let mut jstime = jstime::JSTime::new(options);

    if let Some(filename) = opt.filename {
        std::process::exit(match jstime.import(&filename) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("{}", e);
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

    let mut rl = Editor::<()>::new().unwrap();
    println!("Welcome to jstime v{}!", env!("CARGO_PKG_VERSION"));

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
