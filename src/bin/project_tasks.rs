use std::fs::{self, OpenOptions}; //file system tools
use std::io::{self, Write}; //input/output tools

fn main() {
    let filename = "tasks.txt";

    loop {
        println!("\n--- RUST TASK MANAGER ---");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Exit");
        println!("Enter Choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Enter task:");
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                //save immediately to file
                add_task(filename, task.trim());
            }
            "2" => {
                println!("\n--- YOUR TASKS ---");
                view_tasks(filename);
            }
            "3" => break,
            _ => println!("invalid choice"),
        }
    }
}

fn add_task(filename: &str, task: &str) {
    //OpenOptions lets as APPEND to a file instead of overwriting it
    let mut file = OpenOptions::new()
        .create(true) //create file if it doesn't exist
        .append(true) //add to the bottom
        .open(filename)
        .unwrap();

    //writeln! adds a new line automatically
    writeln!(file, "{}", task).expect("Failed to write to file");
    println!("task saved!");
}

fn view_tasks(filename: &str) {
    //fs::read_to_string loads the whole file into a big String
    match fs::read_to_string(filename) {
        Ok(content) => {
            //.lines() splits the big string by newlines
            for (index, line) in content.lines().enumerate() {
                println!("{}, {}", index + 1, line);
            }
        }
        Err(_) => println!("No tasks found yet!"),
    }
}