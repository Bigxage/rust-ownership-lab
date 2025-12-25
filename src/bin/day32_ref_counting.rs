use std::rc::Rc;
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>), // changed box to Rc (Reference counted)
    Nil, 
}

fn main() {
    //1. create list 'a' (the shared data)
    //we wrap it in Rc so multiple things can own it.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a: {}", Rc::strong_count(&a));

    //2. create list 'b' that starts with 3, then points to 'a'
    // we use Rc::clone(&a). this doesn't copy data, it just increases the count!
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b: {}", Rc::strong_count(&a));

    {
        //3. create list 'c' that starts with 4, then also points to 'a'
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c: {}", Rc::strong_count(&a));
    }
    // 'c' goes out of scope here. the count should drop.

    println!("count after c goes away: {}", Rc::strong_count(&a));
}