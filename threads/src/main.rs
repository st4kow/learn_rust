use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Num {i} from the spawned thread");
            thread::sleep(Duration::from_millis(4));
        }
    });

    for i in 1..10 {
        println!("Num {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    /* Waiting for spwned thread to finish */
    handle.join().unwrap(); // forces main breanch to wait for spawned thread finish


    println!("**********"); println!("Closures and move in threasd context"); println!("**********");

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || { /* move - Need to force taking ownership of local scope variables */
        println!("Here is a vector from an other branch: {v:?}");
    });
    
    //println!("Here is the vactor from main branch: {v:?}"); /* Compile error! v not exist anymore */

    handle.join().unwrap();



    // Here Main thread ends. This shuts down all spawned threads
}
