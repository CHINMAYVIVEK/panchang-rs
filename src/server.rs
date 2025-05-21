use actix_web::{web,App, HttpServer};
use crate::routes;
use std::error::Error;
use dotenv::dotenv;
use std::env;

use crate::db;

/// Starts the HTTP server on the given port.
pub async fn http_server() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| String::from("8080"))
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // db conection
    let pool = db::init_db().await;

    println!("🚀 Starting server at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
            .configure(routes::init)
    })
    .bind((host, port))?
    .run()
    .await?;

    Ok(())
}
