// we define a lifetime 'a
// this reads: "if x lives for 'a, the result also lives for 'a."
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("Solana");
    let result;
    {
        let string2 = String::from("Ethereum");
        // this works because both strings are alive here.
        result = longest(string1.as_str(), string2.as_str());
        println!("the longest chain is: {}", result);
    }
    // if we tried to use 'result' here, it would fail because string2 is dead
}