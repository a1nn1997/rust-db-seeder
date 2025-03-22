pub use self::operations::*;

mod helpers;
mod operations;

use sqlx::{Error as SqlxError, Pool, Postgres};
use std::fs;
use std::path::Path;

/// Setup database connection pool
pub async fn setup_database(database_url: &str) -> Result<Pool<Postgres>, SqlxError> {
    println!("Connecting to database...");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Set search_path to include the global schema
    sqlx::query("SET search_path TO global, public")
        .execute(&pool)
        .await?;

    println!("Connected to database successfully!");
    Ok(pool)
}

/// Initialize database schema by creating tables if they don't exist
pub async fn initialize_database(pool: &Pool<Postgres>) -> Result<(), SqlxError> {
    println!("Initializing database schema...");

    // Path to the schema file
    let schema_path = Path::new("src/db/schema.sql");

    // Read the SQL schema file
    let schema = fs::read_to_string(schema_path).expect("Failed to read schema.sql file");

    // Execute the SQL script to create tables
    sqlx::query(&schema).execute(pool).await?;

    println!("Database schema initialized successfully!");
    Ok(())
}
