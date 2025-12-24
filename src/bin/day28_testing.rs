// the function we want to test
fn add_two(a: i32) -> i32 {
    a + 2
}

fn main() {
    println!("to run tests, use the command: 'cargo test --bin day 28_testing");
}

// the test module (runs only when testing)
#[cfg(test)]
mod tests {
    use super::*; // import things from the main code above

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4); //passes if 2+2 = 4
    }

    #[test]
    fn it_check_math(){
        let result = 2 + 2;
        assert_ne!(result, 5); // passes if result is not 5
    }
}