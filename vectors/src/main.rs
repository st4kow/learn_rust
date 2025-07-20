fn main() {
    // Vector: Data next to each other in memory
    let v: Vec<i32> = Vec::new();
    
    //It is implemented by generics: <T>
    
    //Shortcut macro
    let v = vec![1, 2, 3]; // This is an i32 vector
    
    let mut v = Vec::new(); // Compiler knows this is <i32> because we are pushing that later
    //Adding things to the vector
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    let third: &i32 = &v[2]; // Getting a reference to the third data
    // Thic can PAIC!!
    // &v[x] Gives a reference to a x'th element
    println!("The third element is {third}"); 
    
    let third: Option<&i32> = v.get(2); //Getting the third value, safe
    match third {
        Some(third) => println!("The third eWlement is {third}"),
        None => println!("There is no third element")
    }
    let tenth: Option<&i32> = v.get(9);
    match tenth {
        Some(tenth) => println!("The tenth element is {tenth}"),
        None => println!("Teher is no tenth element")
    }
    let fourth: &i32 = &v[3];
    
    v.push(9);
    //  println!("The third element is{fourth}"); // Using the reference to the fourth
    /*
     DOES NOT work because borrowing checker
     Why? Why adding a new element would interfere with an already existing element?
     Because adding new elmeents might require new memory allocation for the whole vector
     (Vectors stores everything next to each other)
    */
    
}


