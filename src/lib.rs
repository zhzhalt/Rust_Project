use std::error::Error;
use std::fs;
use std::env;


// write a struct to hold all the configuration variables (command line arguments) instead, for cleaner code

// adding an environment variable so the user can choose if they want to do case sensitive or insensitive search
// env variable: something whose value is set outside the code. user defines.

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// clone lets us not worry about ownership and all that, but takes more runtime.

// instead of panicking, using Result as a return value is better because it gets rid of 
// unnecessary text for users to read, and makes more sense for it to be returned. 

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();    // Err is returned if environment variable is not set. if so, ignore case returns false;
    
        Ok(Config { query, file_path, ignore_case, })
    }
}

// if there is an error here when reading the file, instead of expect, a Result will be returned. 

// the box thing is a trait object. 
// it means returns a type that implements error trait, don't have to specify the type.

// ? will return the error value from the current function that the caller has to handle.

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// whats 'a means?
//  we tell Rust that the data returned by the search function will live as long as the data passed into the search function in the contents argument

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
 

// first writing a failing test, then a test that barely passes, then modifying to fit our needs.

// \ tells Rust not to put a newline char at the beginning of the contents of that string literal

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";     

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
