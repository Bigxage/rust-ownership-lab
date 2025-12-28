use std::thread;
use std::time::Duration;

fn main() {
    //1. spawn a new thread (the detached worker)
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("hi, from the spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //2. the main thread keeps working simultaneously
    for i in 1..3 {
        println!("hi, from the main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    //3. wait for the spawned thread to finish before quitting
    handle.join().unwrap();
}