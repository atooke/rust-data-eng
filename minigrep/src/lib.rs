use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Error: not enough arguments, expect query & file_path");
        }
        let query = args[2].clone();
        let file_path = args[4].clone();

        let ignore_case = env::var("CASE_INSENSITIVE").is_ok();
        Ok(Config { query, file_path,ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    //will return the error value from the current function for the caller to handle
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insentitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    
    for line in results {
        println!("{line}");
    }
    Ok(())
}

//'a = lifetime of the reference
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insentitive<'a>( query: &str, contents: & 'a str,) -> Vec<& 'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}




//unit tests section
#[cfg(test)] // only compile when running tests
mod tests {
    use super::*;
    #[test]
    fn cases_senstive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents));
    }

    #[test] //mark function as test
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
