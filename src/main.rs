use std::io;

fn main() {
    println!("Hello Welcome to this TODO program");

    loop {
        println!("Input function: ADD / Print / Delete / quit");

        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("failed to read line");
        let trimmed = name.trim().to_lowercase();

        if trimmed == "add" || trimmed == "a"{
            println!("Add works")
        } else if trimmed == "Print" || trimmed == "p" {
            println!("print works")
        } else if trimmed == "delete" || trimmed == "d" {
            println!("delete works")
        } else if trimmed == "quit" || trimmed == "q" {
            println!("Thank you!");
            break
        }

    }

}
