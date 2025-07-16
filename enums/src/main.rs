#[derive(Debug)]
enum IpAddrKind { //Camel case? Not clear if this is intentional
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum IpAddrEnum { //We can also attach data to enum variants
    V4(String),
    V6(String)
}
//This also describes how enums work internally
// V4, V6 is a function, that constructs the enum type
//also true when no parameter added to it

enum IpAddrComplxEnum { //We can also have different inputs for enum constructor
    V4(u8, u8, u8, u8),
    V6(String)
}
//This is a benefit over structs. structs have fixed data
#[derive(Debug)]
enum Message {
    Quit, // This is a simple enum field, no data associated
    Move { x: i32, y: i32 }, // Has named fields, lika struct
    Write(String), // includes a string data
    ChangeColor(i32, i32, i32)  //includes three i32 numbers
}

//Alternative version based on structs
struct QuitMessage; //unit struct
struct MoveMessage {
    x: i32,
    y: i32
}
struct WriteMessage(String); //Tuple struct
struct ChangeColorMessage(i32, i32, i32); //Tuple struct
//Problem: These are not gruped under a type

impl Message {
    fn call(&self) {
        println!("call functin was called, which is impelemented for the Messag enum!");
    }
}

fn main() {
    //Creating enums
    let four: IpAddrKind = IpAddrKind::V4;
    let six:  IpAddrKind = IpAddrKind::V6;
    
    //Creating ip addresses
    
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
    
    //Creating IpAddr enum with associated data
    let home = IpAddrEnum::V4(String::from("127.0.0.1") );
    let loopback = IpAddrEnum::V6(String::from("::1") );
    
    //Creating complex IpAddr enums
    let home = IpAddrComplxEnum::V4(127, 0, 0, 1);
    let loopback = IpAddrComplxEnum::V6(String::from("::1")); 
    
    //We have implemented a method for the Message enum
    let m = Message::Write(String::from("hello" ) );
    m.call();
    
    println!("Printng simple ip address: {:?}",four);
    println!("Printng Message: {:?}",m);
    
    //Working with the Oprion num
    println!("--The Option enum--");
    //Useful when a value can be something or can be nothing
    //Rust does not have NULL
    let some_number: Option<i32> = Option::Some(5);
    let some_number = Option::Some(5);
    let some_number = Some(5); //This is a short notation, prelude alredy takes cae of Option::
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    
    //Why is this better than using NULL?
    //Complier will not let us do simple operation on it!
    //This makes it safe
    
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    //let sum = x + y; //This does not work!!
    
    //First we need to convert the Option to a number (or type)
    //Before doing any operation! This forces us checking the validity
    
    
  


}

fn route(ip_kind: IpAddrKind) {}