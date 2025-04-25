mod controllers;
mod models;

use crate::controllers::routes::routes::all_services;
use crate::models::db::init::init_db_pool;
use actix_web::{App, HttpServer, web};
pub use tera::Tera;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let tera = Tera::new("src/views/**/*").unwrap();

    let port = 8080;
    let pool = init_db_pool().await?;
    println!("Started the server at port: {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(pool.clone()))
            .configure(all_services)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await?;

    Ok(())
}
