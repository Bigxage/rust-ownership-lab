use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    //1. create a channel (tx = transmitter, rx = receiver)
    let (tx, rx) = mpsc::channel();

    //2. spawn a thread to send messages
    thread::spawn(move || {
        let val = String::from("hello, from the other side");
        //send() returns a result. if the receiver is dead, it fails.
        tx.send(val).unwrap();
        println!("message sent!");
    });

    //3. main thread receives the message
    //recv() blocks execution until a message arrives.
    let received = rx.recv().unwrap();
    println!("main thread got: {}", received);
}