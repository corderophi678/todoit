#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod load;
mod save;
mod todo;

use std::env;
use std::env::VarError;

use crate::todo::Todo;

const STORAGE: &str = "TODOIT_DIR";

enum Cmd {
    Help,
    Add(String),
    List,
    Finish(usize),
    Unfinish(usize),
    Remove(usize),
    Invalid,
}
fn main() {
    // get the directory in which we're saving our json file
    let storage_file = get_storage_file()
        .expect("You need to set a storage directory (TODOIT_DIR) environment variable.");

    // Read current todolist into memory
    let mut my_todos: Vec<Todo> = match load::load(&storage_file) {
        Ok(v) => v,
        Err(_) => {
            let v: Vec<Todo> = Vec::new();
            v
        }
    };

    let mut command: Cmd = Cmd::Invalid;
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        command = Cmd::Help;
    } else {
        command = match &*args[1] {
            "help" | "h" => Cmd::Help,
            "add" | "a" => Cmd::Add(args[2].clone()),
            "list" | "ls" => Cmd::List,
            "finish" | "x" => {
                let id: usize = args[2].trim().parse().expect("Invalid id entered.");
                Cmd::Finish(id)
            }
            "unfinish" | "un" => {
                let id: usize = args[2].trim().parse().expect("Invalid id entered.");
                Cmd::Unfinish(id)
            }
            "remove" | "r" => {
                let id: usize = args[2].trim().parse().expect("Invalid id entered.");
                Cmd::Remove(id)
            }
            _ => Cmd::Invalid,
        };
    }
    match &command {
        Cmd::Help => {
            let usage = r#"USAGE:
            todoit COMMAND [OPTIONS]
            "#;
            let commands = r#"COMMANDS:
            help,       h                       Show this help
            add,        a <TODO>                Add a todo to your list
            list,       ls                      List all of your todos
            finish,     x <TODO ID>             Mark a todo completed
            unfinish,   un <TODO ID>            Mark a todo incomplete
            remove,     r <TODO ID>             Remove a todo from your list
            "#;
            println!("{}", usage);
            println!("{}", commands);
        }
        Cmd::Add(t) => {
            let new_todo = Todo {
                task: t.to_string(),
                completed: false,
            };
            my_todos.push(new_todo);
        }
        Cmd::List => {
            for (i, todo) in my_todos.iter().enumerate() {
                println!("{}: {} - {}", i, todo.task, todo.completed);
            }
        }
        Cmd::Finish(id) => {
            for (i, todo) in my_todos.iter_mut().enumerate() {
                if i == *id {
                    todo.completed = true
                }
            }
        }
        Cmd::Unfinish(id) => {
            for (i, todo) in my_todos.iter_mut().enumerate() {
                if i == *id {
                    todo.completed = false
                }
            }
        }
        Cmd::Remove(id) => {
            my_todos.remove(*id);
        }
        Cmd::Invalid => {
            println!("Invalid command issued. Try passing h or help to show help.");
        }
    };

    match command {
        Cmd::Add(_) | Cmd::Finish(_) | Cmd::Unfinish(_) | Cmd::Remove(_) => {
            save::save(my_todos, &storage_file).expect("Failed to save todos to file");
        }
        _ => {}
    }
}

fn get_storage_file() -> Result<String, VarError> {
    env::var(STORAGE)
}
