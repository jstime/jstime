extern crate jstime;

use std::env;
use std::process;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "jstime", rename_all = "kebab-case")]
struct Opt {
    /// File to read from, or "-" to read from stdin. Interactive mode if a tty
    #[structopt(name = " file | - ", default_value = "-")]
    filename: String,

    /// Prints version information
    #[structopt(short, long)]
    version: bool,

    /// Prints V8 options
    #[structopt(long)]
    v8_options: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut non_v8_args: Vec<String> = Vec::with_capacity(args.len());
    let mut v8_args: Vec<String> = Vec::with_capacity(args.len());
    // Kind of hacky, but sub out "--help" for "-h" before passing to V8 to avoid
    // unwanted V8 help output. Similarly, pass "--help" to V8 when "--v8-options" is passed
    // to jstime.
    for arg in args {
        match &arg[..] {
            "--help" => non_v8_args.push(arg.to_string()),
            "--v8-options" => {
                non_v8_args.push(arg.to_string());
                v8_args.push("--help".to_string());
            }
            _ => v8_args.push(arg.to_string()),
        }
    }

    let mut unparsed_v8_args = jstime::bootstrap::set_flags(v8_args);
    unparsed_v8_args.append(&mut non_v8_args);
    let opt = Opt::from_iter(unparsed_v8_args);

    if opt.v8_options {
        process::exit(0);
    }

    if opt.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    let filename = &opt.filename[..];

    match filename {
        "-" => jstime::repl::start(),
        _ => jstime::module::run_file(filename),
    }
}
