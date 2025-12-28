use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, //weak reference to parent to avoid cycle
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    //1. create a leaf node
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    //2. Create a branch node
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    //3. make the leaf point back to the branch (parent)
    //we downgrade the strong Rc to a weak reference
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf strong count: {}", Rc::strong_count(&leaf));
    println!("branch strong count: {}", Rc::strong_count(&branch));
}