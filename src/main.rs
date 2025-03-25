mod controllers;

use crate::controllers::routes::routes::configure_service;
use actix_web::{App, HttpServer, web};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("src/views/*").unwrap();
    println!("Starting the server at port: 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .configure(configure_service)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
