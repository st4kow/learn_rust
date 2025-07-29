use std::fs;
use std::error::Error;
use std::env; // Working with environment variables

//// Config ////
pub struct Config {
	pub query: String,
	pub file_path: String,
    pub ignore_case: bool
}
impl Config {
	pub fn build(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("Not enough arguments!");
		}
		let query = args[1].clone();
		let file_path = args[2].clone();
        
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
		Ok( Config { query, file_path, ignore_case } )
	}
}

//// Run ////
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.file_path)?;    
    //println!("With text:\n{contents}"); 
    
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
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

////**** TESTS ****////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
	//searching for the line contatining "query". Only one result
    fn one_result() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents) );
    }
    
    #[test]
	//searching for the line contatining "query". Only one result
    fn two_result() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
productive.";

		assert_eq!(vec!["safe, fast, productive.","productive."], search(query, contents) );
    }
    
    #[test]
	//searching for the line contatining "query". Only one result
    fn no_result() {
		let query = "duct";
		let contents = "\
Rust:";
    let exp_res: Vec<&str> = Vec::new();
		assert_eq!(exp_res, search(query, contents) );
    }
    
    #[test]
	//searching for the line contatining "query". Only one result
    fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
		assert_eq!(vec!["safe, fast, productive."], search(query, contents) );
    }
    
    #[test]
	//searching for the line contatining "query". Only one result
    fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
		assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents) );
    }
}
