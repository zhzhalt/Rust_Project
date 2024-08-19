use std::env;
use std::process;

use minigrep::Config;

// main should handle running the program
// lib has the actual logic

fn main() {
    let args: Vec<String> = env::args().collect(); // good to annotate it since Rust can't infer the collection you want when using .collect()
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    // the above steps are to read the arguments from the command line and put them in a vector<string>.

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}



