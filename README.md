# todo-cli

A simple, command-line Todo List application written in Rust.  
This was my first Rust project, built to learn the language's fundamentals by creating something practical.

---

## ✨ Features

- **Add tasks:** Add new todo items to your list.
- **List tasks:** View all your current tasks.
- **Delete tasks:** Remove tasks by their index.
- **Persistent storage:** Your todo list is automatically saved to a JSON file and reloaded when you start the program.
- **Simple and fast:** A no-fuss, keyboard-driven interface.

---

## ⚙️ Installation

1. **Ensure you have Rust and Cargo installed.**  
   Follow the instructions on [rust-lang.org](https://www.rust-lang.org/tools/install).

2. **Clone and build the project:**
   ```bash
   git clone <https://github.com/Tx0sh1/Rtodo>
   cd todo-cli
   cargo build --release
   ```

3. **Run the executable:**
   ```bash
   ./target/release/todo-cli
   ```

---

## 🚀 Usage

Run the program and follow the interactive prompts:

```text
$ cargo run
Hello Welcome to this TODO program
Input function: ADD / Print / Delete / quit
> add
Enter the item you want to add
> Learn Rust ownership
Input function: ADD / Print / Delete / quit
> print
here are your items: ["Learn Rust ownership"]
Input function: ADD / Print / Delete / quit
> quit
Thank you for playing the game!
```

### Commands
- `add` or `a` → Add a new task
- `print` or `p` → List all tasks
- `delete` or `d` → Delete a task by its number
- `quit` or `q` → Exit the program

---

## 🛠 Technical Details

This project was a learning exercise for core Rust concepts:

- **Ownership & Borrowing** → Managing data safely between the main loop and functions
- **Error Handling** → Using `Result` and `Option` for robust user input parsing and file operations
- **Data Serialization** → Using [`serde`](https://serde.rs/) and `serde_json` to persist the todo list to a file
- **Structured Data** → Leveraging `struct` and `enum` to model the application's domain

---

## 📚 Learning Journey

This project was built step-by-step, focusing on one concept at a time:

1. Basic command parsing and loop structure
2. In-memory storage with `Vec<String>`
3. Implementing a structured `TodoItem` with completion status
4. Adding JSON file persistence
5. Refining error handling and user experience

---

## 🦀 Built With

- [Rust](https://www.rust-lang.org/) — as a personal learning milestone.  
