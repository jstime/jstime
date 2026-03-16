use jstime_core as jstime;
use repl_autocomplete::{CompletionCache, JsCompleter, extract_repl_binding_names, refresh_cache};
use rustc_hash::FxHashSet;
use rustyline::{Editor, error::ReadlineError, history::DefaultHistory};
use std::env;
use std::process;
use std::sync::{Arc, Mutex, RwLock};
use structopt::StructOpt;
use structopt::clap;

mod repl_autocomplete;

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

    /// Number of warmup iterations to run before actual execution.
    /// This allows V8's TurboFan JIT compiler to optimize the code.
    /// Useful for benchmarking or performance-critical scripts.
    #[structopt(long, default_value = "0")]
    warmup: usize,
}

fn main() {
    // Parse arguments manually to support trailing script arguments
    let all_args: Vec<String> = env::args().collect();

    // Split at filename (first non-flag argument)
    let mut structopt_args = vec![all_args[0].clone()];
    let mut script_args = Vec::with_capacity(4); // Pre-allocate for typical script args
    let mut found_filename = false;
    let mut expect_value = false; // Track if we expect a value for an option

    for (_i, arg) in all_args.iter().enumerate().skip(1) {
        if expect_value {
            // This is a value for the previous option
            structopt_args.push(arg.clone());
            expect_value = false;
        } else if arg.starts_with("--") {
            // This is a long flag
            structopt_args.push(arg.clone());
            // Check if this flag expects a value (doesn't use = syntax)
            let is_option_with_value = (arg.starts_with("--warmup")
                || arg.starts_with("--v8-options"))
                && !arg.contains('=');
            if is_option_with_value {
                expect_value = true;
            }
        } else if arg.starts_with('-') && !found_filename {
            // This is a short flag
            structopt_args.push(arg.clone());
        } else if !found_filename {
            // This is the filename
            structopt_args.push(arg.clone());
            found_filename = true;
        } else {
            // These are script arguments
            script_args.push(arg.clone());
        }
    }

    let opt = Opt::from_iter_safe(&structopt_args);
    let opt = match opt {
        Ok(o) => o,
        Err(e) => {
            // For help and version, print to stdout and exit with success
            if e.kind == clap::ErrorKind::HelpDisplayed
                || e.kind == clap::ErrorKind::VersionDisplayed
            {
                println!("{}", e);
                process::exit(0);
            }
            // For other errors, print to stderr and exit with failure
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if opt.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    jstime::init(
        opt.v8_options
            .map(|o| o.split(' ').map(|s| s.to_owned()).collect()),
    );

    // Build process.argv - executable path and script arguments
    // Pre-allocate vector with estimated size (1 for executable + 1 for filename if present + script args)
    let initial_capacity = 1 + if opt.filename.is_some() { 1 } else { 0 } + script_args.len();
    let mut process_argv = Vec::with_capacity(initial_capacity);

    // First argument is always the executable
    process_argv.push(all_args[0].clone());

    // Add the filename if provided
    if let Some(ref filename) = opt.filename {
        process_argv.push(filename.clone());
    }

    // Add any additional script arguments
    process_argv.extend(script_args);

    let options = jstime::Options::new(Some(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/snapshot_data.blob"
    ))))
    .with_process_argv(process_argv)
    .with_warmup(opt.warmup);

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
    use std::sync::mpsc::{RecvTimeoutError, channel};
    use std::thread;
    use std::time::Duration;

    // Initialize the completion cache
    let cache = Arc::new(RwLock::new(CompletionCache::new()));
    let mut repl_bindings = FxHashSet::default();
    refresh_cache(&mut jstime, &cache, &repl_bindings);

    let completer = JsCompleter::new(Arc::clone(&cache));
    let mut rl = Editor::<JsCompleter, DefaultHistory>::with_config(
        rustyline::Config::builder()
            .completion_type(rustyline::CompletionType::List)
            .build(),
    )
    .unwrap();
    rl.set_helper(Some(completer));
    println!("Welcome to jstime v{}!", env!("CARGO_PKG_VERSION"));

    let history_path = home_dir().map(|mut p| {
        p.push(".jstime_repl_history");
        let _ = rl.load_history(&p);
        p
    });

    // Wrap the editor in Arc<Mutex> to share it with the readline thread
    let rl_shared = Arc::new(Mutex::new(rl));

    // Track the last interrupt time for double Ctrl+C exit
    let mut last_interrupt_time: Option<std::time::Instant> = None;

    loop {
        // Channel for this readline
        let (tx, rx) = channel();
        let rl_clone = Arc::clone(&rl_shared);

        // Start readline in a separate thread
        thread::spawn(move || {
            let mut rl_guard = rl_clone.lock().unwrap();
            let result = rl_guard.readline(">> ");
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
                // Reset interrupt tracking on successful input
                last_interrupt_time = None;

                // Add to history
                let mut rl_guard = rl_shared.lock().unwrap();
                let _ = rl_guard.add_history_entry(line.as_str());
                drop(rl_guard);

                match jstime.run_script_no_event_loop(&line, "REPL") {
                    Ok(v) => {
                        println!("{v}");
                        repl_bindings.extend(extract_repl_binding_names(&line));
                    }
                    Err(e) => eprintln!("Uncaught: {e}"),
                }
                jstime.tick_event_loop();

                // Refresh completion cache to pick up any new global variables and REPL bindings
                refresh_cache(&mut jstime, &cache, &repl_bindings);
            }
            Err(ReadlineError::Interrupted) => {
                let now = std::time::Instant::now();

                // Check if this is a consecutive Ctrl+C within 1 second
                if let Some(last_time) = last_interrupt_time
                    && now.duration_since(last_time).as_millis() < 1000
                {
                    println!("Thanks for stopping by!");
                    break;
                }

                // First Ctrl+C or too much time has passed
                println!("(To exit, press Ctrl+C again)");
                last_interrupt_time = Some(now);
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
        let mut rl_guard = rl_shared.lock().unwrap();
        let _ = rl_guard.save_history(&history_path);
    }
}
