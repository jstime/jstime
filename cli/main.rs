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

    let options = jstime::Options::new(None);
    // let options = jstime::Options::new(Some(include_bytes!(concat!(
    //     env!("OUT_DIR"),
    //     "/snapshot_data.blob"
    // ))));

    let mut jstime = jstime::JSTime::new(options);

    if let Some(filename) = opt.filename {
        std::process::exit(match jstime.import(&filename) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("{e}");
                1
            }
        });
    } else {
        repl(jstime);
    }
}

fn repl(mut jstime: jstime::JSTime) {
    use dirs::home_dir;
    use rustyline::{error::ReadlineError, DefaultEditor};
    use std::sync::mpsc::{channel, RecvTimeoutError};
    use std::thread;
    use std::time::Duration;

    let mut rl = DefaultEditor::new().unwrap();
    println!("Welcome to jstime v{}!", env!("CARGO_PKG_VERSION"));

    let history_path = home_dir().map(|mut p| {
        p.push(".jstime_repl_history");
        let _ = rl.load_history(&p);
        p
    });

    loop {
        // Channel for this readline
        let (tx, rx) = channel();

        // Start readline in a separate thread
        thread::spawn(move || {
            let mut rl_temp = DefaultEditor::new().unwrap();
            let result = rl_temp.readline(">> ");
            let _ = tx.send(result);
        });

        // Poll for readline completion while ticking event loop
        let readline_result = loop {
            match rx.recv_timeout(Duration::from_millis(10)) {
                Ok(result) => break result,
                Err(RecvTimeoutError::Timeout) => {
                    // Tick the event loop while waiting for input
                    jstime.tick_event_loop();
                }
                Err(RecvTimeoutError::Disconnected) => {
                    return; // Thread died unexpectedly
                }
            }
        };

        match readline_result {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                match jstime.run_script_no_event_loop(&line, "REPL") {
                    Ok(v) => println!("{v}"),
                    Err(e) => eprintln!("Uncaught: {e}"),
                }
                jstime.tick_event_loop();
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
                eprintln!("Error: {err:?}");
                break;
            }
        }
    }

    if let Some(history_path) = history_path {
        let _ = rl.save_history(&history_path);
    }
}
