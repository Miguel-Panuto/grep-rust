use std::env; // Environment lib

fn main() {
    let args:Vec<String> = env::args().collect(); // That will read the user entry args
    print!("{:?}", args); // Print the args
}
