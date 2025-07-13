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
