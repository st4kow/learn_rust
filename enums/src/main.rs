enum IpAddrKind { //Camel case? Not clear if this is intentional
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

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
    
    //TODO
}

fn route(ip_kind: IpAddrKind) {}