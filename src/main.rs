use std::env; // Environment lib
use std::process;

use grep_prop::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // That will read the user entry args

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); // Handle in error case
        process::exit(1);
    });

    if let Err(e) = grep_prop::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}