use std::env; // Environment lib
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect(); // That will read the user entry args
                                                   // Pointer must be pass, not pass the clone and maintain the ownership
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("Inside: {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?; // This function reads a outside file
 
    println!("With text:\n {}", contents);

    Ok(())
}

struct Config {
    // Can be stored inside a struct now
    query: String, // The code will be more controlled
    filename: String,
}

impl Config {
    // This part of the struct
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone(); // This will save what we looking for inside this variable
                                     // Cannot borrow the string, the clone guarantees that, but in the change for more simplicity
                                     // a little of performance give it away

        let filename = args[2].clone(); // And this the filename

        Ok(Config { query, filename }) // Return a config struct
    }
}
