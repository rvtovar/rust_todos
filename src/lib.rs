use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Result};
use sqlx::query;
use dotenv::dotenv;
use std::env;

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

    pub async fn create_table(pool: &Pool<Postgres>) -> Result<()> {
        query!(
        "CREATE TABLE IF NOT EXISTS todos (
            id SERIAL PRIMARY KEY,
            description TEXT NOT NULL,
            status BOOLEAN NOT NULL
        )"
    )
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn add(pool: &Pool<Postgres>, description: &str) -> Result<Todo> {
        let rec = query!(
            "INSERT INTO todos (description, status) VALUES ($1, $2) RETURNING id, description, status",
            description,
            false
        )
            .fetch_one(pool)
            .await?;
        Ok(Todo::new(rec.id, rec.description, rec.status))
    }

    pub async fn list(pool: &Pool<Postgres>) -> Result<Vec<Todo>> {
        let recs = query!(
            "SELECT id, description, status FROM todos"
        )
            .fetch_all(pool)
            .await?;
        Ok(recs.into_iter().map(|rec| Todo::new(rec.id, rec.description, rec.status)).collect())
    }

    pub async fn update(pool: &Pool<Postgres>, id: i32, status: bool) -> Result<Todo> {
        let rec = query!(
            "UPDATE todos SET status = $1 WHERE id = $2 RETURNING id, description, status",
            status,
            id
        )
            .fetch_one(pool)
            .await?;
        Ok(Todo::new(rec.id, rec.description, rec.status))
    }

    pub async fn delete(pool: &Pool<Postgres>, id: i32) -> Result<()> {
        query!(
            "DELETE FROM todos WHERE id = $1",
            id
        )
            .execute(pool)
            .await?;
        Ok(())
    }
}

pub async fn connect() -> Result<Pool<Postgres>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Ensure the table is created before any other operations
    Todo::create_table(&pool).await?;

    Ok(pool)
}