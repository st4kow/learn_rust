fn main() {
    // Iterators are Lazy. They do not do anything until they are used
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    //Reseting the iterator:
    let mut v1_iter = v1.iter(); // Need to be mut, becasue next changes it
    //Testing next function
    assert_eq!(v1_iter.next(), Some(&1) );
    assert_eq!(v1_iter.next(), Some(&2) );
    assert_eq!(v1_iter.next(), Some(&3) );
    assert_eq!(v1_iter.next(), None     );

    // .next() gives inmutable/mutabler ferences or owned types
    // based on .iter() / .iter_mut() / .into_iter()
    
    //CONSUMING ADAPTER. Methods, that consume the iterator
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); // This consumes the iterator
    assert_eq!(total, 6);
    // v1_iter.count(); // not working. Ownership was consumed

    println!("Testing iterator adapters");
    //ITERATOR ADAPTERS. Method, that generates iterators
    let v1_iter =  v1.iter().map(|x| x+1 ); // map maps a closure to an iterator.
    // At this point nothing happened! Iterators a Lazy.
    let v2: Vec<_> = v1_iter.collect(); //collect generates a nev vector and applies the iterator to each element
    for item in v2.iter() {
        println!("Got: {item}");
    }
}

// This is how the Iterator trait looks like:
// Only the next function needs to be implemented
/*
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided

 */
