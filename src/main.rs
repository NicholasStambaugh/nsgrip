use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Use the Config::new function to create a Config instance
    let config = Config::new(&args);
    
    println!("Search for {}", config.query);
    println!("in file... {}", config.file_path);

    // Use the fields directly from the config instance
    let contents = fs::read_to_string(&config.file_path).expect("Should be able to read file!");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // Rename the function to be more descriptive
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        // Use the fields directly instead of creating a new Config instance
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
