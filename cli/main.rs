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
    use rustc_hash::FxHashMap;
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
    use std::sync::{Arc, Mutex, RwLock};
    use std::thread;
    use std::time::Duration;

    /// Cached completion data for the REPL.
    /// This is populated at startup and refreshed after each command.
    struct CompletionCache {
        /// Global names (properties of globalThis)
        globals: Vec<String>,
        /// Cached property names for known objects (lazily populated)
        properties: FxHashMap<String, Vec<String>>,
    }

    impl CompletionCache {
        fn new() -> Self {
            CompletionCache {
                globals: Vec::new(),
                properties: FxHashMap::default(),
            }
        }
    }

    // JavaScript keywords that should always be available for completion
    const JS_KEYWORDS: &[&str] = &[
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
        "true",
        "false",
        "null",
        "undefined",
    ];

    /// JavaScript completer for REPL that uses dynamically discovered completions.
    struct JsCompleter {
        cache: Arc<RwLock<CompletionCache>>,
    }

    impl JsCompleter {
        fn new(cache: Arc<RwLock<CompletionCache>>) -> Self {
            JsCompleter { cache }
        }
    }

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
                // Get the object expression before the dot
                let before_dot = &line[..dot_pos];
                let obj_start = before_dot
                    .rfind(|c: char| !c.is_alphanumeric() && c != '_' && c != '.')
                    .map(|i| i + 1)
                    .unwrap_or(0);
                let obj_expr = &before_dot[obj_start..];

                // Get the property prefix after the dot
                let property_start = dot_pos + 1;
                let property_prefix = &line[property_start..pos];

                // Look up cached properties for this object
                let cache = self.cache.read().unwrap();
                let properties = cache.properties.get(obj_expr);

                let mut completions: Vec<Pair> = if let Some(props) = properties {
                    props
                        .iter()
                        .filter(|p| p.starts_with(property_prefix))
                        .map(|p| Pair {
                            display: p.to_string(),
                            replacement: p.to_string(),
                        })
                        .collect()
                } else {
                    Vec::new()
                };

                completions.sort_by(|a, b| a.display.cmp(&b.display));

                return Ok((property_start, completions));
            }

            // Regular keyword/global completion (no dot)
            let start = line[..pos]
                .rfind(|c: char| !c.is_alphanumeric() && c != '_' && c != '.')
                .map(|i| i + 1)
                .unwrap_or(0);

            let word = &line[start..pos];

            // Combine JavaScript keywords with dynamically discovered globals
            let cache = self.cache.read().unwrap();
            let mut completions: Vec<Pair> = JS_KEYWORDS
                .iter()
                .map(|s| s.to_string())
                .chain(cache.globals.iter().cloned())
                .filter(|k| k.starts_with(word))
                .map(|k| Pair {
                    display: k.clone(),
                    replacement: k,
                })
                .collect();

            // Remove duplicates and sort
            completions.sort_by(|a, b| a.display.cmp(&b.display));
            completions.dedup_by(|a, b| a.display == b.display);

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

    // Global values that don't have meaningful properties for completion
    const JS_PRIMITIVES: &[&str] = &["undefined", "NaN", "Infinity"];

    // Helper function to refresh the completion cache
    fn refresh_cache(jstime: &mut jstime::JSTime, cache: &Arc<RwLock<CompletionCache>>) {
        let globals = jstime.get_global_names();

        // Get properties for each global that looks like an object
        let mut properties = FxHashMap::default();
        for name in &globals {
            // Skip keywords and primitives
            if JS_KEYWORDS.contains(&name.as_str()) || JS_PRIMITIVES.contains(&name.as_str()) {
                continue;
            }

            let props = jstime.get_property_names(name);
            if !props.is_empty() {
                properties.insert(name.clone(), props);
            }
        }

        // Also get nested properties for crypto.subtle
        let subtle_props = jstime.get_property_names("crypto.subtle");
        if !subtle_props.is_empty() {
            properties.insert("crypto.subtle".to_string(), subtle_props);
        }

        // Update the cache
        let mut cache_guard = cache.write().unwrap();
        cache_guard.globals = globals;
        cache_guard.properties = properties;
    }

    // Initialize the completion cache
    let cache = Arc::new(RwLock::new(CompletionCache::new()));
    refresh_cache(&mut jstime, &cache);

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
                    Ok(v) => println!("{v}"),
                    Err(e) => eprintln!("Uncaught: {e}"),
                }
                jstime.tick_event_loop();

                // Refresh completion cache to pick up any new global variables
                refresh_cache(&mut jstime, &cache);
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
