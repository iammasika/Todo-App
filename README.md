This implementation includes:

A Task struct to store task description and completion status

A TodoList struct to manage the list of tasks

Basic operations:

Add new tasks

Mark tasks as done

Delete tasks

List all tasks

Simple error handling for invalid commands

User-friendly interface with commands:

add <task> - Add a new task

done <num> - Mark a task as done

delete <num> - Delete a task

list - Show all tasks

exit - Quit the program

To use the program:

Save it as todo.rs

Compile with rustc todo.rs

Run the executable (./todo on Unix-like systems)

The program uses 1-based indexing for tasks (like most todo lists people are familiar with) and shows tasks with checkboxes ([x] for completed, [ ] for incomplete). All data is stored in memory and will be lost when the program exits.

Example usage:
> add Buy milk
Task added
> add Learn Rust
Task added
> list
1. [ ] Buy milk
2. [ ] Learn Rust
> done 1
Task marked as done
> list
1. [x] Buy milk
2. [ ] Learn Rust
> delete 2
Task deleted
> list
1. [x] Buy milk
