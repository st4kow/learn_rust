fn main() {
    let mut s: String = String::new(); //New empty String
	
	let data: &str = "initial contents"; //string slice
	let s = data.to_string(); //converting string slice to String
	                          //Every type has to_string() that implements Display trait
	let s = "initial contents".to_string(); // Directly on literal
	
	let s = String::from("initial contents"); // The conventional solution
	
	// Rust Strings are UTF-8, these are all valid
	let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
	
	// Updating strings
	let mut s: String = String::from("foo");
	s.push_str("bar"); // pushing a string slice
	
	let s2: &str = "bar";
	s.push_str(s2); //push_str does no take ownership of string slice
	println!("s2 is {s2}");
	println!("s is {s}");
	
	let mut s: String = String::from("lo");
	s.push('l'); // push takes one character! Put it at the end
	println!("s is {s}");
	
	//Concatenation options
	
	let s1 = String::from("Hello, ");
    let mut s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used, s1 no longer valid!
	                   // + operator is using the add() function
					   //fn add(self, s: &str) -> String {
					   // One extra trick: Compiler can excert &String to &str, no problem
	println!("{s3}");
	s2.push('s');
	println!("{s3}");  // concatenation copies the values!, s3 did not change
	
	let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
	let s = s1 + "-" + &s2 + "-" + &s3;
	println!("{s}");
	
	//Also format can be used
	let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
	let s = format!("{}-{}-{}",s1,s2,s3); //Format uses only references, does not take ownership
	let s = format!("{s1}-{s2}-{s3}");
	
	let s1: String = String::from("hi");
    //let h: String = s1[0]; // Strings are not indexable
	
	//Creating string slice: Returns string slice
	let hello: &str = "Здравствуйте";
	let s: &str =  &hello[0..4]; //String slice "Зд"
	println!("{s}");
	//let s: &str = &hello[0..1]; //Panics! Cannot cut through the first character.
	                            // Be careful when creating string slices
	println!("{s}");
	
	let hello: String = String::from("Здравствуйте");
	//let s: &str = &hello[0..1]; //also Panics, same
	println!("{s}");
	
	//Iterating through characters
	for c in "Зд".chars() { //Chars gives back the chars
        println!("{c}");
    }
	
	for b in "Зд".bytes() { //Bytes gives the raw bytes
        println!("{b}");
    }
	
	//Grapheme clusters? Complex, need to use different crate if needed
}
