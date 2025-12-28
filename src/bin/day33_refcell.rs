use std::cell::RefCell;

fn main() {
    //1. 'data' is technically immutable (no 'mut' keyword)
    let data = RefCell::new(10);

    println!("original: {:?}", data);

    //2. we borrow it mutably using .borrow_mut()
    //this would panic at runtime if we had another active borrow!
    *data.borrow_mut() += 20;

    println!("updated (interior mutability): {:?}", data);
}