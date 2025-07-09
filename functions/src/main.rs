fn main() {
    println!("Hello, this is the main function");
    another_function(5);
    
    println!("Lets mesure voltages: ");
    print_labeled_measurement(1,'m');
    print_labeled_measurement(2,'M');
    
    println!("Lets play with statements and expressions");
    let mut x = 5; // This is a statement, there is no return value
    x = x + 1; // This is an expression
    x = x - 1;
    //let y = (let x = 5 ); // Compilation error (would work in Ruby or C for example)
    
    let y = { // This works! Calling a block is an expression (similar to 1+2 is an expression)
        let x = 3;
        x + 1 // NO semicolon here! this is the return value
    };
    println!("Result of calling the block: {y}");
    
    println!("Calling a function that return a number...");
    x = six();
    println!("The returned value: {x}");
    x = plus_one(x);
    println!("Added one to the result: {x}");
    
    
}

fn another_function (x: i32) { // MUST declare the type of the variable!
    println!("Hello, this is the another_function");
    println!("The number in the argument is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
fn six() -> i32 {
    6
    // or return 6
    // or return 6;
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
//Statement: Instructions that perfor some action and do not return anything
//Expression: Evaluate to a resultant value
