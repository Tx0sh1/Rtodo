use std::io;

fn main() {
    println!("Hello Welcome to this TODO program");

    let mut todo_items :Vec<String> = Vec::new();

    loop {
        println!("Input function: ADD / Print / Delete / quit");

        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("failed to read line");
        let command = name.trim().to_lowercase();

        match command.as_str() { // .as_str() lets us match on string slices
            "add" | "a" => {
                println!("Enter the items you want to add");

                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input).expect("failed to read line");
                let trimmed_item = user_input.trim().to_string();
                todo_items.push(trimmed_item);
            },
            "print" | "p" => println!("here are your items: {:?}", todo_items),
            "delete" | "d" => println!("delete works"),
            "quit" | "q" => {
                println!("Thank you!");
                break;
            }
            _ => println!("unknown request made"), // The underscore `_` is the catch-all pattern
        }

    }

}
