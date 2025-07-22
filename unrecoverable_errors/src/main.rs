fn main() {
	// Unrecoverable error, when we can not do anything with the error.
	// This is always a bug, not intentional
	// For example indexing out of an array
	// Or calling panic! macro
	
	// What happens at panic? 
	// 1, Print failure message
	// 2, Unwind
	// 3, clean up the stack (optionaly print the stack trace)
	// 4, quit
    
    //panic!("crash and burn");	
	
	let v = vec![1, 2, 3];
	v[99]; // Indexing out of the valid vecor space
}
