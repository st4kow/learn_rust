use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Interior Mutability design pattern
    // This allows mutating data even when there are only unutable ferences to it
    // There is underlying unsafe code that allow this
    // We need to guarantee that we are following borrowing rules runtime. If not, panic.

    // RefCell<T> represents single ownership, similar to Box<T>
    // BUT RefCell do not need to satisfy borrowing rules during compile time
    // Only for single thereaded applications

    //Example: 
    let x = 5;
    // let y = &mut x; //Does notwork, because x was not declared as mutable.

    //*********************************/
    // Allowing Multiple Owners of Mutable Data with Rc<T> and RefCell<T>

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons((Rc::clone(&value)), (Rc::new(Nil))));
    let b = Cons((Rc::new(RefCell::new(3))), (Rc::clone(&a)));
    let c = Cons((Rc::new(RefCell::new(4))), (Rc::clone(&a)));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}
