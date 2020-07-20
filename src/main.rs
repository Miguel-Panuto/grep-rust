use std::env; // Environment lib
use std::fs;

fn main() {
    let args:Vec<String> = env::args().collect(); // That will read the user entry args
    
    // Pointer must be pass, not pass the clone and maintain the ownership
    let config = parse_config(&args); // Return tuple, the query and filename
    // By now we have the array: [where is compiled, query, filename]

    println!("Searching for {}", config.query);
    println!("Inside: {}", config.filename);

    let contents = fs::read_to_string(config.filename) // This function reads a outside file
        .expect("Something went wrong while reading the file"); // the filename is the path to

    println!("With text:\n {}", contents);
}

struct Config { // Can be stored inside a struct now
    query: String, // The code will be more controlled
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone(); // This will save what we looking for inside this variable
    // Cannot borrow the string, the clone guarantees that, but in the change for more simplicity
    // a little of performance give it away
    let filename = args[2].clone(); // And this the filename

    Config { query, filename } // Return a config struct
}