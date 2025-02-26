use std::io::Error;

use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;

mod handlers;
mod routes;

pub async fn run() -> Result<(), Error> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "info,actix_web=info,actix_server=info");
    env_logger::init();

    let address = ("127.0.0.1", 8080);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::configure)
    })
    .bind(address)?
    .run()
    .await
}
