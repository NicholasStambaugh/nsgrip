use std::env;
use std::process;

use nsgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|_err| {
        eprintln!("Problem parsing arguments: {_err}");
        process::exit(1);
    });

    if let Err(_e) = nsgrep::run(config) {
        eprintln!("CRITICAL ERROR: {_e}");
        process::exit(1);
    }
}
