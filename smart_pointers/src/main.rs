use crate::List::{Cons, Nil};
use std::ops::Deref; // Needed to implement deref operator on MyType

fn hello(name: &str) {
    println!("Hello, {name}"); // Here, there is no auto deref. println expect string slice
}
fn main() {
    // Box<T>  storing smthing on the heap, storing pointer to it on stack
    /*
      When to use?
      1, Types where the size is not known during compile time
      2, Large amount of data and we want to move ownership without copiing too much
      3, (trait object) We want to own a value, and the important thing is the implemented traints not the acutal content
     */

    let b = Box::new(5); //5 stored on heap, b is a pointer to that
    println!("b = {b}"); //Automatic derefencing
    //When b goes out of scope the data will be autmatically deallocated from memory
    // Because they are implementing the Drop trait

    //-- Recursive types --//

    // cons list data structure, came from Lisp
    // This is a list style linked list
    // Example for 3 elements: (1 ,(2, (3, Nil)))

    //let list = Cons(1, Cons(2, Cons(3, Nil))); // does not work, would need infinite mamory
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    //****************************************************************//
    //Treating smart poiters like references with Deref
    // Implementing the deref trait allows customization of * operator

    let x = 5;
    let y = &x; // Reference also behaves as a pointer
    assert_eq!(5, x);
    assert_eq!(5, *y); // dereferencing y
    //assert_eq!(5, y); // derec coercion works only between references. this is by need to do this by hand
    //assert_eq expect values, not references

    let x = 5;
    let y = Box::new(x); // Creating smart pointer, value of x is copied to the heap
    assert_eq!(5, x);
    assert_eq!(5, *y); // Treat it the same way
    //assert_eq!(5, y);

    //Now we try to define out own smart pointer for illustration
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // here *y is transleted to *(y.deref())
    // This way it is equavelent to a simple reference
    // Actually * on a type convert is to a deref() call that give back 
    // a reference, and puts a * before it

    //---- Implicit deref coercions ------
    // Deref changes a reference of a type to an other type of reference
    // Deref happens automatically during function call when the type does not
    // Matches the argument type

    let m = MyBox::new(String::from("Rust")); // MyBox type
    hello(&m); // Here we dereferencing the box, so function will get a String.
    // Automatic dereferencing works only between reference!
    // So for example String cannot be converted to &str automatically
    // 
    hello(&((*m)[..]));  //This would be the call without deref coercions
    //Dere coercion is automatically applyed as many time as needed!

    //DerefMut trait is a completely different trait for mutable references 

    // Three cases how the deref coercion works:
    // 1, From &T to &U when T: Deref<Target=U>
    // 2, From &mut T to &mut U when T: DerefMut<Target=U>
    // 3, From &mut T to &U when T: Deref<Target=U>

    //---------About the Drop trait----------//

    let c = CustomSmartPointer {
        data: String::from("my stuff")
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff")
    };
    println!("CustomSmartPointers created");
    drop(c); // This is not allowed by default, handled automatically
    println!("CustomSmartPointer dropped before the end of main.")
}

enum List {
    Cons(i32, Box<List>),
    Nil
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
/*
  How the needed mamory os a Message enum is calulated?
   - Needed space is calculated for each variant
   - The highest number is used to determine the needed space
   IMPORTANT
   : It does not matter if I create a Quit or a ChangeColor,
   both will use the same space on memory
 */

 //MyBox will behave the samu, but stores everything on stack
 struct MyBox<T>(T); //tuple struct
 impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
 }
 impl<T> Deref for MyBox<T> { // Implementing derefererence operator
    type Target = T; // This is an associated type
    fn deref(&self) -> &Self::Target {
        &self.0
    }
 }

 // Drop trait - used as a destructor in most cases
 // Automatically callsed drop() when goes out of scope
struct CustomSmartPointer {
    data: String
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}",self.data);
    }
}