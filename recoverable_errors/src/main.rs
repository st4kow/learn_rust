use std::fs   ::{self, File};
use std::io   ::{self, ErrorKind, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> { //Main can slo return with types, that implement std::process:Termination trait
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
	//TODO Add content here
	
	//Propagating errors
	let username_result = read_username_from_file();
	let mut username: String =  match username_result {
		Ok(uname) => uname,
		Err(e) => panic!("Error during file reading: {e}")
	};
	println!("Username from file: {username}");
	
	let username_result = read_username_from_file1();
	let mut username: String =  match username_result {
		Ok(uname) => uname,
		Err(e) => panic!("Error during file reading: {e}")
	};
	println!("Username from file: {username}");
	
	let username_result = read_username_from_file2();
	let mut username: String =  match username_result {
		Ok(uname) => uname,
		Err(e) => panic!("Error during file reading: {e}")
	};
	println!("Username from file: {username}");
	
	let username_result = read_username_from_file3();
	let mut username: String =  match username_result {
		Ok(uname) => uname,
		Err(e) => panic!("Error during file reading: {e}")
	};
	println!("Username from file: {username}");
	
	Ok(()) //Terminatin function will convert this to return value integer 0 to be compatible with C
	       //Error would be converted to a positive integer
}
fn read_username_from_file() -> Result<String, io::Error> { // Using matches
	let username_file_result = File::open("hello.txt");
	let mut username_file = match username_file_result {
		Ok(file) => file,
		Err(e) => return Err(e) //Returns with error result
	};
	let mut username = String::new();
	match username_file.read_to_string(&mut username) {
		Ok(_) => Ok(username), //Return with username result
		Err(e) => Err(e)  //Return with error result
	}
}

fn read_username_from_file1() -> Result<String, io::Error> { // Using ? to propagate errors
    let mut username_file = File::open("hello.txt")?; //fn returns with Err(e) if Error
	let mut username = String::new();
	username_file.read_to_string(&mut username)?; //fn returns with Err(e) if Error
	Ok(username)  //fn Returns with Ok(username)
	
	/*
	   ? works on types with "From" trait implemented.
	   ? calls the from function
	*/
}

fn read_username_from_file2() -> Result<String, io::Error> { // Making it more shorter
    let mut username = String::new();
	File::open("hello.txt")?.read_to_string(&mut username)?; // Returns is any error, or does the job
	Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> { // Using function that already does this
	fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> { // ? can also be used on Options
	text.lines() //line gives an iterator
	    .next()? //read next (first) line
		.chars() //gives iterator over the characters
		.last() //get the last character
}
