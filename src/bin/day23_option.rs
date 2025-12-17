fn find_student(id: u32) -> Option<String> {
    if id == 1 {
        return Some("Rahim".to_string());
    } else if id == 2 {
        return Some("Inarah".to_string());
    } else {
        return None;
    }
}

fn main() {
    let student_box = find_student(1);
    check_locker(student_box);

    let missing_box = find_student(99);
    check_locker(missing_box);
}

fn check_locker(locker: Option<String>) {
    match locker {
        Some(name) => println!("Found Student: {}", name),
        None => println!("Student not found in database.")
    }
}