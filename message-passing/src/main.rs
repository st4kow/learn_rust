use std::sync::mpsc;
/* mpsc: multiple producer, single consumer
This type can have multiple sending ends, one consumer end
*/

use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel(); // This returns a tuple
    /* Now both tx, rx are attached to the main thread */
    let tx1 = tx.clone(); // create and other tx end

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("From"),
            String::from("The"),
            String::from("Thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        //println!("Send value is: {val}"); // Does not work! Lost the value
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("More"),
            String::from("Messages"),
            String::from("For"),
            String::from("You"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    //let received = rx.recv().unwrap(); /* receiving the message */
    // THIS BLOCKS THE MAIN THREAD! .try_recv() does not, usually used in a loop polling

    for received in rx { /* Treating rx as an iterator */
        println!("Got: {received}");
    }
}
