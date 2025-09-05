fn main() {

    // Pattern matching for x

    let x = Some(1i32);
    match x {
        None => None,
        Some(i) => Some(i + 1),
    };

    // if let, else if, and else if let expressions

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age { // Here age is ashedowing the old age veriable
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    //while let loops

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    // for loops
    // "x in y"   x is a pattern
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // let expressions
    // let PATTERN = EXPRESSION;

    let x = 5;
    let (x, y, z) = (1, 2, 3);
    // let (x, y) = (1, 2, 3); // Compile error
    let (x, y, _) = (1, 2, 3); // Ignoring in the pattern

    // Function parameters

    let point = (3, 5);
    print_coordinates(&point);

}

// Here x is a pettern
fn foo(x: i32) {
    // code goes here
}
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}