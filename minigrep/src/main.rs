use std::env; //args() lives here. For inputs from the command line
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args ).unwrap_or_else(|err| {
		println!("Problem parsing the arguments: {err}");
		process::exit(1);
	});
	if let Err(e) = minigrep::run(config) {
		println!("Application error: {e}");
		process::exit(1);
	} // using if let is better than match because we do not care about the () value
}