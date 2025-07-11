fn main() {
    let _s = "hello"; // This is a "string" liternal.
                      // unmutable, fixed size
                      // So can be stored on the stack
    let mut s = String::from("hello"); // This is a complex dynamic string type
                                       // Can be changes, so we do not kow the size
                                       // Need to be stored on the heap, fined place for it
                                       // We access it by a pointer (fix sized) on the stack
    
    s.push_str(", world!"); // Chaning the content.
                            // s is a point on the stack
                            // the contant is in the heap so the size can be increased
    println!("Print, becasue why not: {}", s);
    
    //When the memory of s will be freed?
    //When s goes out of scope, guaranteed by the compiler.
    //Compiler calls "drop" function autoatically when the pointer
    //to a heap content goes out of scope.
    
    //***************************//
    println!("--Variables and Data intercating with move--");
    
    //***simple types***//
    let x = 5;
    let y = x;
    // What happens here?
    // x is known, on the stack with value 5
    // y is known, on the stack value is 5
    // simple, real copy of the content
    
    let s1 = String::from("hello");
    let s2 = s1;
    // What happens here? 
    // s1 known pointer on the stack. hello allocated in heap.
    // s2 is known referene on stack. points to the same place
    // string content was not copied!
    // This is called "Shallow copy", but also invalidated s1.
    
    //IMPORTENT OWNERSHIP
    //println!("{s1}"); Does not work! s1 is no logner valid
    //This is ro prevent double free up when s1 and s2 goes out of scope
    
    println!("Only s2 is valid: {}", s2);
    
    //Rust never automatically created "deep copies" of data
    //So all automatic copies are fast and inexpensive
    
    //****************//
    println!("--Scope and Assignment--");
    
    let mut s = String::from("hello");
    s = String::from("ahoy");
    // Here "drop" was called to free up the heap space
    // Than asked for a new heap space to store ahoy
    // also pointer content changed.
    // This is expensive
    
    println!("{}, World!!!", s);
    
    println!("--Variables and Data interacting with clone--");
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
    // Here the heap content is also copied to an other heap space
    
    println!("--Stack-Only Data - Copy trait");
    // Types that implement Copy traint are always deep copied
    // Like u32, i32, bool, string literals.
    // Only types that do not use the heap can impelent this
    println!("--Ownership and functions--");
	let s = String::from("hello"); // s comes into scope
	takes_ownership(s);            // s's value goes into function
	                               // s is no logner valid here
    // println!("{s}"); //This does not work
	
	let x = 5;                     // x comes into scope
	makes_copy(x);                 // because i32 implements the  Copy trait
	                               // x does not move into the function
								   // so it is okay to use x afterward
	println!("{x}");
	
    println!("--Return values and Scope--");
	let s1 = gives_ownership(); // s1 comes into scope, return value moves to s1
	let s2 = String::from("hello"); // s2 comes into scope
	let s3 = takes_and_gives_back(s2); //s2 is invalid from here, s3 comes into scope
	
	println!("--Returning tuples--");
	let s1 = String::from("text");
	let (s2, len) = calculate_length(s1);
	println!("Result: {s2}, with length: {len}");
}
fn takes_ownership(some_string: String) { //some_string comes into scope
    println!("{some_string}");
} // Here some_string goes out of scope! drop is called, memory freed

fn makes_copy(some_integer: i32) { //some_integer comes into scope
    println!("{some_integer}");
} // Here some integer goes out of scope, nothing special happens

fn gives_ownership() -> String { // Will move its value to ret value
                                 // to the function that calls it
    let some_string = String::from("yours"); //some_string comes into scope
	some_string // returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
	// a_string comes into scope
	a_string // a_string returned and goes to the calling function
}
fn calculate_length(s: String) -> (String, usize) { // Return multiple values as a tuple
	let length = s.len();
	(s, length)
}
