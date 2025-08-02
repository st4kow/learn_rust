use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // For cases when a value has multiple owners
    //Rc<T> - reference counted smart pointer
    // Rc keeps track of the references pointing to it. When the numebr is 0, drop()
    // When it is used in prcatice? 
    //  - Far away parts of a program trying to reach the same value on heap,
    //    and we do not know which part will be the last to use it.
    //    and ONLY is SINGLE THxEADED applications
    //    and ONLY for unmutable references

    /*
      b 
       \
        a    
       /
      c
     */

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Rc::new(Cons(3, Rc::clone(&a)));
    let c = Rc::new(Cons(4, Rc::clone(&a)));

    // Rc::clone() is a special function. This increas the reference counter! 
    // We could use a.clone() insted, but Rc:clone(&a) is the convention here
    // To indicate, that this is not a real cloning, it is close to zero cost and
    // has a special purpose
    

    //Solution would be storing references to List, but than we would need to specify lifetime parameters
    // which would induce that every prat of the list is valid until t  he whole list is valid.
    // We do not want that

    println!("Use print to check if reference counter really increased: ");
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Reference counter of \"a\" after creating a: {}", Rc::strong_count(&a));
    let b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("Reference counter of \"a\" after creating b: {}", Rc::strong_count(&a));
    let c = Rc::new(Cons(4, Rc::clone(&a)));
    println!("Reference counter of \"a\" after creating c: {}", Rc::strong_count(&a));
    drop(c);
    println!("Reference counter of \"a\" after dropping c: {}", Rc::strong_count(&a));
}


