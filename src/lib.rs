use std::io::Error;

use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;

mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod utils;

pub async fn run() -> Result<(), Error> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "info,actix_web=info,actix_server=info");
    env_logger::init();

    let address = ("127.0.0.1", 8080);

    let pool = db::get_pool().await;

    log::info!("Starting server...");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .configure(routes::configure)
    })
    .bind(address)?
    .run()
    .await
}
