use std::io;

fn main() {
    println!("Hello Welcome to this TODO program");


    loop {
        println!("Input function: ADD / Print / Delete / quit");

        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("failed to read line");
        let command = name.trim().to_lowercase();

        match command.as_str() { // .as_str() lets us match on string slices
            "add" | "a" => println!("Add works"),
            "print" | "p" => println!("print works"),
            "delete" | "d" => println!("delete works"),
            "quit" | "q" => {
                println!("Thank you!");
                break;
            }
            _ => println!("unknown request made"), // The underscore `_` is the catch-all pattern
        }

    }

}
