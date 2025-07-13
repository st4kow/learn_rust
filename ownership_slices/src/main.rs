fn main() {
    println!("--Ownership_slices--");
	let mut s: String = String::from("hello world");
    let word =  first_word_dumb(&s);
    s.clear();
	println!("The first word starts at {word} index.");
    // Here word still has a value but it is not meaningful	
	// Issue s is not connected to word in any way
	
	println!("--String slices--");
	let s = String::from("hello world");
	let hello = &s[0..5]; //ending index is one more than the last contained. [..:..) mathematicaly
	let world = &s[6..11];
	
	//Slice is a data structure that stores a pointer to the starting index and a length parameter (upper index - lower index)
	
	let slice = &s[0..2];
	let slice = &s[..2]; // These are equal
	
	let slice = &s[1..s.len()];
	let slice = &s[1..]; // These are also equal
	
	let mut s: String = String::from("hello world");
	let word = first_word(&s);
	println!("Printing out the first word using slice: {word}");
	s.clear();
	//println!("Printing out the first word using slice after clear: {word}");
	//This does not work becasue both mutable and unmutable references would be valid at the same time
	
	
	println!("--String literals as slices--");
	let s = "Hello, world"; // This IS a string slice
	println!("{s}");
	let s: &str = "Hello, world"; //This is the same
	println!("{s}");
	
	println!("--Making the first word function general--");
	let my_string = String::from("This is my sting!");
	let word = first_word_universal(&my_string[..10]); // works on slices
	let word = first_word_universal(&my_string[..]); //exotic slices
	let word = first_word_universal(&my_string); // Strings (becasue of deref coercion)
	let my_string_literal = "This_is_my_string!";
	let word = first_word_universal(&my_string_literal[..10]); // works on slices of string literals
	let word = first_word_universal(&my_string_literal[..]); //exotic slices of string literal
	let word = first_word_universal(&my_string_literal); //directly on string literals, becasue those are already slices
	
	println!("--Other slices--");
	let a: [isize; 5] = [1, 2, 3, 4, 5];
	//How would we refer to part of an array?
	let slice: &[isize] = &a[1..3] // 2, 3
	assert_eq!(slice, &[2, 3]);
	
}

fn first_word_dumb(s: &String) -> usize {
	let bytes = s.as_bytes(); // This converts the string to bytes
	for (i, &item) in bytes.iter().enumerate() { //iter() return each element in a collection, emumerate() wraps it in a tuple instead
		if item == b' ' {
			return i;
		}
	}
	s.len()
}

fn first_word(s: &String) -> &str {
	let bytes = s.as_bytes();
	for  (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}
	&s[..] //Return entire string as slice, no lower and upper bounds
}

fn first_word_universal(s: &str) -> &str { // &str is more universal becasue works for String, str, slices, etc
	let bytes = s.as_bytes();
	for  (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}
	&s[..] //Return entire string as slice, no lower and upper bounds
}
