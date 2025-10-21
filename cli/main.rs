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
    use rustyline::{error::ReadlineError, history::DefaultHistory, Editor, completion::{Completer, Pair}, Context};
    use rustyline::highlight::Highlighter;
    use rustyline::hint::Hinter;
    use rustyline::validate::{Validator, ValidationResult, ValidationContext};
    use rustyline::Helper;
    use std::sync::mpsc::{channel, RecvTimeoutError};
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
                        "log", "error", "warn", "info", "debug", "trace",
                        "assert", "clear", "count", "countReset", "dir", "dirxml",
                        "group", "groupCollapsed", "groupEnd", "table", "time",
                        "timeEnd", "timeLog", "timeStamp",
                    ],
                    "Math" => vec![
                        "abs", "acos", "acosh", "asin", "asinh", "atan", "atan2", "atanh",
                        "cbrt", "ceil", "clz32", "cos", "cosh", "exp", "expm1", "floor",
                        "fround", "hypot", "imul", "log", "log10", "log1p", "log2", "max",
                        "min", "pow", "random", "round", "sign", "sin", "sinh", "sqrt",
                        "tan", "tanh", "trunc", "E", "LN10", "LN2", "LOG10E", "LOG2E",
                        "PI", "SQRT1_2", "SQRT2",
                    ],
                    "Array" => vec![
                        "from", "isArray", "of", "prototype",
                    ],
                    "Object" => vec![
                        "assign", "create", "defineProperty", "defineProperties",
                        "entries", "freeze", "fromEntries", "getOwnPropertyDescriptor",
                        "getOwnPropertyDescriptors", "getOwnPropertyNames",
                        "getOwnPropertySymbols", "getPrototypeOf", "is", "isExtensible",
                        "isFrozen", "isSealed", "keys", "preventExtensions", "prototype",
                        "seal", "setPrototypeOf", "values",
                    ],
                    "String" => vec![
                        "fromCharCode", "fromCodePoint", "raw", "prototype",
                    ],
                    "Number" => vec![
                        "isFinite", "isInteger", "isNaN", "isSafeInteger", "parseFloat",
                        "parseInt", "EPSILON", "MAX_SAFE_INTEGER", "MAX_VALUE",
                        "MIN_SAFE_INTEGER", "MIN_VALUE", "NEGATIVE_INFINITY",
                        "POSITIVE_INFINITY", "NaN", "prototype",
                    ],
                    "Promise" => vec![
                        "all", "allSettled", "any", "race", "reject", "resolve", "prototype",
                    ],
                    "JSON" => vec![
                        "parse", "stringify",
                    ],
                    "Date" => vec![
                        "now", "parse", "UTC", "prototype",
                    ],
                    "RegExp" => vec![
                        "prototype",
                    ],
                    "Error" => vec![
                        "prototype",
                    ],
                    "URL" => vec![
                        "createObjectURL", "revokeObjectURL", "prototype",
                    ],
                    "URLSearchParams" => vec![
                        "prototype",
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
                "Array", "Boolean", "Date", "Error", "Function", "Math", "Number",
                "Object", "Promise", "RegExp", "String", "Symbol", "JSON",
                // Common globals
                "console", "undefined", "null", "true", "false", "Infinity", "NaN",
                "isNaN", "isFinite", "parseInt", "parseFloat", "encodeURI", "decodeURI",
                "encodeURIComponent", "decodeURIComponent",
                // jstime-specific
                "setTimeout", "setInterval", "clearTimeout", "clearInterval",
                "queueMicrotask", "URL", "URLSearchParams",
                // Common keywords
                "const", "let", "var", "function", "return", "if", "else", "for", "while",
                "break", "continue", "switch", "case", "default", "try", "catch", "finally",
                "throw", "new", "this", "typeof", "instanceof", "in", "of", "delete", "void",
                "async", "await", "class", "extends", "static", "import", "export", "from",
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
            .build()
    ).unwrap();
    rl.set_helper(Some(JsCompleter));
    println!("Welcome to jstime v{}!", env!("CARGO_PKG_VERSION"));

    let history_path = home_dir().map(|mut p| {
        p.push(".jstime_repl_history");
        let _ = rl.load_history(&p);
        p
    });

    // Use Arc<Mutex<Vec<String>>> to share history entries across threads
    let history_entries = Arc::new(Mutex::new(Vec::new()));

    loop {
        // Channel for this readline
        let (tx, rx) = channel();
        let history_clone = Arc::clone(&history_entries);

        // Start readline in a separate thread
        thread::spawn(move || {
            let mut rl_temp = Editor::<JsCompleter, DefaultHistory>::with_config(
                rustyline::Config::builder()
                    .completion_type(rustyline::CompletionType::List)
                    .build()
            ).unwrap();
            rl_temp.set_helper(Some(JsCompleter));

            // Load recent history into the temp editor
            if let Ok(entries) = history_clone.lock() {
                for entry in entries.iter() {
                    let _ = rl_temp.add_history_entry(entry);
                }
            }

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
                // Add to both the main editor and shared history
                let _ = rl.add_history_entry(line.as_str());
                if let Ok(mut entries) = history_entries.lock() {
                    entries.push(line.clone());
                    // Keep only last 1000 entries to avoid unbounded growth
                    if entries.len() > 1000 {
                        entries.remove(0);
                    }
                }

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
