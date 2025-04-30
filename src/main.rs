use std::io::{self, Write};

#[derive(Debug)]
struct Task {
    description: String,
    done: bool,
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        self.tasks.push(Task {
            description,
            done: false,
        });
    }

    fn mark_done(&mut self, index: usize) -> Result<(), String> {
        match self.tasks.get_mut(index) {
            Some(task) => {
                task.done = true;
                Ok(())
            }
            None => Err(format!("Invalid task index: {}", index + 1)),
        }
    }

    fn delete_task(&mut self, index: usize) -> Result<(), String> {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err(format!("Invalid task index: {}", index + 1))
        }
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks in the list!");
            return;
        }

        for (i, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "[x]" } else { "[ ]" };
            println!("{}. {} {}", i + 1, status, task.description);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    
    println!("Simple Todo List");
    println!("Commands: add <task>, done <num>, delete <num>, list, exit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0].to_lowercase().as_str() {
            "exit" => break,
            "add" => {
                if parts.len() < 2 {
                    println!("Error: Missing task description");
                    continue;
                }
                let description = parts[1..].join(" ");
                todo_list.add_task(description);
                println!("Task added");
            }
            "done" => {
                if parts.len() < 2 {
                    println!("Error: Missing task number");
                    continue;
                }
                match parts[1].parse::<usize>() {
                    Ok(n) if n > 0 => {
                        if let Err(e) = todo_list.mark_done(n - 1) {
                            println!("Error: {}", e);
                        } else {
                            println!("Task marked as done");
                        }
                    }
                    _ => println!("Error: Invalid task number"),
                }
            }
            "delete" => {
                if parts.len() < 2 {
                    println!("Error: Missing task number");
                    continue;
                }
                match parts[1].parse::<usize>() {
                    Ok(n) if n > 0 => {
                        if let Err(e) = todo_list.delete_task(n - 1) {
                            println!("Error: {}", e);
                        } else {
                            println!("Task deleted");
                        }
                    }
                    _ => println!("Error: Invalid task number"),
                }
            }
            "list" => todo_list.list_tasks(),
            _ => println!("Error: Unknown command"),
        }
    }
}