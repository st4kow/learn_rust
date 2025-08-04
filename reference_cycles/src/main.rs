use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // Parent is a weak connection
    children: RefCell<Vec<Rc<Node>>> // Pointing to children is the strong connection


}

fn main() {
    // Reating reference cycles for test

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons((10), (RefCell::new(Rc::clone(&a)))));
    //Now b point to a

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    //Now a points to b

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));


    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());

    // Weak<T> smart pointer
    // Weak references do not express ownership relations

    let leaf = Rc::new(Node { // leaf has value 3, and have no children
        value: 3,
        parent: RefCell::new(Weak::new()), //Points nowhere now
        children: RefCell::new(vec![])
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    {
        let branch = Rc::new(Node { //branch has value 5, have one children - the leaf
            value: 5,
            parent: RefCell::new(Weak::new()), //Points nowhere now
            children: RefCell::new(vec![Rc::clone(&leaf)])
        });

        //Set breanch to be a parent of leaf
        *(leaf.parent.borrow_mut()) = Rc::downgrade(&branch);
        // Rc:downgrade sets this connectiion and increases only weak reference counter
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    //Now the first Node has two owners: leaf and branch
    // How do we add cycles? Think of one direction as a strong connection, that represents ownership
    // Think of ther direction as a weak connection that does not represent ownership

}
