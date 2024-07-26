/*
Todos Structures for A todo App from command line
 */

use rusqlite::{params, Connection, Result};

pub struct Todo {
    pub id: i32,
    pub description: String,
    pub status: bool,
}

impl Todo {
    pub fn new(id: i32, description: String, status: bool) -> Todo {
        Todo {
            id,
            description,
            status,
        }
    }

    pub fn create_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                status BOOLEAN NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add(conn: &Connection, description: &str) -> Result<()> {
        conn.execute(
            "INSERT INTO todos (description, status) VALUES (?1, ?2)",
            params![description, false],
        )?;
        Ok(())
    }

    pub fn list(conn: &Connection) -> Result<Vec<Todo>> {
        let mut stmt = conn.prepare("SELECT id, description, status FROM todos")?;
        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo::new(row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo?);
        }
        Ok(todos)
    }

    pub fn update(conn: &Connection, id: i32, status: bool) -> Result<()> {
        conn.execute("UPDATE todos SET status = ?1 WHERE id = ?2", params![status, id])?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i32) -> Result<()> {
        conn.execute("DELETE FROM todos WHERE id = ?1", params![id])?;
        Ok(())
    }
}

pub fn connect() -> Result<Connection> {
    let conn = Connection::open("todos.db")?;
    Todo::create_table(&conn)?;
    Ok(conn)
}
