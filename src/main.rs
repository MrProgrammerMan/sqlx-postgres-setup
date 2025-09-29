use sqlx::{postgres::PgPoolOptions};
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Read environment variables
    let db_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let db_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let db_name = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let db_host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
    let db_port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set");

    // Connect to default postgres database as admin
    let admin_url = format!(
        "postgres://{}:{}@{}:{}/postgres",
        db_user, db_password, db_host, db_port
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&admin_url)
        .await?;

    // Check if user exists
    let user_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM pg_roles WHERE rolname = $1)"
    )
    .bind(&db_user)
    .fetch_one(&pool)
    .await?;

    if !user_exists {
        println!("Creating user '{}'", db_user);
        sqlx::query(&format!("CREATE USER \"{}\" WITH PASSWORD $1", db_user))
            .bind(&db_password)
            .execute(&pool)
            .await?;
    } else {
        println!("User '{}' already exists", db_user);
    }

    // Check if database exists
    let db_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)"
    )
    .bind(&db_name)
    .fetch_one(&pool)
    .await?;

    if !db_exists {
        println!("Creating database '{}'", db_name);
        sqlx::query(&format!("CREATE DATABASE {} OWNER {}", db_name, db_user))
            .execute(&pool)
            .await?;
    } else {
        println!("Database '{}' already exists", db_name);
    }

    println!("Setup complete!");
    Ok(())
}