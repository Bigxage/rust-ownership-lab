use std::fmt::Debug;

fn print_anything<T: Debug>(value: T) {
    println!("The value is: {:?}", value);
}

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let my_number = 42;
    let my_word = "Solana";
    let my_float = 3.14;

    let my_user = User {
        name: "Rahim".to_string(),
        age: 25,
    };

    print_anything(my_number);
    print_anything(my_word);
    print_anything(my_float);
    print_anything(my_user);
}