use std::env;
use std::process;

use nsgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Use the Config::build function to create a Config instance
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguemnt: {err}");
        process::exit(1);
    });

    println!("Search for {}", config.query);
    println!("in file... {}", config.file_path);

    if let Err(e) = nsgrep::run(config) {
        println!("CRITICAL ERROR: {e}");
        process::exit(1);
    }
}
