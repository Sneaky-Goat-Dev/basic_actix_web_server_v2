use actix_web::web;

pub mod health;
pub mod items;

pub fn configure(cfg: &mut web::ServiceConfig) {
    health::configure_routes(cfg);
    items::configure_routes(cfg);
}
