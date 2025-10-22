use jstime_core as jstime;
use std::env;
use std::process;
use structopt::StructOpt;
use structopt::clap;

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
    // Parse arguments manually to support trailing script arguments
    let all_args: Vec<String> = env::args().collect();

    // Split at filename (first non-flag argument)
    let mut structopt_args = vec![all_args[0].clone()];
    let mut script_args = Vec::with_capacity(4); // Pre-allocate for typical script args
    let mut found_filename = false;

    for (_i, arg) in all_args.iter().enumerate().skip(1) {
        if !found_filename && !arg.starts_with("--") && !arg.starts_with('-') {
            // This is the filename
            structopt_args.push(arg.clone());
            found_filename = true;
        } else if found_filename {
            // These are script arguments
            script_args.push(arg.clone());
        } else {
            // These are jstime options
            structopt_args.push(arg.clone());
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

    let mut options = jstime::Options::new(None);
    options.process_argv = process_argv;
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
    use rustyline::Helper;
    use rustyline::highlight::Highlighter;
    use rustyline::hint::Hinter;
    use rustyline::validate::{ValidationContext, ValidationResult, Validator};
    use rustyline::{
        Context, Editor,
        completion::{Completer, Pair},
        error::ReadlineError,
        history::DefaultHistory,
    };
    use std::sync::mpsc::{RecvTimeoutError, channel};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    // JavaScript completer for REPL
    struct JsCompleter;

    impl Completer for JsCompleter {
        type Candidate = Pair;

        fn complete(
            &self,
            line: &str,
            pos: usize,
            _ctx: &Context<'_>,
        ) -> rustyline::Result<(usize, Vec<Pair>)> {
            // Check if we're completing a property (after a dot)
            if let Some(dot_pos) = line[..pos].rfind('.') {
                // Get the object name before the dot
                let before_dot = &line[..dot_pos];
                let obj_start = before_dot
                    .rfind(|c: char| !c.is_alphanumeric() && c != '_')
                    .map(|i| i + 1)
                    .unwrap_or(0);
                let obj_name = &before_dot[obj_start..];

                // Get the property prefix after the dot
                let property_start = dot_pos + 1;
                let property_prefix = &line[property_start..pos];

                // Property completions for known objects
                let properties = match obj_name {
                    "console" => vec![
                        "log",
                        "error",
                        "warn",
                        "info",
                        "debug",
                        "trace",
                        "assert",
                        "clear",
                        "count",
                        "countReset",
                        "dir",
                        "dirxml",
                        "group",
                        "groupCollapsed",
                        "groupEnd",
                        "table",
                        "time",
                        "timeEnd",
                        "timeLog",
                        "timeStamp",
                    ],
                    "Math" => vec![
                        "abs", "acos", "acosh", "asin", "asinh", "atan", "atan2", "atanh", "cbrt",
                        "ceil", "clz32", "cos", "cosh", "exp", "expm1", "floor", "fround", "hypot",
                        "imul", "log", "log10", "log1p", "log2", "max", "min", "pow", "random",
                        "round", "sign", "sin", "sinh", "sqrt", "tan", "tanh", "trunc", "E",
                        "LN10", "LN2", "LOG10E", "LOG2E", "PI", "SQRT1_2", "SQRT2",
                    ],
                    "Array" => vec!["from", "isArray", "of", "prototype"],
                    "Object" => vec![
                        "assign",
                        "create",
                        "defineProperty",
                        "defineProperties",
                        "entries",
                        "freeze",
                        "fromEntries",
                        "getOwnPropertyDescriptor",
                        "getOwnPropertyDescriptors",
                        "getOwnPropertyNames",
                        "getOwnPropertySymbols",
                        "getPrototypeOf",
                        "is",
                        "isExtensible",
                        "isFrozen",
                        "isSealed",
                        "keys",
                        "preventExtensions",
                        "prototype",
                        "seal",
                        "setPrototypeOf",
                        "values",
                    ],
                    "String" => vec!["fromCharCode", "fromCodePoint", "raw", "prototype"],
                    "Number" => vec![
                        "isFinite",
                        "isInteger",
                        "isNaN",
                        "isSafeInteger",
                        "parseFloat",
                        "parseInt",
                        "EPSILON",
                        "MAX_SAFE_INTEGER",
                        "MAX_VALUE",
                        "MIN_SAFE_INTEGER",
                        "MIN_VALUE",
                        "NEGATIVE_INFINITY",
                        "POSITIVE_INFINITY",
                        "NaN",
                        "prototype",
                    ],
                    "Promise" => vec![
                        "all",
                        "allSettled",
                        "any",
                        "race",
                        "reject",
                        "resolve",
                        "prototype",
                    ],
                    "JSON" => vec!["parse", "stringify"],
                    "Date" => vec!["now", "parse", "UTC", "prototype"],
                    "RegExp" => vec!["prototype"],
                    "Error" => vec!["prototype"],
                    "URL" => vec!["createObjectURL", "revokeObjectURL", "prototype"],
                    "URLSearchParams" => vec!["prototype"],
                    "Headers" => vec![
                        "append", "delete", "entries", "forEach", "get", "has", "keys", "set",
                        "values",
                    ],
                    "Request" => vec![
                        "body",
                        "cache",
                        "clone",
                        "credentials",
                        "headers",
                        "integrity",
                        "method",
                        "mode",
                        "redirect",
                        "referrer",
                        "url",
                    ],
                    "Response" => vec![
                        "body",
                        "bodyUsed",
                        "clone",
                        "headers",
                        "json",
                        "ok",
                        "redirected",
                        "status",
                        "statusText",
                        "text",
                        "type",
                        "url",
                    ],
                    _ => vec![],
                };

                let mut completions: Vec<Pair> = properties
                    .iter()
                    .filter(|p| p.starts_with(property_prefix))
                    .map(|p| Pair {
                        display: p.to_string(),
                        replacement: p.to_string(),
                    })
                    .collect();

                completions.sort_by(|a, b| a.display.cmp(&b.display));

                return Ok((property_start, completions));
            }

            // Regular keyword completion (no dot)
            let start = line[..pos]
                .rfind(|c: char| !c.is_alphanumeric() && c != '_' && c != '.')
                .map(|i| i + 1)
                .unwrap_or(0);

            let word = &line[start..pos];

            // Common JavaScript globals and jstime-specific APIs
            let keywords = vec![
                // JavaScript built-in objects
                "Array",
                "Boolean",
                "Date",
                "Error",
                "Function",
                "Math",
                "Number",
                "Object",
                "Promise",
                "RegExp",
                "String",
                "Symbol",
                "JSON",
                // Common globals
                "console",
                "undefined",
                "null",
                "true",
                "false",
                "Infinity",
                "NaN",
                "isNaN",
                "isFinite",
                "parseInt",
                "parseFloat",
                "encodeURI",
                "decodeURI",
                "encodeURIComponent",
                "decodeURIComponent",
                // jstime-specific
                "setTimeout",
                "setInterval",
                "clearTimeout",
                "clearInterval",
                "queueMicrotask",
                "URL",
                "URLSearchParams",
                // Fetch API
                "fetch",
                "Headers",
                "Request",
                "Response",
                // Common keywords
                "const",
                "let",
                "var",
                "function",
                "return",
                "if",
                "else",
                "for",
                "while",
                "break",
                "continue",
                "switch",
                "case",
                "default",
                "try",
                "catch",
                "finally",
                "throw",
                "new",
                "this",
                "typeof",
                "instanceof",
                "in",
                "of",
                "delete",
                "void",
                "async",
                "await",
                "class",
                "extends",
                "static",
                "import",
                "export",
                "from",
            ];

            let mut completions: Vec<Pair> = keywords
                .iter()
                .filter(|k| k.starts_with(word))
                .map(|k| Pair {
                    display: k.to_string(),
                    replacement: k.to_string(),
                })
                .collect();

            completions.sort_by(|a, b| a.display.cmp(&b.display));

            Ok((start, completions))
        }
    }

    impl Hinter for JsCompleter {
        type Hint = String;
    }

    impl Highlighter for JsCompleter {}

    impl Validator for JsCompleter {
        fn validate(&self, _ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
            Ok(ValidationResult::Valid(None))
        }
    }

    impl Helper for JsCompleter {}

    let mut rl = Editor::<JsCompleter, DefaultHistory>::with_config(
        rustyline::Config::builder()
            .completion_type(rustyline::CompletionType::List)
            .build(),
    )
    .unwrap();
    rl.set_helper(Some(JsCompleter));
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
                    Ok(v) => println!("{v}"),
                    Err(e) => eprintln!("Uncaught: {e}"),
                }
                jstime.tick_event_loop();
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
