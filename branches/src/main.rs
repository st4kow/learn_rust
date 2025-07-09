fn main() {
    println!("-- Hello, BRANCHES -- ");
    let mut a = 4;
    if a < 3 {
        println!("{a} is less than 3");
    } else {
        println!("{a} not less than 3");
    }
    
    if a != 0 {
        println!("{a} is not equal 0");
    }
    
    if a % 4 == 0 {
        println!("{a} is divisable by 4"); // "arms"
    } else if a % 3 == 0 {
        println!("{a} is divisable by 3");
    } else if a % 2 == 0 {
        println!("{a} is divisable by 2");
    }
    
    println!("-- Using if as an EXPRESSION--");
    let condition = true;
    let number = if condition { 5 } else { 6 }; // now if returns these numbers
    //let number = if condition { 5 } else { "six" }; // does not work because of the types
    println!("The value of the number variable: {number}");
    
    println!("-- Loops --");
    println!("***********");
    // Three types : loop, while, for
    let mut x = 0;
    let mut loopret;
    loopret = loop {
        if x == 10 {
            break x * 2 // loop returns with 2*counter
        }; 
        println!("x = {x}, Again!!");
        x = x + 1;
    }; // This makes this an expression
    
    println!("Loop returned with: {loopret}");
    
    println!("Testing labeling of loops to break any if they are nested.");
    'test_label: loop {
        break 'test_label
    }
    
    println!("--Testing while loops--");
    let mut x = 3;
    while x != 0 {
        println!("{x}");
        x -= 1;
    }
    println!("Done");
    
    println!("Printing an array!");
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("at {index} we have {}",arr[index] );
        index += 1;
    }
    // This is not just bad, bad also slow, because rust adds runtime code
    // To catch if we are out of bounds
    
    println!("Trying for loops");
    let arr: [i32; 5] = [100, 200, 300, 400, 500];
    for element in arr {
        println!("The element is {element}");
    }
    //For is safe and fast!
    
    println!("Counting down to 3 using for loops and Ranges");
    for element in (1..3).rev() {  //.rev() reverses ranges
        println!("{element}");
    }
    
    let mut fib = fib_rec(10);
    println!("Fibonacci: {fib}");
    fib = fib_while(10);
    println!("Fibonacci: {fib}");
    let limit = 32;
    println!("Writing fibonacci numbers up to {limit}");
    let mut count = 1;
    while count <= 32 {
        fib = fib_while(count);
        println!("#{count}. fibonacci number: {fib}");
        count += 1;
    }
}

fn fib_rec(x: u32) -> u32 {
    if x == 1 { return x; }
    x + fib_rec(x - 1)
}
fn fib_while(x: u32) -> u32 {
    let mut num = x;
    let mut sum = 0;
    while num > 0 {
        sum += num;
        num -= 1;
    }
    sum
}
