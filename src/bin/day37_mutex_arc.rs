use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    //1. create the shared state
    //mutex protects the data (0)
    //arc allows multiple threads to own the mutex
    let counter = Arc::new(Mutex::new(0));

    //we need a place to store the thread handles so we can join them later
    let mut handles = vec![];

    //2. spawn 10 threads
    for _ in 0..10 {
        //clone the arc pointer
        //we are not copying the data, just creating a new key to the same room
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            //3. lock the mutex
            //.lock() asks: "is the room empty?"
            //if yes: walk in and lock the door.
            //if no: wait here until it opens
            let mut num = counter_clone.lock().unwrap();

            //4. mutate the data
            *num += 1;

            //when 'num' goes out of scope, the lock releases AUTOMATICALLY.
        });

        handles.push(handle);
    }

    //5. wait for everyone to finish
    for handle in handles {
        handle.join().unwrap();
    }

    //6. print the final result
    println!("result: {}", *counter.lock().unwrap());
}