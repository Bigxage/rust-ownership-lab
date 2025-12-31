use std::thread;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    //experiment 1: the crash test (uncomment to see error)

    //let unsafe_pointer = Rc::new(5);

    //thread::spawn(move || {
    //     println!("this will not compile: {:?}", unsafe_pointer)
    // });

    //error: 'Rc<i32>' cannot be sent between threads safely.
    //the trait 'send' is not implemented for 'Rc<i32>'.

    //experiment 2: the safe way (arc)

    let safe_pointer = Arc::new(10);

    let handle = thread::spawn(move || {
        println!("safe thread says: {}", safe_pointer);
    });

    handle.join().unwrap();
    println!("main thread says: success.");
}