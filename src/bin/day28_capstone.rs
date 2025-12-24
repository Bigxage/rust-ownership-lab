// a generic struct that can hold any type
struct Storage<T> {
    item: T,
}

//implementation for the storage
impl<T> Storage<T> {
    fn new(item: T) -> Self {
        Storage { item }
    }

    fn get_item(&self) -> &T {
        &self.item
    }
}

// special implementation only if T implements the Display trait (so we can print it)
use std::fmt::Display;

impl<T: Display> Storage<T> {
    fn print_status(&self) {
        println!("Storage contains: {}", self.item); 
    }
}

fn main() {
    //1. store an integer
    let int_storage = Storage::new(500);
    int_storage.print_status();

    // 2. store a string
    let str_storage = Storage::new(String::from("Solana Blueprint"));
    str_storage.print_status();
}