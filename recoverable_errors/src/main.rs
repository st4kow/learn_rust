use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Lets try opening a file");
	
	/*
	let greeting_file_result = File::open("hello.txt");
	let greeting_file = match greeting_file_result {
		Ok(file) => file,
		Err(error) => panic!("Problem opening the file: {error:?}"),
	};
	*/
	
	// Matching on Different Error types
	let greeting_file_result = File::open("hello.txt");
	let greeting_file = match greeting_file_result {
		Ok(file) => file,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create("hello.txt") {
				Ok(fc) => fc,
				Err(e) => panic!("Problem creating the file: {e:?}")
			},
			_=> { // For every other error kinds 
				panic!("Problem opening the file: {error:?}");
			}
		}
	};
	
	//Shortcuts for panic
	
	
}
