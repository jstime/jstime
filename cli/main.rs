use dirs::home_dir;
use jstime::JSTime;
use jstime_core as jstime;
use regex::Regex;
use rustyline::Editor;
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

#[allow(dead_code)]
const JS_GET_GLOBAL_BUILTINS: &str = "Object.getOwnPropertyNames(globalThis)";

/// Initializes the JSTime Core
fn init_runtime(opt: &Opt) -> JSTime {
    if opt.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    jstime::init(
        opt.v8_options
            .as_ref()
            .map(|o| o.split(' ').map(|s| s.to_owned()).collect()),
    );

    let mut options = jstime::Options::default();
    options.snapshot = Some(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/snapshot_data.blob"
    )));

    JSTime::new(options)
}

fn main() {
    let opt = Opt::from_args();
    let mut jstime = init_runtime(&opt);
    if let Some(filename) = opt.filename {
        std::process::exit(match jstime.import(&filename) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("{:?}", e);
                1
            }
        });
    } else {
        let history_file = home_dir().map(|mut p| {
            p.push(".jstime_repl_history");
            p
        });

        Repl::new(&mut jstime, history_file.unwrap()).readline();
    }
}

pub struct Repl<'a> {
    jstime: &'a mut JSTime,
    history_file: PathBuf,
}

// Rustyline seems to need an external struct to work as intended.
pub struct RustylineHelper {
    completions: Vec<String>,
}

impl rustyline::Helper for RustylineHelper {}
impl rustyline::hint::Hinter for RustylineHelper {}
impl rustyline::validate::Validator for RustylineHelper {}
impl rustyline::highlight::Highlighter for RustylineHelper {}

impl rustyline::completion::Completer for RustylineHelper {
    // Candidate is an assosciated type, that helps the readlineHelper know the type of completion.
    // Here the candidate refers to the "closest possible match" to a given string from all the completions / candidates available.
    type Candidate = String;

    fn complete(
        &self,
        _line: &str,
        _pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> std::result::Result<(usize, Vec<String>), rustyline::error::ReadlineError> {
        let completions = self.completions.clone();
        Ok((0, completions))
    }

    fn update(&self, line: &mut rustyline::line_buffer::LineBuffer, start: usize, elected: &str) {
        let end = line.pos();
        line.replace(start..end, elected)
    }
}

impl<'a> Repl<'a> {
    pub fn new(jstime: &'a mut JSTime, history_file: PathBuf) -> Self {
        let repl = Self {
            jstime,
            history_file,
        };
        repl
    }

    // A rust implementation of the complete function from node repl.
    // https://github.com/nodejs/node/blob/c205f672e9cf0c70ea26f87eb97342947a244e18/lib/repl.js#L1136
    pub fn generate_completions(&mut self, line: &str) -> Vec<String> {
        // <tab>: When pressed on a blank line, displays global and local (scope) variables.
        // When pressed while entering other input, displays relevant autocompletion options.

        // SIMPLE_EXPR_RE - Regex that splits input source into capture groups of Object property access
        lazy_static! {
            static ref SIMPLE_EXPR_RE: Regex =
                Regex::new(r"(?m)(?:[a-zA-Z_$](?:\w|\$)*\??\.)*[a-zA-Z_$](?:\w|\$)*\??\.?$")
                    .unwrap();
        };

        // populate completion_groups based on the what the contextual completions for the line
        let completion_groups: Vec<String> = vec![];

        // return global builtins if the input line is empty.
        if line.is_empty() {
            let result = self
                .jstime
                .run_script(JS_GET_GLOBAL_BUILTINS, "jstime-repl");
            let builtins = result.unwrap().split(',').map(|x| x.to_string()).collect();
            builtins
        } else {
            match SIMPLE_EXPR_RE.captures(line) {
                Some(x) => {
                    let complete_on = &x[0];
                    let mut expr = String::from("");

                    if line.ends_with(".") {
                        // expr = complete_on.split()
                    } else {
                        let bits: Vec<&str> = complete_on.split(".").collect();
                        // filter = bits.pop().unwrap();
                        expr = bits.join(".");
                    }

                    if expr.is_empty() {
                    } else {
                        // based on the new node repl, we should probably parse the source into an ESTree
                        // and then try to get the corresponding propertyNames?
                    }
                }

                None => {}
            }
            completion_groups
        }
    }

    pub fn readline(&mut self) {
        use rustyline::error::ReadlineError;

        // This looks kinda weird, but Rustyline Editor requires a struct to be passed as Helper.
        // Since Repl has already impl'd all the traits necessary for Helper we use that instead.

        let helper = RustylineHelper {
            completions: vec![],
        };

        let mut rl = Editor::new();
        rl.set_helper(Some(helper));

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
                        _ => match self.jstime.run_script(&line, ".repl") {
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

        rl.save_history(&self.history_file).unwrap();
    }
}

#[test]

fn test_global_completions() {
    jstime::init(Some(vec![]));

    let mut options = jstime::Options::default();
    options.snapshot = Some(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/snapshot_data.blob"
    )));

    let mut jstime = JSTime::new(options);

    fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }

    let globals = vec![
        "Object",
        "Function",
        "Array",
        "Number",
        "parseFloat",
        "parseInt",
        "Infinity",
        "NaN",
        "undefined",
        "Boolean",
        "String",
        "Symbol",
        "Date",
        "Promise",
        "RegExp",
        "Error",
        "EvalError",
        "RangeError",
        "ReferenceError",
        "SyntaxError",
        "TypeError",
        "URIError",
        "globalThis",
        "JSON",
        "Math",
        "console",
        "ArrayBuffer",
        "Uint8Array",
        "Int8Array",
        "Uint16Array",
        "Int16Array",
        "Uint32Array",
        "Int32Array",
        "Float32Array",
        "Float64Array",
        "Uint8ClampedArray",
        "BigUint64Array",
        "BigInt64Array",
        "DataView",
        "Map",
        "BigInt",
        "Set",
        "WeakMap",
        "WeakSet",
        "Proxy",
        "Reflect",
        "decodeURI",
        "decodeURIComponent",
        "encodeURI",
        "encodeURIComponent",
        "escape",
        "unescape",
        "eval",
        "isFinite",
        "isNaN",
        "queueMicrotask",
        "SharedArrayBuffer",
        "Atomics",
        "AggregateError",
        "FinalizationRegistry",
        "WeakRef",
        "WebAssembly",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect();

    let history_file = home_dir().map(|mut p| {
        p.push(".jstime_repl_history");
        p
    });

    let repl = &mut Repl::new(&mut jstime, history_file.unwrap());
    let completions = repl.generate_completions("");
    assert_eq!(do_vecs_match(&completions, &globals), true);
}
