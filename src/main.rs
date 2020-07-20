use std::env; // Environment lib

fn main() {
    let args:Vec<String> = env::args().collect(); // That will read the user entry args
    
    let query = &args[1]; // This will save what we looking for inside this variable
    let filename = &args[2]; // And this the filename
    // By now we have the array: [where is compiled, query, filename]

    println!("Searching for {}", query);
    println!("Inside: {}", filename);
}
