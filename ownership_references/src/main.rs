fn main() {
    let mut s1 = String::from("hello");
	let len = calculate_length(&s1);
	println!("Size of {s1} is {len}");
	
	//change(&s1); // does not work because reference needs to be mutable
	change(&mut s1);
	println!("The changes string: {s1}");
	
	let mut s1 = String::from("hello");
	let s1_ref_1 = &s1;
	let s1_ref_2 = &s1; // variable can have multiple unmutable references pointing to it
	//let s1_mut_ref = &mut s1; // Does not work. Variable cannot have mutable and unmutable references at the same time
	                            // if the references are used in the future
	println!("{s1_ref_1},{s1_ref_2}");
	
	let mut s1 = String::from("hello");
	let r1 = &s1;
	let r2 = &s1;
	let r3 = &mut s1; // This works, becasue r1, r2 never used at the future
	println!("{s1}");
	
	let mut s1 = String::from("hello");
	let s1_mut_ref_1 = &mut s1;
	//let s1_mut_ref_2 = &mut s1;   // Does not work. Variable cannot have multiple unmutable references at the same time
	//println!("{s1_mut_ref_1},{s1_mut_ref_2}");
	
	let mut s1 = String::from("hello");
	{
	    let s1_ref: &mut String = &mut s1;
	}
	let s1_ref_2: &mut String = &mut s1; // This works, because s1_ref went out of scope
	println!("{s1_ref_2}");
	
	println!("Testing dangling references");
//	let s = dangle(); // Does not work becasue the function is wrong
	
}
fn calculate_length(s: &String) -> usize { // s is a reference to a String 
	s.len()
} // Here s goes out of scope, but does not have ownership, string is not freed, valid
//fn change(s: &String) {   // Does not work, because need to be mutable reference
//	s.push_str(", world");
//}
fn change(s: &mut String) {
	s.push_str(", world");
}
//fn dangle() -> &String {  // Does not work, function gives back reference that point to a variable which is not valid (lifetime)
//	let s = String::from("hello");
//	s
//}