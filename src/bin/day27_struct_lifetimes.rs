// the struct needs a lifetime because it holds a reference (&str)
struct UserProfile<'a> {
    username: &'a str, // this reference must live as long as the struct
    active: bool,
}

fn main() {
    let server_data = String::from("Rahim_Admin_User");

    //we take a slice (reference) from the string
    let first_part = server_data.split('_').next().expect("No name found");

    //we create the struct using that reference
    let user = UserProfile {
        username: first_part,
        active: true,
    };

    println!("User: {} is active: {}", user.username, user.active);
}