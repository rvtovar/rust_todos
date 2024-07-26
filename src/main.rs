// use clap::{App, Arg, SubCommand};
use todos::{connect, Todo};

fn main() {
    // let matches = App::new("Todo CLI")
    //     .version("1.0")
    //     .author("Author Name <email@example.com>")
    //     .about("Manages your todos")
    //     .subcommand(
    //         SubCommand::with_name("add")
    //             .about("Adds a new todo")
    //             .arg(Arg::with_name("DESCRIPTION").required(true)),
    //     )
    //     .subcommand(SubCommand::with_name("list").about("Lists all todos"))
    //     .subcommand(
    //         SubCommand::with_name("delete")
    //             .about("Deletes a todo")
    //             .arg(Arg::with_name("ID").required(true)),
    //     )
    //     .get_matches();
    //
    // let conn = connect().expect("Failed to establish database connection");
    //
    // if let Some(matches) = matches.subcommand_matches("add") {
    //     if let Some(description) = matches.value_of("DESCRIPTION") {
    //         Todo::add(&conn, description).expect("Failed to add todo");
    //     }
    // } else if let Some(_) = matches.subcommand_matches("list") {
    //     let todos = Todo::list(&conn).expect("Failed to list todos");
    //     for todo in todos {
    //         println!("{}: {}", todo.id, todo.description);
    //     }
    // } else if let Some(matches) = matches.subcommand_matches("delete") {
    //     if let Some(id) = matches.value_of("ID") {
    //         let id: i32 = id.parse().expect("ID must be a number");
    //         Todo::delete(&conn, id).expect("Failed to delete todo");
    //     }
    // }
}
