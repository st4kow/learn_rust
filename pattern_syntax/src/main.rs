enum Plane {
    Color(Color),
    Pattern(i32)
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

struct Point {
    x: i32,
    y: i32,
}

enum MessageText {
    Hello { id: i32 }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


fn main() {

    // Mathing literals

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Mathcing named variables - complication

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // Here y as a new variable introduced! Shadowing old y, Some(y) will match Some(5)
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    // Multiple patterns

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching ranges of values
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // Mathcing character ranges

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    /* Break apart values */

    // Destructuring structs
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; // Destructuring a struct 
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p; // Shorthand notation, but name matches filed name
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {  // Destructuring during pattern matching
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Destructuring enums

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => { // Shorthand notation for struct-like enum, but we can do the same as we did for structs
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
    }

    // Destructuring nested enums

    let plane = Plane::Color(Color::Hsv(1,2,3));

    match plane {
        Plane::Color(Color::Rgb(r, g, b)) => {
            println!("Plane colors to red {r}, green {g}, and blue {b}");
        }
        Plane::Color(Color::Hsv(h, s, v)) => {
            println!("Plane color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }

    // Destructuring structs and tuples - this can be mixed by match expressions

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    /* Ignoring values in pattern */

    // Ignoring entire pattern with _
    foo(2,3); // Check function definition

    // Ignoring parts of a value with nested _ 

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => { // Mathing for anything if it is Some, Some
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    // in a tuple
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    // Ignoring an unused variable with _
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));

    //if let Some(_s) = s { // Compilation error, becasue s ownership is still moved to _s
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");

    // Ignoring reamining parts of a value with ..

    let origin = Point3D { x: 0, y: 0, z: 0 };
    match origin {
        Point3D { x, .. } => println!("x is {x}"), // Mathing only for x
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
    
    /* Compilation error, becasue this is not unambiguous
    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {second}")
        },
    }
    */

    // Extra conditions with match guards

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }


    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"), // behavse as ( 4|5|6) if y because of precedence
        _ => println!("no"),
    }

    // @ bindings
    // @ is creating a variable also tests the content againt a pattern
    
    let msg = MessageText::Hello { id: 5 };

    match msg {
        MessageText::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        MessageText::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        MessageText::Hello { id } => println!("Found some other id: {id}"),
    }

}

fn foo(_: i32, y: i32) { // _ is ignoring the first parameter, benefit keep function compatible with already existing code, or implementing a trait where something is not needed
    println!("This code only uses the y parameter: {y}");
}

