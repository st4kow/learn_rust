use std::io;

fn main() {
    println!("--integer types---");
    let mut int : i8 = 100; // signed 8bit
    loop {
        println!("Actual value: {int}");
        int = int + 1;
        println!("Next value value: {int}");
        break; // comment this to test what happens at overflow
    }
    
    let _int : u8  = 1; // unsigned 8 bit
    let _int : i16 = 1; // signed 16bit
    let _int : u16 = 1; // unsigned 16 bit
    let _int : i32 = 1; // signed 32 bit
    let _int : u32 = 1; // unsigned 32 bit
    let _int : i64 = 1; // signed 64 bit
    let _int : u64 = 1; // unsigned 64 bit
    let _int : isize = 1; // architecture dependent 32 / 64 bit signed
    let _int : usize = 1; // architecture dependent 32 / 64 bit unsigned
    
    println!("--testing integer literals--"); // IEEE-754
    let mut _int : i32 = 1234;
    println!("liternal test: {_int}");
    _int = 1_234; //decimal nice
    println!("liternal test: {_int}");
    _int = 0xFF; //hexa
    println!("liternal test: {_int}");
    _int = 0xFF_AA_FF; //hexa niccce
    println!("liternal test: {_int}");
    _int = 0o7; //octal
    println!("liternal test: {_int}");
    _int = 0b1111_0000_1111; // binary nice
    println!("liternal test: {_int}");
    
    println!("--Testing floating point numbers--");
    println!("Should print: 1.12345678987654321");
    let _float : f32  = 1.12345678987654321;
    println!("Float test: {_float}");
    let _double : f64 = 1.12345678987654321;
    println!("Double test: {_double}");
    let _default = 1.12345678987654321;
    println!("Default precision: {_default}"); // Default if f64
    
    
    println!("--Numerical operations---");
    let _sum = 1 + 2;
    let _diff = 2 - 1;
    let _product = 2 * 2;
    let _quotient = 54.23 / 23.43; // Divide floats!
    let _truncated = -5 / 3; //Divide ints!
    let _rema = 5 % 2;
    
    println!("--Boolean type--");
    // one byte size
    let _flag = true;
    let _flag : bool = false;
    
    println!("--Character type--");
    // Char is four bites in size. Unicode scalar value
    // from UTF U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive
    let _z = 'z';
    //let _Z : char = 'Z'; // compiler tells you that this should not be uppec case
    let _zbig : char = 'Z';
    let _hearth_eyed_cat : char = '😻';
    println!("These are the chars: {},{},{}",_z,_zbig,_hearth_eyed_cat);
    
    println!(".......");
    println!("--Compound types--");
    println!(".......");
    
    println!("--Tupple--");
    // way of grouping together variety of types
    // Fix, cannot grow or shring
    
    let _tup = (500, 1.234, 'Z');
    let _tup: (u32, f64, char) = (500, 1.234, 'Z');
    let (_x, _y, _z) = _tup; // destructuring
    //let (_x: u32, _y: f64, _z: char) = _tup; // This does not work.. How to do this than?
    println!("Tupple test: {},{},{}",_x,_y,_z);
    //access elements piece-by-piece
    let _zero   = _tup.0;
    let _first  = _tup.1;
    let _second = _tup.2;
    println!("Tupple test: {},{},{}",_zero,_first,_second);
    
    let _unit = (); // Empty tuple is the unit value
    // Expressions implicitly return this value they return nothing
    
    println!("--Arrays--");
    //Every element must have the same type
    //In rust, arrays have fixed length
    //Good, when we want data to be collected on the stack, not the heap
    //Vector can grow or shrink in size
    let _a = [1, 2, 3, 4, 5];
    let _a: [i32; 5] = [1, 2, 3, 4, 5]; // Specify type than number of elements
    let _nums: [i32; 5] = [1; 5]; // equivalent to [1, 1, 1, 1, 1]
    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    //println!("{_months}"); //Cannot be formatted with the default formatter
    println!("Accessign array elements...");
    let _first = _a[0];
    let _second = _a[1];
    println!("First: {_first}, Second: {_second}");
    
    println!("Let's try accessing non existing array index");
    let be_silly: bool = true;
    if be_silly {
        println!("We are silly");
        println!("Please enter an index to reach in the array.");
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Error reading input");
        let index: usize = index
            .trim()
            .parse()
            .expect("Could not identify as a number");
        let element = _a[index];
        println!("Index was: {element} and the value there is: {}",_a[index]);
    } else {
        println!("be_silly was set to: {be_silly}, so did not panic");
    }
}