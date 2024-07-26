use clap::Parser;
use colored::*;
use todos::{connect, Todo};

#[derive(Parser)]
#[command(name = "Todo", version = "1.0", about = "Manages your todos")]
struct Cli {
    #[arg(short, long, help = "Add a new todo item")]
    add: Option<String>,
    #[arg(short, long, help = "List all todo items")]
    list: bool,
    #[arg(short, long, help = "Mark a todo item as complete")]
    complete: Option<i32>,
    #[arg(short, long, help = "Reopen a completed todo item")]
    reopen: Option<i32>,
    #[arg(short, long, help = "Delete a todo item")]
    delete: Option<i32>,
}

fn main() {
    let cli = Cli::parse();
    let conn = connect().expect("Failed to establish database connection");

    if let Some(description) = cli.add {
        let todo = Todo::add(&conn, &description).expect("Failed to add todo");
        println!("{}", format!("Added: {}: {}", todo.id, todo.description).blue());
    } else if cli.list {
        let todos = Todo::list(&conn).expect("Failed to list todos");
        for todo in todos {
            let msg = if todo.status {
                format!("{}: {}", todo.id, todo.description.chars().map(|c| format!("\u{0336}{}", c)).collect::<String>()).red()
            } else {
                format!("{}: {}", todo.id, todo.description).green()
            };
            println!("{}", msg);
        }
    } else if let Some(id) = cli.complete {
        let todo = Todo::update(&conn, id, true).expect("Failed to update todo");
        println!("{}", format!("Completed: {}: {}", todo.id, todo.description).purple());
    } else if let Some(id) = cli.reopen {
        let todo = Todo::update(&conn, id, false).expect("Failed to update todo");
        println!("{}", format!("Reopened: {}: {}", todo.id, todo.description).blue());
    } else if let Some(id) = cli.delete {
        Todo::delete(&conn, id).expect("Failed to delete todo");
        println!("{}", format!("Deleted: {}", id).red());
    }
}