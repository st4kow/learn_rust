fn main() {
    println!("--VARIABLES--");
    
    //unmutable valueable
    //let y = 1; // This flags as a warning unused
    let _y = 1; // "_" means it is unused, intentionaly
    //y = 2; //Compile error
    println!("The values of the unused variable is: {_y}");
    
    //mutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    
    //constants (not a variable)
    // const insted of let and type must be annotated!
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // This must be a constant expression
    println!("constatnt: Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
    
    println!("--SHADOWING---");
    
    //Using let again effectively creates a new variable. We can event change the type of it
    let a: u32 = 5;
    println!("Values of a: {a}");
    let a = a + 1; // Used let again
    println!("Values of a: {a}");
    { // This is a new scope
        let a = a + 1;
        println!("Values of a: {a}");
    }
    println!("Values of a: {a}");
    
    //good example
    let spaces = "     ";
    let _spaces = spaces.len(); // Shadowed the tape to u32
    
    let mut _spaces_wrong = "     ";
    //spaces_wrong = spaces_wrong.len(); //Compilation error. Cannot change the type
}
