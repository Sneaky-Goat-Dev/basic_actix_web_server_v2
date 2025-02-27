use actix_web::web;

pub mod auth;
pub mod health;
pub mod items;

pub fn configure(cfg: &mut web::ServiceConfig) {
    health::configure_routes(cfg);
    items::configure_routes(cfg);
    auth::configure_routes(cfg);
}
