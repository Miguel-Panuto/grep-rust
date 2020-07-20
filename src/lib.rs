use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?; // This function reads a outside file
 
    println!("With text:\n {}", contents);

    Ok(())
}