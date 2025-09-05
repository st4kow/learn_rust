use std::slice;

//Defininf global variable:
static HELLO_WORLD: &str = "Hello World";

//Defining static mutable variable
static mut COUNTER: u32 = 0;

fn main() {
    /*
        Unsfafe rust:
        
        Dereference a raw pointer
        Call an unsafe function or method
        Access or modify a mutable static variable
        Implement an unsafe trait
        Access fields of a union

        Borrow checker still checking references in unsafe code

    */

    /* RAW POINTERS */

    // Raw pointers: inutable / mutable
    // Inmutable means thet a pointer cannot be directly assigned to after beeing dereferenced
    // *const T
    // *mut T

    // * is not the dereference operator. It is part of the type name

    /*
    RAW POINTERS
    Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    Aren’t guaranteed to point to valid memory
    Are allowed to be null
    Don’t implement any automatic cleanup

    */

    let mut num : i32 = 5;

    // Creating raw pointers is allowed in safe code

    let r1 : *const i32 = &raw const num;
    let r2 : *mut i32   = &raw mut num;

    unsafe {
        println!("r1 address is {}, value is {}", r1 as u32, *r1); //dereferencing r1
        println!("r2 address is {}, value is {}", r2 as u32, *r2); //dereferencing r2
    }

    // Pointing to an undefined memory place

    let address = 0x044usize;
    let r = address as *const i32; // Raw pointer to the address
    unsafe {
        if false {  //for test
            println!("random address is {}, value is {}", r as u32, *r); //dereferencing r1
        }
    }

    /* CALLING UNSAFE FUNCTIONS/METHODS */

    unsafe fn dangerous() { /* function code here */ }
    
    // calling unsafe function
    unsafe {
        dangerous();
    }

    /* CREATING SAFE ABSTRACTIONS AROUND UNSAFE CODE */

    //Example code, splitting a vector
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = split_at_mut(r,3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // This cannot be implemented by only safe code!. Split_at_mut has unsafe inside

    /* CALLING EXTERBNAL CODE */

    // Foreign Function Interface - FFI - "C"

    println!("Absolute value of -3 according to C: {}", abs(-3));

    
    //About global variables.
    println!("Printing global variable: {HELLO_WORLD}");

    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER)); //mutable static variables can be access only by raw pointers
    }
}

//Custom implementetion as a function
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr: *mut i32 = values.as_mut_ptr(); // get raw pointer to the values

    assert!(mid <= len);

    
    unsafe {
            // slices are pointers to the data and the size of the data
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len-mid)
        )
    }

    //(&mut values[..mid], &mut values[mid..]) // does not work because we would have two mutable references to the save vector

}

unsafe extern "C" {  // external functions with "C" style FFI, ABI
    safe fn abs(input: i32) -> i32; // we can say explicity that it can be called from a safe block "safe" becasuse it uses only an i32
}

//creating a function in rust that can be called from an other language
#[unsafe(no_mangle)] //no mange means compiler cannot change the name of this function, we need to be careful with dupliated names because of this
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Implementing unsafe traits
unsafe trait foounsafe {
    // method goes here
}
unsafe impl foounsafe for i32 {
    // Method implementatiob goes here
}