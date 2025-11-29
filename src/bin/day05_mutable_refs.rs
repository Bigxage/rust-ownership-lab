fn main() {
  let mut my_string = String::from("I am");
  add_ambition(&mut my_string);
  println!("The result is: {}", my_string)
}

fn add_ambition(s: &mut String) {
    s.push_str(" the greatest!");
}