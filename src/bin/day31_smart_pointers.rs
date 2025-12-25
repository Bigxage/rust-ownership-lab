// a 'recursive' type
// a list can contain .... another list
// rust can't know the size of this at compile time, so we must
// wrap it in box (pointer)
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    //creating a linked list: 1 -> 2 -> 3 -> Nil
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("created a recursive linked list using box!");
}