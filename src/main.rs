use std::error::Error;

mod db;
mod panchang;
mod routes;
mod server;


#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {    
    server::http_server().await?;

    Ok(())
}
