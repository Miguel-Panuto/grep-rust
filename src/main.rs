use std::env; // Environment lib
use std::process;

use grep_prop::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // That will read the user entry args

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err); // Handle in error case
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("Inside: {}", config.filename);

    if let Err(e) = grep_prop::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}