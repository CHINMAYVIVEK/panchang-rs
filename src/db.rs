use sqlx::{PgPool, postgres::{PgPoolOptions, PgConnectOptions}};
use std::env;

pub async fn init_db() -> PgPool {
    let host = env::var("DB_HOST").expect("DB_HOST must be set");
    let port = env::var("DB_PORT")
        .expect("DB_PORT must be set")
        .parse::<u16>()
        .expect("DB_PORT must be a valid number");
    let user = env::var("DB_USER").expect("DB_USER must be set");
    let password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let dbname = env::var("DB_NAME").expect("DB_NAME must be set");

    println!("Attempting to connect to PostgreSQL database at {}:{}", host, port);

    match PgPoolOptions::new()
        .max_connections(5)
        .connect_with(
            PgConnectOptions::new()
                .host(&host)
                .port(port)
                .username(&user)
                .password(&password)
                .database(&dbname)
        )
        .await {
            Ok(pool) => {
                println!("✅ Successfully connected to the database");
                pool
            }
            Err(err) => {
                eprintln!("❌ Failed to connect to the database: {}", err);
                panic!("Database connection error: {}", err);
            }
        }
}