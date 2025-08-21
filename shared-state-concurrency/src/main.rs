use std::sync::Mutex;
use std::thread;
use std::rc::Rc;

fn main() {

    println!("*** Understanding Mutex locking, unlocking ****");
    //mutex: mutual exlusion

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); //lock() used to assec mut value and als lock the mutex
        // lock() fails, if anyone holding the lock panics
        // lock() BLOCKS THE tHREAD
        *num = 6; // setting the value to 6
    } // here num goes out of scope, calling drop(), unlocking the mutex, cannot be forgotten
   
    println!("m = {m:?}");

    println!("*** Mutex from multiple threads ****");

    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 { /* Spawning 10 threads, incrementing counter */
        let counter = Rc::clone( &counter );
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap() );
}
