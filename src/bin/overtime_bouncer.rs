fn main ()  {
    let name = String::from("Hacker");
    check_access(&name);
}

fn check_access(s: &String){
    if s == "Bigxage"{
        println!("Welcome, Master.");
    } else {
        println!("Access Denied.");
    }
}