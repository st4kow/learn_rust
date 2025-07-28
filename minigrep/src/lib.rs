use std::fs;
use std::error::Error;

//// Config ////
pub struct Config {
	query: String,
	file_path: String
}
impl Config {
	pub fn build(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("Not enough arguments!");
		}
		let query = args[1].clone();
		let file_path = args[2].clone();
		Ok( Config { query, file_path } )
	}
}

//// Run ////
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.file_path)?;    
    //println!("With text:\n{contents}"); 
	
	Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	vec![]
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
}
