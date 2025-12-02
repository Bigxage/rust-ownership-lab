fn main(){
    let status = String::from("Pending");

match status.as_str() {
    "Success" => println!("Transfer complete"),
    "Failed" => println!("Transfer failed"),
    "Pending" => println!("Please wait..."),
    _ => println!("Unknown status"),
    }
}