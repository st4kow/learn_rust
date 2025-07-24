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
	
	//Alternative, using closures
	let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
	
	//Shortcuts for panic unwrap, expect
	let greeting_file = File::open("hello.txt").unwrap();
	// Unwrap return with the content ok Ok(...)
	// Or if Err, panics
	
	let greeting_file = File::open("hello.txt").expect("error using expect");
	// Expect does the same
	// But proper panic message can be added
}
