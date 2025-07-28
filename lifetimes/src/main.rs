use std::fmt::Display;

fn main() {
    // Every reference has a lifetime
    // This is the scope where the reference is valid
    
    //Lifetimes prevent dangling references
    
    
    /*
    
    let r; // r is in the outer scope
    {
        let x = 5; // x is defined in inner scope
        r = &x; // r gets value in the inner scope
    } // x deallocated here!
    // r would be invalid (dangling) here, becasue x oes not exists
    println!("r: {r}"); // Using r
    //Error: Boorwed value does not live long enough
    
    */
    
    let x = 5;
    let r = &x;
    println!("r: {r}");
    
    //Generic Lifetimes in Functions
    
    let string1: String = String::from("abcd"); //String
    let string2: &str    = "xyz"; //String slice
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");  
    
    let string1: String = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str()); // Good, bot x and y are valid
        println!("The longest string is {result}");
    } // result valid up to this point, because y gets invalid here
 
    /*
    let string1: String = String::from("long string is long");
    let result: &str;
    {
        let string2: String = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    } //result goes out of scope, because string2 goes out of scope
    println!("The longest string is {result}"); // Does not work
    */
    
    // Structs //
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Content of ImportantExcerpt: {}", i.part );
    
    // Static lifetimes //
    let s: &'static str = "I have a static lifetime.";
    //static means it can live the entire duration of the program.
    //All string literals have the 'static lifetime
}
/*
fn longest(x: &str, y: &str) -> &str { // Does not work. Returning reference have problems
    if x.len() > y.len() { x } else { y }
}
*/

/*
  Problem: Rust can't tell wheather the reference beeing return defers to x or y.
  Really a problem, because based on the if else, if refers to x or y.
  We do not know it either
  
  -> Need to help the borrow checker
*/

/* Syntax for lifetime parameters
  &i32         // Rereference for i32
  &'a i32      // Reference with an explicit lifetime for i32
  &'a mut i32  // Mutable ference to i32 with explicit lifetime
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Here the output is valid until both x and y refereces are valid
    if x.len() > y.len() { x } else { y }
}

// Lifetimes need to be specified based on what the function is doing
fn returns_first<'a>(x: &'a str, y: &str) -> &'a str {
    x // Returns x, do nothing to do with y
}

/*
fn longest_1<'a>(x: &str, y: &str) -> &'a str { // Error, return lifetime needs to match at least on of the inputs lifetime
    let result: String = String::from("really long string");
    result.as_str()
}
*/

// Lifetime annotations in struct definitions
struct ImportantExcerpt<'a> {
    part: &'a str  // part lives untion the struct lives
}
// Implementing method on structs with lifetime
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { // we are not referencing anything, no need for lefetime
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str { // Here third elision rule (&self) applies, no need for lifetimes
        println!("Attention please: {announcement}");
        self.part
    }
}

// Lifetime Elision
fn first_word(s: &str) -> &str { // Lifetimes not defined here. No need, it is trivial
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Generic type parameters, Trait Bounds, and lifetimes together

fn longest_with_an_announcement<T, 'a> (
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where
    T: Display
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}


