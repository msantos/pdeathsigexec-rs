use clap::Parser;
use std::ffi::CString;
use std::process::exit;

use pdeathsigexec::{execvp, signal};

/// signal process when parent exits
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// termination signal
    #[clap(short, long, default_value_t = 9)]
    signal: i32,

    #[clap(required = true)]
    argv: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let argv: Vec<_> = args
        .argv
        .iter()
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect();

    match signal(args.signal) {
        Ok(_) => (),
        Err(errno) => exit(errno),
    }

    exit(execvp(argv));
}
